//! Mega Test Infrastructure Library
//!
//! Provides the core infrastructure for running 1,000,000+ tests in parallel
//! with comprehensive coverage tracking, mutation testing, and performance monitoring.

pub mod test_database;
pub mod parallel_runner;
pub mod coverage;
pub mod mutation;
pub mod reporting;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

/// Test execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: Uuid,
    pub test_name: String,
    pub test_type: TestType,
    pub status: TestStatus,
    pub duration: Duration,
    pub error_message: Option<String>,
    pub coverage_data: Option<CoverageData>,
}

/// Type of test
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestType {
    Unit,
    Integration,
    EndToEnd,
    Fuzz,
    Performance,
    Security,
    Mutation,
    Property,
}

/// Test execution status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestStatus {
    Passed,
    Failed,
    Skipped,
    Timeout,
    Error,
}

/// Coverage data for a test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageData {
    pub lines_covered: usize,
    pub lines_total: usize,
    pub branches_covered: usize,
    pub branches_total: usize,
    pub functions_covered: usize,
    pub functions_total: usize,
}

impl CoverageData {
    pub fn line_coverage_percent(&self) -> f64 {
        if self.lines_total == 0 {
            0.0
        } else {
            (self.lines_covered as f64 / self.lines_total as f64) * 100.0
        }
    }

    pub fn branch_coverage_percent(&self) -> f64 {
        if self.branches_total == 0 {
            0.0
        } else {
            (self.branches_covered as f64 / self.branches_total as f64) * 100.0
        }
    }

    pub fn function_coverage_percent(&self) -> f64 {
        if self.functions_total == 0 {
            0.0
        } else {
            (self.functions_covered as f64 / self.functions_total as f64) * 100.0
        }
    }
}

/// Test suite configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuiteConfig {
    pub max_parallel_tests: usize,
    pub test_timeout: Duration,
    pub coverage_enabled: bool,
    pub mutation_testing_enabled: bool,
    pub fuzz_iterations: usize,
    pub performance_samples: usize,
}

impl Default for TestSuiteConfig {
    fn default() -> Self {
        Self {
            max_parallel_tests: num_cpus::get() * 4,
            test_timeout: Duration::from_secs(60),
            coverage_enabled: true,
            mutation_testing_enabled: false,
            fuzz_iterations: 10_000,
            performance_samples: 1_000,
        }
    }
}

/// Test metrics aggregation
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TestMetrics {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub timeout: usize,
    pub error: usize,
    pub total_duration: Duration,
    pub coverage: Option<CoverageData>,
}

impl TestMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_result(&mut self, result: &TestResult) {
        self.total_tests += 1;
        match result.status {
            TestStatus::Passed => self.passed += 1,
            TestStatus::Failed => self.failed += 1,
            TestStatus::Skipped => self.skipped += 1,
            TestStatus::Timeout => self.timeout += 1,
            TestStatus::Error => self.error += 1,
        }
        self.total_duration += result.duration;
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total_tests as f64) * 100.0
        }
    }

    pub fn print_summary(&self) {
        println!("\n=== TEST EXECUTION SUMMARY ===");
        println!("Total Tests: {}", self.total_tests);
        println!("Passed: {} ({:.2}%)", self.passed, self.success_rate());
        println!("Failed: {}", self.failed);
        println!("Skipped: {}", self.skipped);
        println!("Timeout: {}", self.timeout);
        println!("Error: {}", self.error);
        println!("Total Duration: {:?}", self.total_duration);

        if let Some(cov) = &self.coverage {
            println!("\n=== COVERAGE SUMMARY ===");
            println!("Line Coverage: {:.2}%", cov.line_coverage_percent());
            println!("Branch Coverage: {:.2}%", cov.branch_coverage_percent());
            println!("Function Coverage: {:.2}%", cov.function_coverage_percent());
        }
    }
}
