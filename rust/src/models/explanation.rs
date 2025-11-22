use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Detailed explanation for scores and decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    /// Summary of the explanation
    pub summary: String,

    /// Detailed breakdown
    pub details: Vec<ExplanationDetail>,

    /// Key insights
    pub insights: Vec<String>,

    /// Supporting evidence
    pub evidence: Vec<Evidence>,

    /// Score breakdown
    pub score_breakdown: ScoreBreakdown,

    /// Visual representation data
    pub visualization_data: Option<VisualizationData>,
}

impl Explanation {
    pub fn new(summary: String) -> Self {
        Self {
            summary,
            details: Vec::new(),
            insights: Vec::new(),
            evidence: Vec::new(),
            score_breakdown: ScoreBreakdown::default(),
            visualization_data: None,
        }
    }

    pub fn add_detail(&mut self, detail: ExplanationDetail) {
        self.details.push(detail);
    }

    pub fn add_insight(&mut self, insight: String) {
        self.insights.push(insight);
    }

    pub fn add_evidence(&mut self, evidence: Evidence) {
        self.evidence.push(evidence);
    }
}

/// Detailed explanation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationDetail {
    /// Category of this detail
    pub category: String,

    /// Title of the detail
    pub title: String,

    /// Description
    pub description: String,

    /// Impact on score
    pub impact: Impact,

    /// Supporting data points
    pub data_points: HashMap<String, serde_json::Value>,
}

/// Impact level on the overall score
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Impact {
    Critical,   // Major impact on score
    High,       // Significant impact
    Medium,     // Moderate impact
    Low,        // Minor impact
    Negligible, // Minimal impact
}

/// Evidence supporting the explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    /// Type of evidence
    pub evidence_type: EvidenceType,

    /// Description
    pub description: String,

    /// Source of the evidence
    pub source: String,

    /// Confidence level (0.0-1.0)
    pub confidence: f64,

    /// Timestamp
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,

    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceType {
    TestResult,
    Measurement,
    Observation,
    Calculation,
    HistoricalData,
    Benchmark,
    ExternalValidation,
}

/// Breakdown of how the score was calculated
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScoreBreakdown {
    /// Components that contributed to the score
    pub components: Vec<ScoreComponent>,

    /// Total raw score
    pub raw_score: f64,

    /// Maximum possible score
    pub max_score: f64,

    /// Normalized score (percentage)
    pub normalized_score: f64,

    /// Applied weights
    pub weights: HashMap<String, f64>,

    /// Formula used for calculation
    pub formula: String,
}

/// Individual component of a score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreComponent {
    /// Name of the component
    pub name: String,

    /// Component's raw score
    pub value: f64,

    /// Maximum possible value
    pub max_value: f64,

    /// Percentage of max
    pub percentage: f64,

    /// Weight in final calculation
    pub weight: f64,

    /// Contribution to final score
    pub contribution: f64,

    /// Why this value was assigned
    pub rationale: String,
}

/// Data for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationData {
    /// Chart type recommended
    pub chart_type: ChartType,

    /// Data series for plotting
    pub series: Vec<DataSeries>,

    /// Axis labels
    pub axes: AxisLabels,

    /// Thresholds to display
    pub thresholds: Vec<Threshold>,

    /// Color coding information
    pub color_coding: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChartType {
    RadarChart,
    BarChart,
    LineChart,
    HeatMap,
    GaugeChart,
    TreeMap,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSeries {
    pub name: String,
    pub data: Vec<DataPoint>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisLabels {
    pub x_axis: String,
    pub y_axis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub name: String,
    pub value: f64,
    pub color: String,
    pub description: String,
}

impl ScoreBreakdown {
    pub fn new(raw_score: f64, max_score: f64) -> Self {
        let normalized_score = if max_score > 0.0 {
            (raw_score / max_score) * 100.0
        } else {
            0.0
        };

        Self {
            components: Vec::new(),
            raw_score,
            max_score,
            normalized_score,
            weights: HashMap::new(),
            formula: "score = (Σ component_values / max_possible) × 100".to_string(),
        }
    }

    pub fn add_component(&mut self, component: ScoreComponent) {
        self.components.push(component);
    }

    pub fn add_weight(&mut self, name: String, weight: f64) {
        self.weights.insert(name, weight);
    }
}

impl ScoreComponent {
    pub fn new(
        name: String,
        value: f64,
        max_value: f64,
        weight: f64,
        rationale: String,
    ) -> Self {
        let percentage = if max_value > 0.0 {
            (value / max_value) * 100.0
        } else {
            0.0
        };
        let contribution = percentage * (weight / 100.0);

        Self {
            name,
            value,
            max_value,
            percentage,
            weight,
            contribution,
            rationale,
        }
    }
}
