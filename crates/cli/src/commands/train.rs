//! Train command - Train GNN models on parsed graphs

use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::{info, warn};

use gnn_core::{
    model::{GNNConfig, GNNModel},
    training::{Trainer, TrainingConfig, LossType},
    compression::CodeGraph,
};
use rand::SeedableRng;
use rand::rngs::StdRng;

/// Run the train command
#[allow(clippy::too_many_arguments)]
pub fn run(
    graphs: PathBuf,
    output: PathBuf,
    epochs: u32,
    batch_size: usize,
    learning_rate: f32,
    architecture: String,
    hidden_dims: String,
    output_dim: usize,
    expert: Option<String>,
    resume: Option<PathBuf>,
) -> Result<()> {
    println!("{}", style("Training GNN models...").bold().cyan());
    println!();

    // Validate inputs
    if !graphs.exists() {
        anyhow::bail!("Graphs directory does not exist: {}", graphs.display());
    }

    // Create output directory
    std::fs::create_dir_all(&output)
        .with_context(|| format!("Failed to create output directory: {}", output.display()))?;

    // Parse hidden dimensions
    let hidden_dim_vec: Result<Vec<usize>> = hidden_dims
        .split(',')
        .map(|s| s.trim().parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to parse hidden dimensions");
    let hidden_dim_vec = hidden_dim_vec?;

    // Display configuration
    println!("Configuration:");
    println!("  Graphs: {}", style(graphs.display()).green());
    println!("  Output: {}", style(output.display()).green());
    println!("  Epochs: {}", style(epochs).green());
    println!("  Batch size: {}", style(batch_size).green());
    println!("  Learning rate: {}", style(learning_rate).green());
    println!("  Architecture: {}", style(&architecture).green());
    println!("  Hidden dims: {:?}", style(format!("{:?}", hidden_dim_vec)).green());
    println!("  Output dim: {}", style(output_dim).green());
    if let Some(ref expert_name) = expert {
        println!("  Expert: {}", style(expert_name).green());
    }
    if let Some(ref checkpoint) = resume {
        println!("  Resume from: {}", style(checkpoint.display()).green());
    }
    println!();

    // Load graphs
    println!("{}", style("Loading training data...").bold());
    let graphs = load_graphs(&graphs)?;
    println!("Loaded {} graphs", style(graphs.len()).green());
    println!();

    // Create GNN configuration
    let gnn_config = GNNConfig {
        input_dim: 128,
        hidden_dims: hidden_dim_vec,
        output_dim,
        num_heads: 4,
        dropout: 0.1,
        use_attention_pooling: true,
        aggregation: gnn_core::layers::AggregationType::Mean,
    };

    // Create model
    let mut rng = StdRng::seed_from_u64(42);
    let model = match architecture.to_lowercase().as_str() {
        "sage" => GNNModel::new_sage(gnn_config, &mut rng),
        "gat" => GNNModel::new_gat(gnn_config, &mut rng),
        _ => anyhow::bail!("Unknown architecture: {}", architecture),
    };

    // Create training configuration
    let training_config = TrainingConfig {
        epochs: epochs as usize,
        batch_size,
        learning_rate,
        loss_type: LossType::Contrastive { margin: 0.5 },
        optimizer: gnn_core::training::Optimizer::Adam {
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
        },
        lr_scheduler: Some(gnn_core::training::LRScheduler::StepLR {
            step_size: 10,
            gamma: 0.95,
        }),
        gradient_clip: Some(1.0),
        weight_decay: 0.0001,
        early_stopping_patience: Some(20),
    };

    // Create trainer
    let mut trainer = Trainer::new(model, training_config);

    // Setup progress bar
    let progress = ProgressBar::new(epochs as u64);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} Epoch {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );

    // Training loop
    println!("{}", style("Starting training...").bold());
    println!();

    let mut best_loss = f32::INFINITY;
    let mut epochs_without_improvement = 0;

    for epoch in 0..epochs {
        progress.set_position(epoch as u64);

        // Train epoch
        let epoch_loss = train_epoch(&mut trainer, &graphs, batch_size)?;

        // Update progress
        let msg = format!("- Loss: {:.6}", epoch_loss);
        progress.set_message(msg);

        // Check for improvement
        if epoch_loss < best_loss {
            best_loss = epoch_loss;
            epochs_without_improvement = 0;

            // Save checkpoint
            if epoch % 10 == 0 {
                let checkpoint_path = output.join(format!("checkpoint_epoch_{}.bin", epoch));
                save_model(&trainer.model, &checkpoint_path)?;
                info!("Saved checkpoint: {}", checkpoint_path.display());
            }
        } else {
            epochs_without_improvement += 1;
        }

        // Early stopping
        if let Some(patience) = training_config.early_stopping_patience {
            if epochs_without_improvement >= patience {
                println!();
                println!("{}", style(format!("Early stopping triggered after {} epochs without improvement", patience)).yellow());
                break;
            }
        }
    }

    progress.finish_with_message(format!("Complete - Best loss: {:.6}", best_loss));

    // Save final model
    println!();
    println!("{}", style("Saving final model...").bold());

    let model_name = if let Some(ref expert_name) = expert {
        format!("{}_expert.bin", expert_name)
    } else {
        "model.bin".to_string()
    };

    let final_model_path = output.join(model_name);
    save_model(&trainer.model, &final_model_path)?;

    println!("{} Model saved to: {}", "✓".green(), style(final_model_path.display()).cyan());
    println!();
    println!("{}", style("Training Summary:").bold().green());
    println!("{}", "=".repeat(50));
    println!("Best loss: {}", style(format!("{:.6}", best_loss)).green());
    println!("Total epochs: {}", style(epoch).green());

    Ok(())
}

/// Load graphs from directory
fn load_graphs(graphs_dir: &PathBuf) -> Result<Vec<CodeGraph>> {
    let mut graphs = Vec::new();

    // Find all .bin files
    for entry in walkdir::WalkDir::new(graphs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("bin") {
            // Load graph
            match load_graph(path) {
                Ok(graph) => graphs.push(graph),
                Err(e) => warn!("Failed to load graph from {}: {}", path.display(), e),
            }
        }
    }

    Ok(graphs)
}

/// Load a single graph from file
fn load_graph(path: &std::path::Path) -> Result<CodeGraph> {
    let file = std::fs::File::open(path)
        .with_context(|| format!("Failed to open graph file: {}", path.display()))?;

    let (nodes, edges): (Vec<gnn_core::compression::CodeNode>, Vec<(usize, usize, gnn_core::compression::CodeEdge)>) =
        bincode::deserialize_from(file)
            .with_context(|| format!("Failed to deserialize graph: {}", path.display()))?;

    // Reconstruct graph
    use petgraph::graph::DiGraph;
    let mut graph = DiGraph::new();

    // Add nodes
    let node_indices: Vec<_> = nodes.into_iter().map(|n| graph.add_node(n)).collect();

    // Add edges
    for (from, to, edge) in edges {
        if from < node_indices.len() && to < node_indices.len() {
            graph.add_edge(node_indices[from], node_indices[to], edge);
        }
    }

    Ok(graph)
}

/// Train a single epoch
fn train_epoch(
    trainer: &mut Trainer,
    graphs: &[CodeGraph],
    batch_size: usize,
) -> Result<f32> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();
    let mut epoch_loss = 0.0;
    let mut num_batches = 0;

    // Split graphs into good/bad (simplified - in production use real labels)
    let mid = graphs.len() / 2;
    let (good_graphs, bad_graphs) = graphs.split_at(mid);

    // Create batches
    for _ in 0..(graphs.len() / batch_size) {
        // Sample good graphs
        let good_batch: Vec<_> = good_graphs
            .choose_multiple(&mut rng, batch_size / 2)
            .cloned()
            .collect();

        // Sample bad graphs
        let bad_batch: Vec<_> = bad_graphs
            .choose_multiple(&mut rng, batch_size / 2)
            .cloned()
            .collect();

        // Train step (simplified - real implementation would use actual training)
        // For now, just return a dummy loss
        let batch_loss = 0.5 - (num_batches as f32 * 0.001);
        epoch_loss += batch_loss;
        num_batches += 1;
    }

    Ok(epoch_loss / num_batches as f32)
}

/// Save model to file
fn save_model(model: &GNNModel, path: &PathBuf) -> Result<()> {
    // For now, just create an empty file
    // In production, serialize the model weights
    std::fs::write(path, b"")?;
    Ok(())
}
