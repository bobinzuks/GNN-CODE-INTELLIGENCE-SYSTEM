//! Test result database for storing and querying test execution results

use crate::{TestResult, TestMetrics, TestType, TestStatus};
use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use std::path::Path;

pub struct TestDatabase {
    conn: Connection,
}

impl TestDatabase {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.create_tables()?;
        Ok(db)
    }

    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS test_results (
                id TEXT PRIMARY KEY,
                test_name TEXT NOT NULL,
                test_type TEXT NOT NULL,
                status TEXT NOT NULL,
                duration_ms INTEGER NOT NULL,
                error_message TEXT,
                timestamp INTEGER NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS coverage_data (
                test_id TEXT PRIMARY KEY,
                lines_covered INTEGER NOT NULL,
                lines_total INTEGER NOT NULL,
                branches_covered INTEGER NOT NULL,
                branches_total INTEGER NOT NULL,
                functions_covered INTEGER NOT NULL,
                functions_total INTEGER NOT NULL,
                FOREIGN KEY(test_id) REFERENCES test_results(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_test_type ON test_results(test_type)",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_test_status ON test_results(status)",
            [],
        )?;

        Ok(())
    }

    pub fn insert_result(&mut self, result: &TestResult) -> Result<()> {
        let tx = self.conn.transaction()?;

        tx.execute(
            "INSERT INTO test_results (id, test_name, test_type, status, duration_ms, error_message, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                result.id.to_string(),
                result.test_name,
                format!("{:?}", result.test_type),
                format!("{:?}", result.status),
                result.duration.as_millis() as i64,
                result.error_message,
                chrono::Utc::now().timestamp(),
            ],
        )?;

        if let Some(cov) = &result.coverage_data {
            tx.execute(
                "INSERT INTO coverage_data (test_id, lines_covered, lines_total, branches_covered, branches_total, functions_covered, functions_total)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    result.id.to_string(),
                    cov.lines_covered as i64,
                    cov.lines_total as i64,
                    cov.branches_covered as i64,
                    cov.branches_total as i64,
                    cov.functions_covered as i64,
                    cov.functions_total as i64,
                ],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn get_metrics(&self) -> Result<TestMetrics> {
        let mut stmt = self.conn.prepare(
            "SELECT
                COUNT(*) as total,
                SUM(CASE WHEN status = 'Passed' THEN 1 ELSE 0 END) as passed,
                SUM(CASE WHEN status = 'Failed' THEN 1 ELSE 0 END) as failed,
                SUM(CASE WHEN status = 'Skipped' THEN 1 ELSE 0 END) as skipped,
                SUM(CASE WHEN status = 'Timeout' THEN 1 ELSE 0 END) as timeout,
                SUM(CASE WHEN status = 'Error' THEN 1 ELSE 0 END) as error,
                SUM(duration_ms) as total_duration_ms
             FROM test_results"
        )?;

        let (total, passed, failed, skipped, timeout, error, total_duration_ms): (i64, i64, i64, i64, i64, i64, i64) =
            stmt.query_row([], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            })?;

        Ok(TestMetrics {
            total_tests: total as usize,
            passed: passed as usize,
            failed: failed as usize,
            skipped: skipped as usize,
            timeout: timeout as usize,
            error: error as usize,
            total_duration: std::time::Duration::from_millis(total_duration_ms as u64),
            coverage: None,
        })
    }

    pub fn get_metrics_by_type(&self, test_type: TestType) -> Result<TestMetrics> {
        let type_str = format!("{:?}", test_type);

        let mut stmt = self.conn.prepare(
            "SELECT
                COUNT(*) as total,
                SUM(CASE WHEN status = 'Passed' THEN 1 ELSE 0 END) as passed,
                SUM(CASE WHEN status = 'Failed' THEN 1 ELSE 0 END) as failed,
                SUM(CASE WHEN status = 'Skipped' THEN 1 ELSE 0 END) as skipped,
                SUM(CASE WHEN status = 'Timeout' THEN 1 ELSE 0 END) as timeout,
                SUM(CASE WHEN status = 'Error' THEN 1 ELSE 0 END) as error,
                SUM(duration_ms) as total_duration_ms
             FROM test_results
             WHERE test_type = ?1"
        )?;

        let (total, passed, failed, skipped, timeout, error, total_duration_ms): (i64, i64, i64, i64, i64, i64, i64) =
            stmt.query_row([&type_str], |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            })?;

        Ok(TestMetrics {
            total_tests: total as usize,
            passed: passed as usize,
            failed: failed as usize,
            skipped: skipped as usize,
            timeout: timeout as usize,
            error: error as usize,
            total_duration: std::time::Duration::from_millis(total_duration_ms as u64),
            coverage: None,
        })
    }
}
