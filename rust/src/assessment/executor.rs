use super::{AGISystem, AssessmentError, TestConfig};
use crate::models::TestResult;
use std::time::Duration;
use tokio::time::timeout;

/// Executor for running tests against AGI systems
pub struct TestExecutor {
    test_timeout: Duration,
    max_concurrent: usize,
    parallel_execution: bool,
}

impl TestExecutor {
    pub fn new(test_timeout: Duration, max_concurrent: usize, parallel_execution: bool) -> Self {
        Self {
            test_timeout,
            max_concurrent,
            parallel_execution,
        }
    }

    /// Execute multiple tests, optionally in parallel
    pub async fn execute_tests<T: AGISystem>(
        &self,
        system: &T,
        tests: &[TestConfig],
    ) -> Result<Vec<TestResult>, AssessmentError> {
        if self.parallel_execution {
            self.execute_parallel(system, tests).await
        } else {
            self.execute_sequential(system, tests).await
        }
    }

    /// Execute tests in parallel with concurrency limit
    async fn execute_parallel<T: AGISystem>(
        &self,
        system: &T,
        tests: &[TestConfig],
    ) -> Result<Vec<TestResult>, AssessmentError> {
        use futures::stream::{self, StreamExt};

        let results = stream::iter(tests)
            .map(|test| self.execute_single_test(system, test))
            .buffer_unordered(self.max_concurrent)
            .collect::<Vec<_>>()
            .await;

        results.into_iter().collect()
    }

    /// Execute tests sequentially
    async fn execute_sequential<T: AGISystem>(
        &self,
        system: &T,
        tests: &[TestConfig],
    ) -> Result<Vec<TestResult>, AssessmentError> {
        let mut results = Vec::new();
        for test in tests {
            let result = self.execute_single_test(system, test).await?;
            results.push(result);
        }
        Ok(results)
    }

    /// Execute a single test with timeout and retry logic
    async fn execute_single_test<T: AGISystem>(
        &self,
        system: &T,
        test_config: &TestConfig,
    ) -> Result<TestResult, AssessmentError> {
        let mut retries = test_config.max_retries;
        let mut last_error = None;

        loop {
            match timeout(
                Duration::from_millis(test_config.timeout_ms),
                system.execute_test(&test_config.name, test_config),
            )
            .await
            {
                Ok(Ok(result)) => return Ok(result),
                Ok(Err(e)) => last_error = Some(e),
                Err(_) => {
                    last_error = Some(AssessmentError::TestTimeout(format!(
                        "Test '{}' timed out after {}ms",
                        test_config.name, test_config.timeout_ms
                    )));
                }
            }

            if retries == 0 {
                break;
            }
            retries -= 1;
            tracing::warn!(
                test = %test_config.name,
                retries_left = retries,
                "Test failed, retrying..."
            );
        }

        Err(last_error.unwrap_or_else(|| {
            AssessmentError::TestExecutionFailed("Unknown error".to_string())
        }))
    }
}
