//! Coverage aggregation executable

use anyhow::Result;
use mega_tests::*;
use coverage::CoverageAggregator;
use std::path::PathBuf;
use test_database::TestDatabase;

fn main() -> Result<()> {
    println!("=== COVERAGE AGGREGATOR ===\n");

    let db_path = PathBuf::from("mega_tests_results.db");
    let db = TestDatabase::new(&db_path)?;

    let metrics = db.get_metrics()?;

    println!("Total Coverage Statistics:");
    if let Some(coverage) = &metrics.coverage {
        println!("  Line Coverage: {:.2}%", coverage.line_coverage_percent());
        println!(
            "  Branch Coverage: {:.2}%",
            coverage.branch_coverage_percent()
        );
        println!(
            "  Function Coverage: {:.2}%",
            coverage.function_coverage_percent()
        );
        println!();
        println!("  Lines: {}/{}", coverage.lines_covered, coverage.lines_total);
        println!(
            "  Branches: {}/{}",
            coverage.branches_covered, coverage.branches_total
        );
        println!(
            "  Functions: {}/{}",
            coverage.functions_covered, coverage.functions_total
        );
    } else {
        println!("  No coverage data available");
    }

    println!("\n=== Coverage by Module ===");
    println!("(To be implemented with tarpaulin integration)");

    Ok(())
}
