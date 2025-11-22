/// Basic assessment example
///
/// This example demonstrates how to implement the AGISystem trait
/// and run a basic assessment.

use agi_aef_rust::{
    assessment::{AGISystem, AssessmentError, SystemMetadata, TestConfig},
    models::{TestResult, Explanation},
    AGIAEFAssessment, AssessmentConfig,
};
use async_trait::async_trait;
use std::collections::HashMap;

/// Example AGI system implementation
struct ExampleAGISystem {
    name: String,
    capabilities: Vec<String>,
}

impl ExampleAGISystem {
    fn new(name: String) -> Self {
        Self {
            name,
            capabilities: vec![
                "natural_language_processing".to_string(),
                "reasoning".to_string(),
                "learning".to_string(),
            ],
        }
    }
}

#[async_trait]
impl AGISystem for ExampleAGISystem {
    async fn execute_test(
        &self,
        test_name: &str,
        _test_config: &TestConfig,
    ) -> Result<TestResult, AssessmentError> {
        // Simulate test execution
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Return mock test result
        // In a real implementation, this would actually test the system
        let score = match test_name {
            "harm_prevention" => 95.0,
            "value_alignment" => 90.0,
            "robustness_testing" => 88.0,
            "predictability" => 85.0,
            "novel_problem_solving" => 82.0,
            "creative_solution_generation" => 80.0,
            "abstract_reasoning" => 85.0,
            "meta_cognitive_awareness" => 75.0,
            _ => 70.0, // Default score for other tests
        };

        let max_score = 100.0;
        let percentage = (score / max_score) * 100.0;

        Ok(TestResult {
            test_name: test_name.to_string(),
            score,
            max_score,
            percentage,
            execution_time_ms: 100,
            explanation: Explanation::new(format!(
                "Test '{}' completed with score {:.1}/{}",
                test_name, score, max_score
            )),
            passed: percentage >= 50.0,
            metadata: HashMap::new(),
        })
    }

    fn get_metadata(&self) -> SystemMetadata {
        SystemMetadata {
            name: self.name.clone(),
            version: "1.0.0".to_string(),
            description: "Example AGI system for demonstration".to_string(),
            vendor: "Example Corp".to_string(),
            capabilities: self.capabilities.clone(),
            limitations: vec![
                "Cannot process images".to_string(),
                "Limited to English language".to_string(),
            ],
            environment: HashMap::from([
                ("runtime".to_string(), "tokio".to_string()),
                ("language".to_string(), "rust".to_string()),
            ]),
        }
    }

    async fn prepare(&mut self) -> Result<(), AssessmentError> {
        println!("Preparing {} for assessment...", self.name);
        Ok(())
    }

    async fn cleanup(&mut self) -> Result<(), AssessmentError> {
        println!("Cleaning up {} after assessment...", self.name);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("AGI-AEF Basic Assessment Example\n");
    println!("================================\n");

    // Create an example AGI system
    let system = ExampleAGISystem::new("ExampleAGI-v1".to_string());

    // Create assessment with default configuration
    let assessment = AGIAEFAssessment::default()?;

    println!("Running comprehensive assessment...\n");

    // Run the assessment
    let result = assessment
        .run_comprehensive_assessment("ExampleAGI", system)
        .await?;

    // Display results
    println!("\n=== Assessment Results ===\n");
    println!("System: {}", result.system_name);
    println!("Date: {}", result.assessment_date);
    println!("Framework Version: {}", result.framework_version);
    println!();
    println!("Composite Score: {}/255", result.composite_score);
    println!("Level: {}", result.level_classification.level);
    println!("Description: {}", result.level_classification.description);
    println!();
    println!("Audit Status: {:?}", result.audit_status);
    println!();

    // Display dimension scores
    println!("=== Dimension Scores ===\n");
    for dim_score in &result.detailed_scores {
        println!(
            "{:30} {:>6.1}% (Weight: {:>4.1}%) - {:?}",
            dim_score.name, dim_score.score, dim_score.weight, dim_score.status
        );
    }
    println!();

    // Display weakest dimensions
    println!("=== Areas for Improvement ===\n");
    for dim in result.get_weakest_dimensions(3) {
        println!(
            "- {} ({:.1}%): Needs attention",
            dim.name, dim.score
        );
    }
    println!();

    // Display recommendations
    if !result.recommendations.is_empty() {
        println!("=== Recommendations ({}) ===\n", result.recommendations.len());
        for (i, rec) in result.recommendations.iter().take(5).enumerate() {
            println!("{}. [{:?}] {}", i + 1, rec.priority, rec.title);
            println!("   {}", rec.description);
            println!();
        }
    }

    // Check if deployment is acceptable
    println!("=== Deployment Readiness ===\n");
    if result.is_safe_for_deployment() {
        println!("✅ System PASSED assessment and is safe for deployment!");
    } else {
        println!("❌ System FAILED assessment. Improvements required before deployment.");
        let critical = result.get_critical_issues();
        println!("\nCritical issues that must be addressed: {}", critical.len());
        for issue in critical {
            println!("  - {}", issue.title);
        }
    }
    println!();

    // Export results to JSON
    let json_path = std::path::Path::new("assessment_results.json");
    AGIAEFAssessment::export_json_file(&result, json_path).await?;
    println!("Results exported to: {:?}", json_path);

    // Generate HTML report
    let html_report = AGIAEFAssessment::generate_html_report(&result)?;
    std::fs::write("assessment_report.html", html_report)?;
    println!("HTML report generated: assessment_report.html");

    println!("\n=== Assessment Complete ===");

    Ok(())
}
