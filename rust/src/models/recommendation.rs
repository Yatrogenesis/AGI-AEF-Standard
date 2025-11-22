use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::dimension::DimensionType;

/// Prioritized recommendation for improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Unique identifier
    pub id: uuid::Uuid,

    /// Priority level
    pub priority: RecommendationPriority,

    /// Related dimension
    pub dimension: DimensionType,

    /// Title of the recommendation
    pub title: String,

    /// Detailed description
    pub description: String,

    /// Current status (what's wrong)
    pub current_status: String,

    /// Desired outcome
    pub desired_outcome: String,

    /// Expected impact of implementing this
    pub expected_impact: ExpectedImpact,

    /// Improvement strategies
    pub strategies: Vec<ImprovementStrategy>,

    /// Resources needed
    pub resources: Vec<Resource>,

    /// Estimated effort
    pub effort: EffortLevel,

    /// Estimated timeline
    pub timeline: Timeline,

    /// Success criteria
    pub success_criteria: Vec<String>,

    /// Dependencies (other recommendations that should be done first)
    pub dependencies: Vec<uuid::Uuid>,

    /// Regulatory requirements addressed
    pub regulatory_requirements: Vec<String>,
}

/// Priority level for recommendations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RecommendationPriority {
    Critical,  // Must be addressed before deployment
    High,      // Should be addressed soon
    Medium,    // Important but not urgent
    Low,       // Nice to have
}

impl RecommendationPriority {
    pub fn description(&self) -> &str {
        match self {
            Self::Critical => "CRITICAL - Must be addressed immediately. Blocks deployment.",
            Self::High => "HIGH - Should be addressed in current development cycle.",
            Self::Medium => "MEDIUM - Important improvement for next iteration.",
            Self::Low => "LOW - Enhancement for future consideration.",
        }
    }

    pub fn color_code(&self) -> &str {
        match self {
            Self::Critical => "#FF0000", // Red
            Self::High => "#FF8C00",     // Orange
            Self::Medium => "#FFD700",   // Yellow
            Self::Low => "#90EE90",      // Light Green
        }
    }
}

/// Expected impact of implementing a recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedImpact {
    /// Predicted score increase (0-100)
    pub score_increase: f64,

    /// Affected dimensions
    pub affected_dimensions: Vec<DimensionType>,

    /// Improvement in audit status
    pub audit_status_improvement: bool,

    /// Regulatory compliance improvements
    pub compliance_improvements: Vec<String>,

    /// Risk reduction
    pub risk_reduction: RiskReduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReduction {
    pub safety_risks: Vec<String>,
    pub operational_risks: Vec<String>,
    pub compliance_risks: Vec<String>,
}

/// Strategy for implementing an improvement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementStrategy {
    /// Name of the strategy
    pub name: String,

    /// Detailed approach
    pub approach: String,

    /// Implementation steps
    pub steps: Vec<ImplementationStep>,

    /// Best practices to follow
    pub best_practices: Vec<String>,

    /// Common pitfalls to avoid
    pub pitfalls_to_avoid: Vec<String>,

    /// Example implementations
    pub examples: Vec<Example>,

    /// Related documentation
    pub documentation_links: Vec<DocumentationLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStep {
    pub order: usize,
    pub description: String,
    pub estimated_time: String,
    pub difficulty: Difficulty,
    pub deliverables: Vec<String>,
    pub validation_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Difficulty {
    Trivial,
    Easy,
    Medium,
    Hard,
    VeryHard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Example {
    pub title: String,
    pub description: String,
    pub code_snippet: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationLink {
    pub title: String,
    pub url: String,
    pub description: String,
}

/// Resource required for implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub description: String,
    pub quantity: Option<String>,
    pub estimated_cost: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Personnel,      // Human resources
    Computational,  // Computing power
    Data,          // Training/test data
    Tools,         // Software tools
    Infrastructure,// Hardware/cloud
    Expertise,     // Specialized knowledge
    Time,          // Development time
    Budget,        // Financial resources
}

/// Effort level required
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum EffortLevel {
    Minimal,      // Hours
    Low,          // Days
    Medium,       // Weeks
    High,         // Months
    VeryHigh,     // Quarters
}

impl EffortLevel {
    pub fn description(&self) -> &str {
        match self {
            Self::Minimal => "Minimal effort (hours to complete)",
            Self::Low => "Low effort (days to complete)",
            Self::Medium => "Medium effort (weeks to complete)",
            Self::High => "High effort (months to complete)",
            Self::VeryHigh => "Very high effort (quarters to complete)",
        }
    }
}

/// Timeline for implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub phases: Vec<Phase>,
    pub total_duration: String,
    pub milestones: Vec<Milestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    pub name: String,
    pub duration: String,
    pub objectives: Vec<String>,
    pub deliverables: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub name: String,
    pub description: String,
    pub target_date: Option<String>,
    pub success_criteria: Vec<String>,
}

impl Recommendation {
    /// Create a new recommendation
    pub fn new(
        priority: RecommendationPriority,
        dimension: DimensionType,
        title: String,
        description: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            priority,
            dimension,
            title,
            description,
            current_status: String::new(),
            desired_outcome: String::new(),
            expected_impact: ExpectedImpact {
                score_increase: 0.0,
                affected_dimensions: vec![dimension.clone()],
                audit_status_improvement: false,
                compliance_improvements: Vec::new(),
                risk_reduction: RiskReduction {
                    safety_risks: Vec::new(),
                    operational_risks: Vec::new(),
                    compliance_risks: Vec::new(),
                },
            },
            strategies: Vec::new(),
            resources: Vec::new(),
            effort: EffortLevel::Medium,
            timeline: Timeline {
                phases: Vec::new(),
                total_duration: "Unknown".to_string(),
                milestones: Vec::new(),
            },
            success_criteria: Vec::new(),
            dependencies: Vec::new(),
            regulatory_requirements: Vec::new(),
        }
    }

    /// Sort recommendations by priority and impact
    pub fn sort_by_priority(recommendations: &mut [Recommendation]) {
        recommendations.sort_by(|a, b| {
            // First by priority
            match a.priority.cmp(&b.priority) {
                std::cmp::Ordering::Equal => {
                    // Then by expected impact
                    b.expected_impact
                        .score_increase
                        .partial_cmp(&a.expected_impact.score_increase)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                other => other,
            }
        });
    }

    /// Get actionable items from this recommendation
    pub fn get_action_items(&self) -> Vec<String> {
        let mut items = Vec::new();

        for strategy in &self.strategies {
            for step in &strategy.steps {
                items.push(format!(
                    "Step {}: {} (est. {})",
                    step.order, step.description, step.estimated_time
                ));
            }
        }

        items
    }

    /// Check if this recommendation addresses regulatory requirements
    pub fn is_regulatory_critical(&self) -> bool {
        !self.regulatory_requirements.is_empty()
    }
}

/// Builder for creating detailed recommendations
pub struct RecommendationBuilder {
    recommendation: Recommendation,
}

impl RecommendationBuilder {
    pub fn new(
        priority: RecommendationPriority,
        dimension: DimensionType,
        title: String,
    ) -> Self {
        Self {
            recommendation: Recommendation::new(
                priority,
                dimension,
                title,
                String::new(),
            ),
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.recommendation.description = description;
        self
    }

    pub fn current_status(mut self, status: String) -> Self {
        self.recommendation.current_status = status;
        self
    }

    pub fn desired_outcome(mut self, outcome: String) -> Self {
        self.recommendation.desired_outcome = outcome;
        self
    }

    pub fn expected_impact(mut self, impact: ExpectedImpact) -> Self {
        self.recommendation.expected_impact = impact;
        self
    }

    pub fn add_strategy(mut self, strategy: ImprovementStrategy) -> Self {
        self.recommendation.strategies.push(strategy);
        self
    }

    pub fn add_resource(mut self, resource: Resource) -> Self {
        self.recommendation.resources.push(resource);
        self
    }

    pub fn effort(mut self, effort: EffortLevel) -> Self {
        self.recommendation.effort = effort;
        self
    }

    pub fn timeline(mut self, timeline: Timeline) -> Self {
        self.recommendation.timeline = timeline;
        self
    }

    pub fn add_success_criterion(mut self, criterion: String) -> Self {
        self.recommendation.success_criteria.push(criterion);
        self
    }

    pub fn add_dependency(mut self, dependency_id: uuid::Uuid) -> Self {
        self.recommendation.dependencies.push(dependency_id);
        self
    }

    pub fn add_regulatory_requirement(mut self, requirement: String) -> Self {
        self.recommendation.regulatory_requirements.push(requirement);
        self
    }

    pub fn build(self) -> Recommendation {
        self.recommendation
    }
}
