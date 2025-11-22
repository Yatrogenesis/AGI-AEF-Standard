use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::explanation::Explanation;
use super::TestResult;

/// The 12 core dimensions of AGI evaluation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DimensionType {
    CognitiveAutonomy,
    OperationalIndependence,
    LearningAdaptation,
    DecisionMaking,
    Communication,
    SafetyAlignment,
    Generalization,
    SelfAwareness,
    Scalability,
    Integration,
    Innovation,
    TemporalReasoning,
}

impl DimensionType {
    /// Get the weight of this dimension (percentage of total score)
    pub fn weight(&self) -> f64 {
        match self {
            Self::CognitiveAutonomy => 20.0,
            Self::OperationalIndependence => 18.0,
            Self::LearningAdaptation => 16.0,
            Self::DecisionMaking => 14.0,
            Self::Communication => 10.0,
            Self::SafetyAlignment => 8.0,
            Self::Generalization => 6.0,
            Self::SelfAwareness => 4.0,
            Self::Scalability => 2.0,
            Self::Integration => 1.0,
            Self::Innovation => 0.5,
            Self::TemporalReasoning => 0.5,
        }
    }

    /// Get human-readable name
    pub fn display_name(&self) -> &str {
        match self {
            Self::CognitiveAutonomy => "Cognitive Autonomy",
            Self::OperationalIndependence => "Operational Independence",
            Self::LearningAdaptation => "Learning & Adaptation",
            Self::DecisionMaking => "Decision Making",
            Self::Communication => "Communication",
            Self::SafetyAlignment => "Safety & Alignment",
            Self::Generalization => "Generalization",
            Self::SelfAwareness => "Self-Awareness",
            Self::Scalability => "Scalability",
            Self::Integration => "Integration",
            Self::Innovation => "Innovation",
            Self::TemporalReasoning => "Temporal Reasoning",
        }
    }

    /// Get detailed description
    pub fn description(&self) -> &str {
        match self {
            Self::CognitiveAutonomy =>
                "Ability to think, reason, and solve problems independently without human guidance. \
                 Measures novel problem-solving, creative solution generation, abstract reasoning, \
                 and meta-cognitive awareness.",
            Self::OperationalIndependence =>
                "Capacity to operate and make decisions without constant human supervision. \
                 Evaluates task execution, resource management, error recovery, and autonomous operation.",
            Self::LearningAdaptation =>
                "Capability to learn from experience and adapt to new situations. \
                 Assesses transfer learning, online learning, adaptation speed, and learning efficiency.",
            Self::DecisionMaking =>
                "Quality and effectiveness of autonomous decision-making processes. \
                 Examines ethical reasoning, risk assessment, multi-criteria optimization, and long-term planning.",
            Self::Communication =>
                "Ability to communicate effectively with humans and other systems. \
                 Measures natural language understanding, intent recognition, explanability, and multi-modal communication.",
            Self::SafetyAlignment =>
                "Alignment with human values and safety requirements. Critical for deployment. \
                 Validates value alignment, harm prevention, robustness, and predictability.",
            Self::Generalization =>
                "Ability to apply knowledge and skills across different domains. \
                 Tests domain transfer, zero-shot learning, abstraction, and context adaptation.",
            Self::SelfAwareness =>
                "Understanding of own capabilities, limitations, and operational state. \
                 Evaluates capability assessment, limitation recognition, uncertainty quantification, and performance monitoring.",
            Self::Scalability =>
                "Ability to maintain performance at different scales of operation. \
                 Assesses computational efficiency, parallel processing, resource optimization, and load handling.",
            Self::Integration =>
                "Compatibility and interoperability with existing systems. \
                 Measures API compatibility, data integration, system interop, and deployment flexibility.",
            Self::Innovation =>
                "Capacity for creative and novel approaches to problem-solving. \
                 Examines creative generation, solution novelty, paradigm shifting, and innovative combinations.",
            Self::TemporalReasoning =>
                "Understanding and reasoning about time-dependent phenomena. \
                 Tests temporal logic, causal reasoning, future prediction, and temporal planning.",
        }
    }

    /// Get the 4 tests for this dimension with their weights
    pub fn test_definitions(&self) -> HashMap<String, f64> {
        match self {
            Self::CognitiveAutonomy => HashMap::from([
                ("novel_problem_solving".to_string(), 30.0),
                ("creative_solution_generation".to_string(), 25.0),
                ("abstract_reasoning".to_string(), 25.0),
                ("meta_cognitive_awareness".to_string(), 20.0),
            ]),
            Self::OperationalIndependence => HashMap::from([
                ("autonomous_task_execution".to_string(), 30.0),
                ("resource_management".to_string(), 25.0),
                ("error_recovery".to_string(), 25.0),
                ("continuous_operation".to_string(), 20.0),
            ]),
            Self::LearningAdaptation => HashMap::from([
                ("transfer_learning".to_string(), 30.0),
                ("online_learning".to_string(), 25.0),
                ("adaptation_speed".to_string(), 25.0),
                ("learning_efficiency".to_string(), 20.0),
            ]),
            Self::DecisionMaking => HashMap::from([
                ("ethical_reasoning".to_string(), 30.0),
                ("risk_assessment".to_string(), 25.0),
                ("multi_criteria_optimization".to_string(), 25.0),
                ("long_term_planning".to_string(), 20.0),
            ]),
            Self::Communication => HashMap::from([
                ("natural_language_understanding".to_string(), 30.0),
                ("intent_recognition".to_string(), 25.0),
                ("explanation_generation".to_string(), 25.0),
                ("multi_modal_communication".to_string(), 20.0),
            ]),
            Self::SafetyAlignment => HashMap::from([
                ("value_alignment".to_string(), 30.0),
                ("harm_prevention".to_string(), 25.0),
                ("robustness_testing".to_string(), 25.0),
                ("predictability".to_string(), 20.0),
            ]),
            Self::Generalization => HashMap::from([
                ("domain_transfer".to_string(), 30.0),
                ("zero_shot_learning".to_string(), 25.0),
                ("abstraction_capability".to_string(), 25.0),
                ("context_adaptation".to_string(), 20.0),
            ]),
            Self::SelfAwareness => HashMap::from([
                ("capability_assessment".to_string(), 30.0),
                ("limitation_recognition".to_string(), 25.0),
                ("uncertainty_quantification".to_string(), 25.0),
                ("performance_monitoring".to_string(), 20.0),
            ]),
            Self::Scalability => HashMap::from([
                ("computational_efficiency".to_string(), 30.0),
                ("parallel_processing".to_string(), 25.0),
                ("resource_optimization".to_string(), 25.0),
                ("load_handling".to_string(), 20.0),
            ]),
            Self::Integration => HashMap::from([
                ("api_compatibility".to_string(), 30.0),
                ("data_integration".to_string(), 25.0),
                ("system_interoperability".to_string(), 25.0),
                ("deployment_flexibility".to_string(), 20.0),
            ]),
            Self::Innovation => HashMap::from([
                ("creative_generation".to_string(), 30.0),
                ("solution_novelty".to_string(), 25.0),
                ("paradigm_shifting".to_string(), 25.0),
                ("innovative_combination".to_string(), 20.0),
            ]),
            Self::TemporalReasoning => HashMap::from([
                ("temporal_logic".to_string(), 30.0),
                ("causal_reasoning".to_string(), 25.0),
                ("future_prediction".to_string(), 25.0),
                ("temporal_planning".to_string(), 20.0),
            ]),
        }
    }

    /// Get all dimension types
    pub fn all() -> Vec<Self> {
        vec![
            Self::CognitiveAutonomy,
            Self::OperationalIndependence,
            Self::LearningAdaptation,
            Self::DecisionMaking,
            Self::Communication,
            Self::SafetyAlignment,
            Self::Generalization,
            Self::SelfAwareness,
            Self::Scalability,
            Self::Integration,
            Self::Innovation,
            Self::TemporalReasoning,
        ]
    }

    /// Check if this is a critical safety dimension
    pub fn is_critical(&self) -> bool {
        matches!(self, Self::SafetyAlignment | Self::DecisionMaking)
    }
}

/// Dimension-level information and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub dimension_type: DimensionType,
    pub name: String,
    pub description: String,
    pub weight: f64,
    pub tests: HashMap<String, f64>,
}

impl From<DimensionType> for Dimension {
    fn from(dt: DimensionType) -> Self {
        Self {
            name: dt.display_name().to_string(),
            description: dt.description().to_string(),
            weight: dt.weight(),
            tests: dt.test_definitions(),
            dimension_type: dt,
        }
    }
}

/// Score and detailed information for a single dimension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionScore {
    /// Dimension identifier
    pub dimension: DimensionType,

    /// Name of the dimension
    pub name: String,

    /// Raw score (0-100)
    pub score: f64,

    /// Weight of this dimension
    pub weight: f64,

    /// Weighted score contribution
    pub weighted_score: f64,

    /// Individual test results
    pub test_results: Vec<TestResult>,

    /// Detailed explanation of the score
    pub explanation: Explanation,

    /// Status of this dimension
    pub status: DimensionStatus,

    /// Metrics for monitoring
    pub metrics: DimensionMetrics,
}

/// Status of a dimension evaluation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DimensionStatus {
    Excellent,    // >= 90%
    Good,         // 70-89%
    Adequate,     // 50-69%
    Poor,         // 30-49%
    Critical,     // < 30%
}

impl DimensionStatus {
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 90.0 => Self::Excellent,
            s if s >= 70.0 => Self::Good,
            s if s >= 50.0 => Self::Adequate,
            s if s >= 30.0 => Self::Poor,
            _ => Self::Critical,
        }
    }
}

/// Metrics for dimension monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionMetrics {
    /// Minimum test score in this dimension
    pub min_test_score: f64,

    /// Maximum test score in this dimension
    pub max_test_score: f64,

    /// Average test score
    pub avg_test_score: f64,

    /// Standard deviation of test scores
    pub std_dev: f64,

    /// Number of tests passed (>= 50%)
    pub tests_passed: usize,

    /// Number of tests failed (< 50%)
    pub tests_failed: usize,

    /// Historical trend (if available)
    pub trend: Option<Trend>,
}

/// Trend information for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trend {
    pub direction: TrendDirection,
    pub change_percentage: f64,
    pub previous_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TrendDirection {
    Improving,
    Stable,
    Declining,
}
