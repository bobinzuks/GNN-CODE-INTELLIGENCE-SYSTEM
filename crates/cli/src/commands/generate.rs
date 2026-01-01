//! Generate command - Generate code with LLM + GNN guidance

use anyhow::{Context, Result};
use colored::Colorize;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tracing::info;

/// Run the generate command
#[allow(clippy::too_many_arguments)]
pub async fn run(
    prompt: String,
    context: Option<PathBuf>,
    model: String,
    endpoint: String,
    output: Option<PathBuf>,
    language: Option<String>,
    temperature: f32,
    gnn_fix: bool,
) -> Result<()> {
    println!("{}", style("Generating code with LLM + GNN...").bold().cyan());
    println!();

    // Display configuration
    println!("Configuration:");
    println!("  Model: {}", style(&model).green());
    println!("  Endpoint: {}", style(&endpoint).green());
    println!("  Temperature: {}", style(temperature).green());
    if let Some(ref ctx) = context {
        println!("  Context: {}", style(ctx.display()).green());
    }
    if let Some(ref lang) = language {
        println!("  Language: {}", style(lang).green());
    }
    println!("  GNN post-processing: {}", style(gnn_fix).green());
    println!();

    // Load context embedding if provided
    let context_embedding = if let Some(ref ctx_path) = context {
        println!("{}", style("Loading context...").bold());
        Some(load_context_embedding(ctx_path)?)
    } else {
        None
    };

    // Build prompt
    let full_prompt = build_prompt(&prompt, &context_embedding, language.as_deref())?;

    // Call LLM
    println!("{}", style("Generating with LLM...").bold());
    println!();

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("Invalid spinner template"),
    );
    spinner.set_message("Waiting for LLM response...");

    let generated_code = call_llm(&endpoint, &model, &full_prompt, temperature).await?;

    spinner.finish_and_clear();

    // Apply GNN post-processing if requested
    let final_code = if gnn_fix {
        println!("{}", style("Applying GNN post-processing...").bold());
        apply_gnn_fix(&generated_code, language.as_deref())?
    } else {
        generated_code
    };

    // Output result
    if let Some(output_path) = output {
        println!("{}", style("Saving to file...").bold());
        std::fs::write(&output_path, &final_code)
            .with_context(|| format!("Failed to write output: {}", output_path.display()))?;
        println!("{} Code saved to: {}", "✓".green(), style(output_path.display()).cyan());
    } else {
        println!();
        println!("{}", style("Generated Code:").bold().green());
        println!("{}", "=".repeat(80));
        println!("{}", final_code);
        println!("{}", "=".repeat(80));
    }

    Ok(())
}

/// Load context embedding from file
fn load_context_embedding(path: &PathBuf) -> Result<Vec<f32>> {
    let data = std::fs::read(path)
        .with_context(|| format!("Failed to read context file: {}", path.display()))?;

    // Try bincode first
    if let Ok(embedding) = bincode::deserialize::<gnn_core::compression::ProjectEmbedding>(&data) {
        return Ok(embedding.vector);
    }

    // Try raw vector
    if let Ok(vector) = bincode::deserialize::<Vec<f32>>(&data) {
        return Ok(vector);
    }

    // Try JSON
    let json_str = String::from_utf8(data)
        .context("Failed to parse context file as UTF-8")?;

    if let Ok(embedding) = serde_json::from_str::<gnn_core::compression::ProjectEmbedding>(&json_str) {
        return Ok(embedding.vector);
    }

    if let Ok(vector) = serde_json::from_str::<Vec<f32>>(&json_str) {
        return Ok(vector);
    }

    anyhow::bail!("Failed to parse context embedding from file");
}

/// Build LLM prompt with context
fn build_prompt(
    user_prompt: &str,
    context: &Option<Vec<f32>>,
    language: Option<&str>,
) -> Result<String> {
    let mut full_prompt = String::new();

    // Add system message
    full_prompt.push_str("You are an expert code generator. Generate clean, idiomatic, production-quality code.\n\n");

    // Add language context if provided
    if let Some(lang) = language {
        full_prompt.push_str(&format!("Language: {}\n\n", lang));
    }

    // Add codebase context if provided
    if let Some(ref embedding) = context {
        full_prompt.push_str("Codebase context (semantic embedding):\n");
        full_prompt.push_str(&format!("<context_embedding dims={}>\n", embedding.len()));
        // In production, you would project this to token space
        // For now, just indicate it's present
        full_prompt.push_str("  [Codebase patterns and conventions encoded]\n");
        full_prompt.push_str("</context_embedding>\n\n");
    }

    // Add user prompt
    full_prompt.push_str("Task:\n");
    full_prompt.push_str(user_prompt);
    full_prompt.push_str("\n\nGenerate the code:");

    Ok(full_prompt)
}

/// Call LLM endpoint
async fn call_llm(
    endpoint: &str,
    model: &str,
    prompt: &str,
    temperature: f32,
) -> Result<String> {
    // Check if endpoint is Ollama
    if endpoint.contains("11434") || endpoint.contains("ollama") {
        call_ollama(endpoint, model, prompt, temperature).await
    } else {
        // Assume OpenAI-compatible API
        call_openai_compatible(endpoint, model, prompt, temperature).await
    }
}

/// Call Ollama API
async fn call_ollama(
    endpoint: &str,
    model: &str,
    prompt: &str,
    temperature: f32,
) -> Result<String> {
    let client = reqwest::Client::new();

    let request_body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        "options": {
            "temperature": temperature,
        }
    });

    let response = client
        .post(format!("{}/api/generate", endpoint))
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request to Ollama")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("LLM request failed with status {}: {}", status, body);
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .context("Failed to parse LLM response")?;

    let generated_text = response_json["response"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No response field in LLM output"))?
        .to_string();

    Ok(generated_text)
}

/// Call OpenAI-compatible API
async fn call_openai_compatible(
    endpoint: &str,
    model: &str,
    prompt: &str,
    temperature: f32,
) -> Result<String> {
    let client = reqwest::Client::new();

    let request_body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "temperature": temperature,
    });

    let response = client
        .post(format!("{}/v1/chat/completions", endpoint))
        .json(&request_body)
        .send()
        .await
        .context("Failed to send request to LLM")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("LLM request failed with status {}: {}", status, body);
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .context("Failed to parse LLM response")?;

    let generated_text = response_json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("No content in LLM response"))?
        .to_string();

    Ok(generated_text)
}

/// Apply GNN-based post-processing to fix issues
fn apply_gnn_fix(code: &str, language: Option<&str>) -> Result<String> {
    // In production, this would:
    // 1. Parse the generated code
    // 2. Run it through expert analysis
    // 3. Apply suggested fixes
    // 4. Return corrected code

    // For now, just do basic cleanup
    let mut fixed = code.to_string();

    // Remove markdown code blocks if present
    if fixed.contains("```") {
        fixed = fixed
            .lines()
            .filter(|line| !line.starts_with("```"))
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Trim whitespace
    fixed = fixed.trim().to_string();

    Ok(fixed)
}
