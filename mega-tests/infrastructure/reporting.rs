//! Test reporting and output generation

use crate::{TestMetrics, TestResult, TestType};
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct TestReporter {
    results: Vec<TestResult>,
}

impl TestReporter {
    pub fn new(results: Vec<TestResult>) -> Self {
        Self { results }
    }

    pub fn generate_html_report(&self, output_path: &Path) -> Result<()> {
        let mut file = File::create(output_path)?;

        writeln!(file, "<!DOCTYPE html>")?;
        writeln!(file, "<html><head><title>Test Report</title>")?;
        writeln!(file, "<style>")?;
        writeln!(file, "body {{ font-family: Arial, sans-serif; margin: 20px; }}")?;
        writeln!(file, ".passed {{ color: green; }}")?;
        writeln!(file, ".failed {{ color: red; }}")?;
        writeln!(file, "table {{ border-collapse: collapse; width: 100%; }}")?;
        writeln!(file, "th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}")?;
        writeln!(file, "th {{ background-color: #4CAF50; color: white; }}")?;
        writeln!(file, "</style></head><body>")?;

        writeln!(file, "<h1>Mega Test Suite Report</h1>")?;

        // Summary
        let mut metrics = TestMetrics::new();
        for result in &self.results {
            metrics.add_result(result);
        }

        writeln!(file, "<h2>Summary</h2>")?;
        writeln!(file, "<p>Total Tests: {}</p>", metrics.total_tests)?;
        writeln!(file, "<p class='passed'>Passed: {} ({:.2}%)</p>", metrics.passed, metrics.success_rate())?;
        writeln!(file, "<p class='failed'>Failed: {}</p>", metrics.failed)?;
        writeln!(file, "<p>Duration: {:?}</p>", metrics.total_duration)?;

        // Test table
        writeln!(file, "<h2>Test Results</h2>")?;
        writeln!(file, "<table>")?;
        writeln!(file, "<tr><th>Test Name</th><th>Type</th><th>Status</th><th>Duration</th></tr>")?;

        for result in &self.results {
            let status_class = if result.status == crate::TestStatus::Passed {
                "passed"
            } else {
                "failed"
            };

            writeln!(
                file,
                "<tr><td>{}</td><td>{:?}</td><td class='{}'>{:?}</td><td>{:?}</td></tr>",
                result.test_name, result.test_type, status_class, result.status, result.duration
            )?;
        }

        writeln!(file, "</table>")?;
        writeln!(file, "</body></html>")?;

        Ok(())
    }

    pub fn generate_json_report(&self, output_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.results)?;
        std::fs::write(output_path, json)?;
        Ok(())
    }

    pub fn print_summary(&self) {
        let mut metrics = TestMetrics::new();
        for result in &self.results {
            metrics.add_result(result);
        }
        metrics.print_summary();
    }
}
