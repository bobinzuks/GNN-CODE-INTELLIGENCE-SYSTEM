//! LLM Bridge CLI - Command-line interface for testing LLM integration
//!
//! This CLI allows testing the LLM bridge functionality with various providers,
//! projection strategies, and injection methods.

use llm_bridge::*;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== LLM Bridge CLI ===\n");

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "help" | "-h" | "--help" => print_help(),
        "test-ollama" => test_ollama().await?,
        "test-openai" => test_openai().await?,
        "interactive" => interactive_mode().await?,
        "projection" => test_projection()?,
        "injection" => test_injection()?,
        "validation" => test_validation()?,
        "stream" => test_streaming().await?,
        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }

    Ok(())
}

fn print_help() {
    println!("Usage: llm-bridge-cli <command>");
    println!("\nCommands:");
    println!("  help              Show this help message");
    println!("  test-ollama       Test Ollama integration");
    println!("  test-openai       Test OpenAI integration");
    println!("  interactive       Interactive mode for testing");
    println!("  projection        Test projection layer");
    println!("  injection         Test token injection strategies");
    println!("  validation        Test code validation");
    println!("  stream            Test streaming generation");
    println!("\nEnvironment variables:");
    println!("  OLLAMA_HOST       Ollama API URL (default: http://localhost:11434)");
    println!("  OPENAI_API_KEY    OpenAI API key");
    println!("  OPENAI_BASE_URL   OpenAI API base URL (default: https://api.openai.com/v1)");
}

async fn test_ollama() -> Result<()> {
    println!("Testing Ollama integration...\n");

    let ollama_host = env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".to_string());
    let ollama = OllamaClient::new(&ollama_host);

    // Health check
    print!("Checking Ollama connection... ");
    io::stdout().flush().unwrap();

    match ollama.health_check().await {
        Ok(true) => println!("OK"),
        Ok(false) => {
            println!("FAILED");
            println!("Error: Could not connect to Ollama at {}", ollama_host);
            println!("Make sure Ollama is running: ollama serve");
            return Ok(());
        }
        Err(e) => {
            println!("ERROR: {}", e);
            return Ok(());
        }
    }

    // List models
    println!("\nAvailable models:");
    match ollama.list_models().await {
        Ok(models) => {
            if models.is_empty() {
                println!("  No models found. Pull a model with: ollama pull llama3.2");
            } else {
                for model in models {
                    println!("  - {} ({:.2} GB)", model.name, model.size as f64 / 1_000_000_000.0);
                }
            }
        }
        Err(e) => println!("  Error listing models: {}", e),
    }

    // Create bridge
    println!("\nCreating LLM Bridge...");
    let projection_config = ProjectionConfig::default();
    let mut bridge = LLMBridge::new(Box::new(ollama), projection_config);

    // Generate with GNN embedding
    println!("\nGenerating code with GNN context...");
    let gnn_embedding = vec![0.1; 512]; // Mock embedding
    let prompt = "Write a Rust function that calculates the factorial of a number";

    println!("Prompt: {}", prompt);
    println!("\nGenerating...");

    match bridge.generate(prompt, &gnn_embedding, InjectionStrategy::Prepend).await {
        Ok(response) => {
            println!("\n=== Generated Code ===");
            println!("{}", response.text);
            if let Some(stats) = response.stats {
                println!("\n=== Statistics ===");
                println!("Prompt tokens: {}", stats.prompt_tokens);
                println!("Completion tokens: {}", stats.completion_tokens);
                println!("Total tokens: {}", stats.total_tokens);
                println!("Generation time: {}ms", stats.generation_time_ms);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

async fn test_openai() -> Result<()> {
    println!("Testing OpenAI integration...\n");

    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("Error: OPENAI_API_KEY environment variable not set");
            println!("Set it with: export OPENAI_API_KEY=your-key");
            return Ok(());
        }
    };

    let base_url = env::var("OPENAI_BASE_URL")
        .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let openai = OpenAIClient::with_base_url(base_url, api_key);

    // Health check
    print!("Checking OpenAI connection... ");
    io::stdout().flush().unwrap();

    match openai.health_check().await {
        Ok(true) => println!("OK"),
        Ok(false) => {
            println!("FAILED");
            return Ok(());
        }
        Err(e) => {
            println!("ERROR: {}", e);
            return Ok(());
        }
    }

    // Create bridge
    println!("\nCreating LLM Bridge...");
    let projection_config = ProjectionConfig::default();
    let mut bridge = LLMBridge::new(Box::new(openai), projection_config);

    // Generate with GNN embedding
    println!("\nGenerating code with GNN context...");
    let gnn_embedding = vec![0.1; 512];
    let prompt = "Write a Python function that implements binary search";

    println!("Prompt: {}", prompt);
    println!("\nGenerating...");

    match bridge.generate_with_params(
        prompt,
        &gnn_embedding,
        InjectionStrategy::Structured {
            format: injection::StructuredFormat::Markdown,
        },
        GenerationParameters::default(),
        "gpt-4",
    ).await {
        Ok(response) => {
            println!("\n=== Generated Code ===");
            println!("{}", response.text);
            if let Some(stats) = response.stats {
                println!("\n=== Statistics ===");
                println!("Prompt tokens: {}", stats.prompt_tokens);
                println!("Completion tokens: {}", stats.completion_tokens);
                println!("Total tokens: {}", stats.total_tokens);
                println!("Generation time: {}ms", stats.generation_time_ms);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}

async fn interactive_mode() -> Result<()> {
    println!("Interactive Mode - Type 'exit' to quit\n");

    // Setup provider
    println!("Select provider:");
    println!("  1. Ollama (local)");
    println!("  2. OpenAI");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    let provider: Box<dyn LLMProvider> = match choice {
        "1" => {
            let host = env::var("OLLAMA_HOST")
                .unwrap_or_else(|_| "http://localhost:11434".to_string());
            Box::new(OllamaClient::new(host))
        }
        "2" => {
            let api_key = env::var("OPENAI_API_KEY")
                .expect("OPENAI_API_KEY not set");
            Box::new(OpenAIClient::new(api_key))
        }
        _ => {
            println!("Invalid choice");
            return Ok(());
        }
    };

    let projection_config = ProjectionConfig::default();
    let mut bridge = LLMBridge::new(provider, projection_config);

    loop {
        print!("\nEnter prompt (or 'exit'): ");
        io::stdout().flush().unwrap();

        let mut prompt = String::new();
        io::stdin().read_line(&mut prompt).unwrap();
        let prompt = prompt.trim();

        if prompt == "exit" {
            break;
        }

        if prompt.is_empty() {
            continue;
        }

        // Mock GNN embedding
        let embedding = vec![0.1; 512];

        println!("\nGenerating...");
        match bridge.generate(prompt, &embedding, InjectionStrategy::Prepend).await {
            Ok(response) => {
                println!("\n{}", response.text);
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("\nGoodbye!");
    Ok(())
}

fn test_projection() -> Result<()> {
    println!("Testing Projection Layer...\n");

    // Test different projection types
    let configs = vec![
        ("Linear", ProjectionConfig {
            input_dim: 512,
            output_dim: 768,
            projection_type: projection::ProjectionType::Linear,
            normalize: true,
            activation: projection::ActivationType::ReLU,
        }),
        ("Identity", ProjectionConfig {
            input_dim: 512,
            output_dim: 512,
            projection_type: projection::ProjectionType::Identity,
            normalize: false,
            activation: projection::ActivationType::None,
        }),
        ("Pooling", ProjectionConfig {
            input_dim: 512,
            output_dim: 256,
            projection_type: projection::ProjectionType::Pooling {
                pool_type: projection::PoolingType::Average,
            },
            normalize: true,
            activation: projection::ActivationType::None,
        }),
    ];

    let input = vec![0.5; 512];

    for (name, config) in configs {
        println!("Testing {} projection:", name);
        let layer = ProjectionLayer::new(config);

        match layer.project(&input) {
            Ok(output) => {
                println!("  Input dim: {}", input.len());
                println!("  Output dim: {}", output.len());
                println!("  Output mean: {:.4}", output.iter().sum::<f32>() / output.len() as f32);
                println!("  Output norm: {:.4}", output.iter().map(|x| x * x).sum::<f32>().sqrt());
                println!("  ✓ Success\n");
            }
            Err(e) => println!("  ✗ Error: {}\n", e),
        }
    }

    Ok(())
}

fn test_injection() -> Result<()> {
    println!("Testing Token Injection Strategies...\n");

    let injector = TokenInjector::new();
    let embedding = vec![0.5; 512];
    let prompt = "Write a function to reverse a string";

    let strategies = vec![
        ("Prepend", InjectionStrategy::Prepend),
        ("Append", InjectionStrategy::Append),
        ("JSON", InjectionStrategy::Structured {
            format: injection::StructuredFormat::JSON,
        }),
        ("Markdown", InjectionStrategy::Structured {
            format: injection::StructuredFormat::Markdown,
        }),
    ];

    for (name, strategy) in strategies {
        println!("Strategy: {}", name);
        match injector.inject(prompt, &embedding, strategy) {
            Ok(result) => {
                println!("Length: {} chars", result.len());
                println!("Preview: {}...\n", &result[..result.len().min(100)]);
            }
            Err(e) => println!("Error: {}\n", e),
        }
    }

    Ok(())
}

fn test_validation() -> Result<()> {
    println!("Testing Code Validation...\n");

    let processor = PostProcessor::new();
    let embedding = vec![0.5; 512];

    let test_cases = vec![
        ("Valid code", "fn main() { println!(\"Hello\"); }"),
        ("Missing brace", "fn main() { println!(\"Hello\");"),
        ("Empty code", ""),
        ("Unbalanced brackets", "fn test() { let x = [1, 2, 3; }"),
    ];

    for (name, code) in test_cases {
        println!("Test: {}", name);
        println!("Code: {}", if code.is_empty() { "(empty)" } else { code });

        match processor.validate(code, &embedding) {
            Ok(result) => {
                println!("  Valid: {}", result.is_valid);
                println!("  Issues: {}", result.issues.len());
                for issue in &result.issues {
                    println!("    - {}", issue);
                }
                println!("  Syntax errors: {}", result.syntax_errors.len());
                for error in &result.syntax_errors {
                    println!("    - Line {}: {}", error.line, error.message);
                }
                if let Some(similarity) = result.semantic_similarity {
                    println!("  Semantic similarity: {:.3}", similarity);
                }
                println!();
            }
            Err(e) => println!("  Error: {}\n", e),
        }
    }

    Ok(())
}

async fn test_streaming() -> Result<()> {
    use futures::StreamExt;

    println!("Testing Streaming Generation...\n");

    let ollama_host = env::var("OLLAMA_HOST")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());
    let ollama = OllamaClient::new(&ollama_host);

    // Health check
    if !ollama.health_check().await.unwrap_or(false) {
        println!("Error: Cannot connect to Ollama");
        return Ok(());
    }

    let projection_config = ProjectionConfig::default();
    let mut bridge = LLMBridge::new(Box::new(ollama), projection_config);

    let embedding = vec![0.1; 512];
    let prompt = "Write a short haiku about coding";

    println!("Prompt: {}", prompt);
    println!("\nStreaming response:");
    println!("---");

    match bridge.generate_stream(prompt, &embedding, InjectionStrategy::Prepend).await {
        Ok(mut stream) => {
            while let Some(result) = stream.next().await {
                match result {
                    Ok(chunk) => {
                        print!("{}", chunk.text);
                        io::stdout().flush().unwrap();
                        if chunk.done {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("\nError: {}", e);
                        break;
                    }
                }
            }
            println!("\n---");
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
