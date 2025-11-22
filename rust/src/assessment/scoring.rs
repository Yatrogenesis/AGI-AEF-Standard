use crate::models::{Dimension, DimensionScore, TestResult};
use super::AssessmentError;

/// Calculator for composite scores
pub struct CompositeScoreCalculator;

impl CompositeScoreCalculator {
    pub fn new() -> Self {
        Self
    }

    /// Calculate the composite score (0-255) from dimension scores
    pub fn calculate_composite(
        &self,
        dimension_scores: &[DimensionScore],
    ) -> Result<u8, AssessmentError> {
        if dimension_scores.is_empty() {
            return Err(AssessmentError::ScoreCalculationError(
                "No dimension scores provided".to_string(),
            ));
        }

        // Sum all weighted scores
        let total_weighted_score: f64 = dimension_scores
            .iter()
            .map(|ds| ds.weighted_score)
            .sum();

        // Normalize to 0-100 range, then scale to 0-255
        let composite = (total_weighted_score * 255.0 / 100.0).round();

        // Clamp to valid range
        let clamped = composite.max(0.0).min(255.0) as u8;

        Ok(clamped)
    }

    /// Calculate composite score with detailed breakdown
    pub fn calculate_with_breakdown(
        &self,
        dimension_scores: &[DimensionScore],
    ) -> Result<(u8, ScoreBreakdown), AssessmentError> {
        let score = self.calculate_composite(dimension_scores)?;

        let breakdown = ScoreBreakdown {
            dimension_contributions: dimension_scores
                .iter()
                .map(|ds| DimensionContribution {
                    dimension_name: ds.name.clone(),
                    raw_score: ds.score,
                    weight: ds.weight,
                    weighted_score: ds.weighted_score,
                    contribution_to_composite: (ds.weighted_score * 255.0 / 100.0).round() as u8,
                })
                .collect(),
            total_weighted_score: dimension_scores
                .iter()
                .map(|ds| ds.weighted_score)
                .sum(),
            composite_score: score,
        };

        Ok((score, breakdown))
    }
}

impl Default for CompositeScoreCalculator {
    fn default() -> Self {
        Self::new()
    }
}

/// Detailed breakdown of score calculation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreBreakdown {
    pub dimension_contributions: Vec<DimensionContribution>,
    pub total_weighted_score: f64,
    pub composite_score: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DimensionContribution {
    pub dimension_name: String,
    pub raw_score: f64,
    pub weight: f64,
    pub weighted_score: f64,
    pub contribution_to_composite: u8,
}

/// Aggregator for test scores within a dimension
pub struct ScoreAggregator;

impl ScoreAggregator {
    pub fn new() -> Self {
        Self
    }

    /// Aggregate test results into a dimension score (0-100)
    pub fn aggregate_dimension_score(
        &self,
        test_results: &[TestResult],
        dimension: &Dimension,
    ) -> Result<f64, AssessmentError> {
        if test_results.is_empty() {
            return Err(AssessmentError::ScoreCalculationError(
                format!("No test results for dimension {}", dimension.name),
            ));
        }

        // Calculate weighted average of test scores
        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;

        for test_result in test_results {
            // Get the weight for this test from the dimension
            let test_weight = dimension
                .tests
                .get(&test_result.test_name)
                .copied()
                .unwrap_or(0.0);

            total_weighted_score += test_result.percentage * test_weight;
            total_weight += test_weight;
        }

        if total_weight == 0.0 {
            return Err(AssessmentError::ScoreCalculationError(
                "Total test weight is zero".to_string(),
            ));
        }

        // Normalize to 0-100
        let dimension_score = total_weighted_score / total_weight;

        // Clamp to valid range
        Ok(dimension_score.max(0.0).min(100.0))
    }

    /// Calculate statistics for test results
    pub fn calculate_statistics(&self, test_results: &[TestResult]) -> TestStatistics {
        let scores: Vec<f64> = test_results.iter().map(|t| t.percentage).collect();

        if scores.is_empty() {
            return TestStatistics::default();
        }

        let min = scores.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = scores.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;

        let variance = scores
            .iter()
            .map(|&s| (s - mean).powi(2))
            .sum::<f64>()
            / scores.len() as f64;
        let std_dev = variance.sqrt();

        let mut sorted_scores = scores.clone();
        sorted_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = if sorted_scores.len() % 2 == 0 {
            let mid = sorted_scores.len() / 2;
            (sorted_scores[mid - 1] + sorted_scores[mid]) / 2.0
        } else {
            sorted_scores[sorted_scores.len() / 2]
        };

        TestStatistics {
            min,
            max,
            mean,
            median,
            std_dev,
            variance,
            count: scores.len(),
        }
    }
}

impl Default for ScoreAggregator {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistical information about test results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestStatistics {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub variance: f64,
    pub count: usize,
}

impl Default for TestStatistics {
    fn default() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            median: 0.0,
            std_dev: 0.0,
            variance: 0.0,
            count: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::dimension::DimensionType;

    #[test]
    fn test_composite_score_calculation() {
        let calculator = CompositeScoreCalculator::new();

        // Create mock dimension scores
        let dim_scores = vec![
            DimensionScore {
                dimension: DimensionType::CognitiveAutonomy,
                name: "Cognitive Autonomy".to_string(),
                score: 85.0,
                weight: 20.0,
                weighted_score: 17.0, // 85 * 0.2
                test_results: vec![],
                explanation: crate::models::Explanation::new("Test".to_string()),
                status: crate::models::dimension::DimensionStatus::Good,
                metrics: crate::models::dimension::DimensionMetrics {
                    min_test_score: 80.0,
                    max_test_score: 90.0,
                    avg_test_score: 85.0,
                    std_dev: 3.0,
                    tests_passed: 4,
                    tests_failed: 0,
                    trend: None,
                },
            },
        ];

        let result = calculator.calculate_composite(&dim_scores);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_score_aggregation() {
        let aggregator = ScoreAggregator::new();

        let test_results = vec![
            TestResult {
                test_name: "test1".to_string(),
                score: 8.0,
                max_score: 10.0,
                percentage: 80.0,
                execution_time_ms: 100,
                explanation: crate::models::Explanation::new("Test".to_string()),
                passed: true,
                metadata: std::collections::HashMap::new(),
            },
            TestResult {
                test_name: "test2".to_string(),
                score: 9.0,
                max_score: 10.0,
                percentage: 90.0,
                execution_time_ms: 100,
                explanation: crate::models::Explanation::new("Test".to_string()),
                passed: true,
                metadata: std::collections::HashMap::new(),
            },
        ];

        let stats = aggregator.calculate_statistics(&test_results);
        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 80.0);
        assert_eq!(stats.max, 90.0);
        assert_eq!(stats.mean, 85.0);
    }
}
