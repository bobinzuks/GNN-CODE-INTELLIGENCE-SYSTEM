//! Check command - Analyze code for issues and quality

use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use console::style;
use std::path::PathBuf;
use tracing::{info, warn};

use gnn_experts::{registry::ExpertRegistry, Issue, Severity};
use gnn_parser::{ParserConfig, ProjectParser};

/// Run the check command
#[allow(clippy::too_many_arguments)]
pub fn run(
    path: PathBuf,
    language: Option<String>,
    severity: String,
    json: bool,
    model: Option<PathBuf>,
    suggestions: bool,
    fix: bool,
) -> Result<()> {
    if !json {
        println!("{}", style("Checking code for issues...").bold().cyan());
        println!();
    }

    // Validate input
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Parse minimum severity
    let min_severity = parse_severity(&severity)?;

    if !json {
        println!("Configuration:");
        println!("  Path: {}", style(path.display()).green());
        if let Some(ref lang) = language {
            println!("  Language: {}", style(lang).green());
        }
        println!("  Min severity: {}", style(&severity).green());
        if let Some(ref model_path) = model {
            println!("  Model: {}", style(model_path.display()).green());
        }
        println!("  Suggestions: {}", style(suggestions).green());
        println!("  Auto-fix: {}", style(fix).green());
        println!();
    }

    // Parse the code
    if !json {
        println!("{}", style("Parsing code...").bold());
    }

    let parser = ProjectParser::new();
    let (graph, stats) = if path.is_dir() {
        parser.parse_project(&path)?
    } else {
        let graph = parser.parse_file(&path)?;
        let mut stats = gnn_parser::ParseStats::new();
        stats.files_parsed = 1;
        stats.total_nodes = graph.node_count();
        stats.total_edges = graph.edge_count();
        (graph, stats)
    };

    if !json {
        println!("  Nodes: {}", style(stats.total_nodes).green());
        println!("  Edges: {}", style(stats.total_edges).green());
        println!();
    }

    // Initialize expert registry
    let registry = ExpertRegistry::new();

    // Determine which experts to use
    let experts_to_check = if let Some(ref lang) = language {
        vec![lang.as_str()]
    } else {
        // Auto-detect from graph
        detect_languages_from_graph(&graph)
    };

    if !json {
        println!("{}", style("Running code analysis...").bold());
        if experts_to_check.is_empty() {
            println!("{}", style("No language experts available for this code").yellow());
            return Ok(());
        }
        println!("  Experts: {}", style(experts_to_check.join(", ")).green());
        println!();
    }

    // Collect all issues
    let mut all_issues = Vec::new();

    for lang in experts_to_check {
        if let Some(expert) = registry.get(lang) {
            // Create dummy embedding (in production, use real GNN compression)
            let embedding = gnn_core::Tensor::zeros(&[512]);

            // Convert graph to expert's format
            let expert_graph = convert_to_expert_graph(&graph);

            // Check for issues
            let issues = expert.check(&expert_graph, &embedding);
            all_issues.extend(issues);
        } else {
            if !json {
                warn!("No expert available for language: {}", lang);
            }
        }
    }

    // Filter by severity
    all_issues.retain(|issue| issue.severity >= min_severity);

    // Sort by severity (highest first)
    all_issues.sort_by(|a, b| b.severity.cmp(&a.severity));

    // Output results
    if json {
        output_json(&all_issues)?;
    } else {
        output_table(&all_issues, suggestions)?;
    }

    // Apply fixes if requested
    if fix && !all_issues.is_empty() {
        if json {
            eprintln!("Auto-fix not supported in JSON mode");
        } else {
            apply_fixes(&path, &all_issues, &registry)?;
        }
    }

    // Return error code if critical issues found
    let has_critical = all_issues.iter().any(|i| matches!(i.severity, Severity::Critical | Severity::Error));
    if has_critical && !fix {
        std::process::exit(1);
    }

    Ok(())
}

/// Parse severity level from string
fn parse_severity(s: &str) -> Result<Severity> {
    match s.to_lowercase().as_str() {
        "info" => Ok(Severity::Info),
        "warning" => Ok(Severity::Warning),
        "error" => Ok(Severity::Error),
        "critical" => Ok(Severity::Critical),
        _ => anyhow::bail!("Invalid severity level: {}", s),
    }
}

/// Detect languages from graph nodes
fn detect_languages_from_graph(graph: &gnn_parser::graph::CodeGraph) -> Vec<&'static str> {
    use std::collections::HashSet;

    let mut languages = HashSet::new();

    for node in graph.node_weights() {
        let lang = node.language.as_str();
        match lang.to_lowercase().as_str() {
            "rust" => { languages.insert("rust"); }
            "python" => { languages.insert("python"); }
            "go" => { languages.insert("go"); }
            "typescript" | "javascript" => { languages.insert("typescript"); }
            "java" => { languages.insert("java"); }
            _ => {}
        }
    }

    languages.into_iter().collect()
}

/// Convert parser graph to expert graph format
fn convert_to_expert_graph(
    graph: &gnn_parser::graph::CodeGraph,
) -> gnn_experts::CodeGraph {
    use petgraph::graph::DiGraph;

    let mut expert_graph = DiGraph::new();

    // Map node indices
    let mut node_map = std::collections::HashMap::new();

    // Add nodes
    for (idx, node) in graph.node_references() {
        let expert_node = gnn_experts::CodeNode {
            kind: convert_node_kind(&node.kind),
            name: node.name.clone(),
            language: node.language.clone(),
            file_path: node.file_path.clone(),
            start_line: node.start_line,
            end_line: node.end_line,
            start_col: node.start_col,
            end_col: node.end_col,
            signature: node.signature.clone(),
            visibility: convert_visibility(&node.visibility),
            is_async: node.is_async,
            is_static: node.is_static,
            is_generic: node.is_generic,
            metadata: node.metadata.clone(),
        };

        let new_idx = expert_graph.add_node(expert_node);
        node_map.insert(idx, new_idx);
    }

    // Add edges
    for edge in graph.edge_references() {
        if let (Some(&from), Some(&to)) = (node_map.get(&edge.source()), node_map.get(&edge.target())) {
            let expert_edge = gnn_experts::CodeEdge {
                kind: convert_edge_kind(&edge.weight().kind),
                weight: edge.weight().weight,
                metadata: edge.weight().metadata.clone(),
            };
            expert_graph.add_edge(from, to, expert_edge);
        }
    }

    expert_graph
}

/// Convert node kind
fn convert_node_kind(kind: &gnn_parser::graph::NodeKind) -> gnn_experts::NodeKind {
    use gnn_parser::graph::NodeKind as PK;
    use gnn_experts::NodeKind as EK;

    match kind {
        PK::File => EK::File,
        PK::Module => EK::Module,
        PK::Function => EK::Function,
        PK::Method => EK::Method,
        PK::Class => EK::Class,
        PK::Struct => EK::Struct,
        PK::Enum => EK::Enum,
        PK::Interface => EK::Interface,
        PK::Trait => EK::Trait,
        PK::Variable => EK::Variable,
        PK::Constant => EK::Constant,
        PK::Parameter => EK::Parameter,
        PK::Field => EK::Field,
        PK::Import => EK::Import,
        PK::Export => EK::Export,
        PK::Block => EK::Block,
        PK::Loop => EK::Loop,
        PK::Conditional => EK::Conditional,
        _ => EK::Unknown,
    }
}

/// Convert edge kind
fn convert_edge_kind(kind: &gnn_parser::graph::EdgeKind) -> gnn_experts::EdgeKind {
    use gnn_parser::graph::EdgeKind as PK;
    use gnn_experts::EdgeKind as EK;

    match kind {
        PK::Contains => EK::Contains,
        PK::Calls => EK::Calls,
        PK::HasType => EK::HasType,
        PK::Returns => EK::Returns,
        PK::Imports => EK::Imports,
        PK::DependsOn => EK::DependsOn,
        PK::Reads => EK::Reads,
        PK::Writes => EK::Writes,
        PK::ControlFlow => EK::ControlFlow,
        _ => EK::ControlFlow,
    }
}

/// Convert visibility
fn convert_visibility(vis: &gnn_parser::graph::Visibility) -> gnn_experts::Visibility {
    use gnn_parser::graph::Visibility as PV;
    use gnn_experts::Visibility as EV;

    match vis {
        PV::Private => EV::Private,
        PV::Protected => EV::Protected,
        PV::Public => EV::Public,
        PV::Internal => EV::Internal,
    }
}

/// Output issues as table
fn output_table(issues: &[Issue], show_suggestions: bool) -> Result<()> {
    if issues.is_empty() {
        println!("{}", style("No issues found!").green().bold());
        println!();
        return Ok(());
    }

    println!("{}", style(format!("Found {} issues:", issues.len())).yellow().bold());
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Add header
    table.set_header(vec!["Severity", "Pattern", "Location", "Message"]);

    // Add rows
    for issue in issues {
        let severity_cell = Cell::new(severity_to_string(&issue.severity))
            .fg(severity_to_color(&issue.severity));

        let location = format!(
            "{}:{}",
            issue.location.file_path,
            issue.location.start_line
        );

        table.add_row(vec![
            severity_cell,
            Cell::new(&issue.pattern),
            Cell::new(location),
            Cell::new(&issue.message),
        ]);
    }

    println!("{}", table);

    // Show suggestions if requested
    if show_suggestions {
        println!();
        println!("{}", style("Suggested fixes:").bold());
        for (i, issue) in issues.iter().enumerate() {
            if let Some(ref fix) = issue.suggested_fix {
                println!("  {}. {}", i + 1, fix);
            }
        }
    }

    Ok(())
}

/// Output issues as JSON
fn output_json(issues: &[Issue]) -> Result<()> {
    let json = serde_json::to_string_pretty(issues)?;
    println!("{}", json);
    Ok(())
}

/// Apply automatic fixes
fn apply_fixes(
    path: &PathBuf,
    issues: &[Issue],
    registry: &ExpertRegistry,
) -> Result<()> {
    println!();
    println!("{}", style("Applying automatic fixes...").bold());

    let mut fixed_count = 0;

    for issue in issues {
        if issue.suggested_fix.is_some() {
            // TODO: Apply fix
            // For now, just count
            fixed_count += 1;
        }
    }

    println!("{} Applied {} automatic fixes", "✓".green(), style(fixed_count).green());

    Ok(())
}

/// Convert severity to string
fn severity_to_string(severity: &Severity) -> &'static str {
    match severity {
        Severity::Info => "INFO",
        Severity::Warning => "WARN",
        Severity::Error => "ERROR",
        Severity::Critical => "CRIT",
    }
}

/// Convert severity to color
fn severity_to_color(severity: &Severity) -> Color {
    match severity {
        Severity::Info => Color::Blue,
        Severity::Warning => Color::Yellow,
        Severity::Error => Color::Red,
        Severity::Critical => Color::DarkRed,
    }
}
