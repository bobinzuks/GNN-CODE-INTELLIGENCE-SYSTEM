//! Parallel test runner for executing thousands of tests concurrently

use crate::{TestResult, TestStatus, TestType, TestSuiteConfig};
use anyhow::Result;
use rayon::prelude::*;
use std::sync::Arc;
use std::time::{Duration, Instant};
use uuid::Uuid;

pub type TestFunction = Arc<dyn Fn() -> Result<()> + Send + Sync>;

pub struct TestCase {
    pub name: String,
    pub test_type: TestType,
    pub function: TestFunction,
}

impl TestCase {
    pub fn new(name: String, test_type: TestType, function: TestFunction) -> Self {
        Self {
            name,
            test_type,
            function,
        }
    }
}

pub struct ParallelTestRunner {
    config: TestSuiteConfig,
    tests: Vec<TestCase>,
}

impl ParallelTestRunner {
    pub fn new(config: TestSuiteConfig) -> Self {
        Self {
            config,
            tests: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test: TestCase) {
        self.tests.push(test);
    }

    pub fn add_tests(&mut self, tests: Vec<TestCase>) {
        self.tests.extend(tests);
    }

    pub fn run_all(&self) -> Vec<TestResult> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.max_parallel_tests)
            .build()
            .unwrap();

        pool.install(|| {
            self.tests
                .par_iter()
                .map(|test| self.run_test(test))
                .collect()
        })
    }

    fn run_test(&self, test: &TestCase) -> TestResult {
        let start = Instant::now();
        let id = Uuid::new_v4();

        let status = match (test.function)() {
            Ok(_) => TestStatus::Passed,
            Err(e) => {
                return TestResult {
                    id,
                    test_name: test.name.clone(),
                    test_type: test.test_type,
                    status: TestStatus::Failed,
                    duration: start.elapsed(),
                    error_message: Some(e.to_string()),
                    coverage_data: None,
                };
            }
        };

        TestResult {
            id,
            test_name: test.name.clone(),
            test_type: test.test_type,
            status,
            duration: start.elapsed(),
            error_message: None,
            coverage_data: None,
        }
    }

    pub fn test_count(&self) -> usize {
        self.tests.len()
    }
}
