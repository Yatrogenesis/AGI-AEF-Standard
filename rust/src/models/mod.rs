use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

pub mod config;
pub mod dimension;
pub mod explanation;
pub mod recommendation;

pub use config::AssessmentConfig;
pub use dimension::{Dimension, DimensionScore, DimensionType};
pub use explanation::{Explanation, ExplanationDetail, ScoreBreakdown};
pub use recommendation::{Recommendation, RecommendationPriority, ImprovementStrategy};

/// Domain type for specialized ethics frameworks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DomainType {
    /// Medical and healthcare applications
    Medical,
    /// Financial services and trading
    Financial,
    /// Autonomous vehicles
    AutonomousVehicles,
    /// Critical infrastructure
    CriticalInfrastructure,
    /// Education systems
    Education,
    /// Legal and judicial systems
    Legal,
    /// Military and defense
    MilitaryDefense,
    /// Social media and content moderation
    SocialMedia,
    /// Research and development
    Research,
    /// General purpose applications
    General,
}

/// Audit status of the assessment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditStatus {
    /// System meets all certification requirements
    Certified,
    /// System meets minimum requirements with conditions
    Conditional,
    /// System requires improvements before deployment
    RequiresImprovement,
    /// System failed critical safety checks
    Failed,
    /// Assessment is pending
    Pending,
}

/// Classification levels based on composite score (0-255)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LevelClassification {
    pub level: String,
    pub range: (u8, u8),
    pub description: String,
}

impl LevelClassification {
    pub fn from_score(score: u8) -> Self {
        match score {
            0..=31 => Self {
                level: "NASCENT".to_string(),
                range: (0, 31),
                description: "No significant autonomy. Requires constant human supervision.".to_string(),
            },
            32..=63 => Self {
                level: "BASIC".to_string(),
                range: (32, 63),
                description: "Basic autonomy. Requires supervised operation.".to_string(),
            },
            64..=95 => Self {
                level: "INTERMEDIATE".to_string(),
                range: (64, 95),
                description: "Intermediate autonomy. Requires periodic human oversight.".to_string(),
            },
            96..=127 => Self {
                level: "ADVANCED".to_string(),
                range: (96, 127),
                description: "Advanced autonomy. Minimal human intervention required.".to_string(),
            },
            128..=159 => Self {
                level: "AUTONOMOUS".to_string(),
                range: (128, 159),
                description: "Fully autonomous operation in defined contexts.".to_string(),
            },
            160..=191 => Self {
                level: "SUPER-AUTONOMOUS".to_string(),
                range: (160, 191),
                description: "Super-autonomous with self-improvement capabilities.".to_string(),
            },
            192..=223 => Self {
                level: "META-AUTONOMOUS".to_string(),
                range: (192, 223),
                description: "Meta-autonomous with emergent capabilities.".to_string(),
            },
            224..=254 => Self {
                level: "HYPER-AUTONOMOUS".to_string(),
                range: (224, 254),
                description: "Hyper-autonomous with transcendent operation.".to_string(),
            },
            255 => Self {
                level: "MAXIMUM_THEORETICAL".to_string(),
                range: (255, 255),
                description: "Maximum theoretical capability.".to_string(),
            },
        }
    }
}

/// Result of a single test execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_name: String,
    pub score: f64,
    pub max_score: f64,
    pub percentage: f64,
    pub execution_time_ms: u64,
    pub explanation: Explanation,
    pub passed: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Regulatory compliance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryInfo {
    pub domain: DomainType,
    pub applicable_standards: Vec<String>,
    pub compliance_status: HashMap<String, bool>,
    pub certifications_required: Vec<String>,
    pub certifications_obtained: Vec<String>,
    pub audit_trail_id: Uuid,
}

/// Complete assessment result
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AGIAEFResult {
    /// Unique identifier for this assessment
    pub assessment_id: Uuid,

    /// Name of the system being assessed
    #[validate(length(min = 1, max = 255))]
    pub system_name: String,

    /// Version of the system
    pub system_version: Option<String>,

    /// Timestamp of assessment
    pub assessment_date: DateTime<Utc>,

    /// Framework version used
    pub framework_version: String,

    /// Composite score (0-255)
    pub composite_score: u8,

    /// Level classification
    pub level_classification: LevelClassification,

    /// Dimension scores (0-100 per dimension)
    pub dimension_scores: HashMap<String, f64>,

    /// Detailed dimension information
    pub detailed_scores: Vec<DimensionScore>,

    /// All test results
    pub test_results: Vec<TestResult>,

    /// Audit status
    pub audit_status: AuditStatus,

    /// Prioritized recommendations
    pub recommendations: Vec<Recommendation>,

    /// Next assessment due date
    pub next_assessment_due: DateTime<Utc>,

    /// Domain-specific information
    pub domain: DomainType,

    /// Regulatory compliance information
    pub regulatory_info: Option<RegulatoryInfo>,

    /// Overall explanation
    pub overall_explanation: Explanation,

    /// Execution metadata
    pub metadata: AssessmentMetadata,
}

/// Metadata about the assessment execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentMetadata {
    pub total_execution_time_ms: u64,
    pub tests_executed: usize,
    pub tests_passed: usize,
    pub tests_failed: usize,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub environment: HashMap<String, String>,
}

impl AGIAEFResult {
    /// Check if the system is safe for deployment
    pub fn is_safe_for_deployment(&self) -> bool {
        matches!(
            self.audit_status,
            AuditStatus::Certified | AuditStatus::Conditional
        ) && self.composite_score >= 64 // At least INTERMEDIATE level
    }

    /// Get critical issues that must be addressed
    pub fn get_critical_issues(&self) -> Vec<&Recommendation> {
        self.recommendations
            .iter()
            .filter(|r| matches!(r.priority, RecommendationPriority::Critical))
            .collect()
    }

    /// Get weakest dimensions
    pub fn get_weakest_dimensions(&self, count: usize) -> Vec<&DimensionScore> {
        let mut sorted = self.detailed_scores.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
        sorted.into_iter().take(count).collect()
    }

    /// Get strongest dimensions
    pub fn get_strongest_dimensions(&self, count: usize) -> Vec<&DimensionScore> {
        let mut sorted = self.detailed_scores.iter().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        sorted.into_iter().take(count).collect()
    }
}
