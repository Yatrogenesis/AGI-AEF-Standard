use super::{AssessmentEngine, AssessmentError};
use crate::models::{AGIAEFResult, AssessmentConfig};

/// Main entry point for AGI assessments
pub struct AGIAEFAssessment {
    engine: AssessmentEngine,
}

impl AGIAEFAssessment {
    /// Create a new assessment instance with the given configuration
    pub fn new(config: AssessmentConfig) -> Result<Self, AssessmentError> {
        let engine = AssessmentEngine::new(config)?;
        Ok(Self { engine })
    }

    /// Create an assessment instance with default configuration
    pub fn default() -> Result<Self, AssessmentError> {
        Self::new(AssessmentConfig::default())
    }

    /// Create an assessment for medical domain
    pub fn for_medical_domain() -> Result<Self, AssessmentError> {
        Self::new(AssessmentConfig::medical())
    }

    /// Create an assessment for financial domain
    pub fn for_financial_domain() -> Result<Self, AssessmentError> {
        Self::new(AssessmentConfig::financial())
    }

    /// Create an assessment for autonomous vehicles
    pub fn for_autonomous_vehicles() -> Result<Self, AssessmentError> {
        Self::new(AssessmentConfig::autonomous_vehicles())
    }

    /// Create an assessment for critical infrastructure
    pub fn for_critical_infrastructure() -> Result<Self, AssessmentError> {
        Self::new(AssessmentConfig::critical_infrastructure())
    }

    /// Create a quick assessment (for testing/development)
    pub fn quick() -> Result<Self, AssessmentError> {
        Self::new(AssessmentConfig::quick())
    }

    /// Run a comprehensive assessment
    pub async fn run_comprehensive_assessment<T: super::AGISystem>(
        &self,
        system_name: &str,
        system: T,
    ) -> Result<AGIAEFResult, AssessmentError> {
        self.engine
            .run_comprehensive_assessment(system_name, system)
            .await
    }

    /// Export assessment result to JSON
    pub fn export_json(result: &AGIAEFResult) -> Result<String, AssessmentError> {
        serde_json::to_string_pretty(result).map_err(AssessmentError::from)
    }

    /// Export assessment result to JSON file
    pub async fn export_json_file(
        result: &AGIAEFResult,
        path: &std::path::Path,
    ) -> Result<(), AssessmentError> {
        let json = Self::export_json(result)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    /// Generate a detailed HTML report
    pub fn generate_html_report(result: &AGIAEFResult) -> Result<String, AssessmentError> {
        // This will use the Tera templating engine
        // Placeholder for now
        Ok(format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>AGI Assessment Report - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ background: #2c3e50; color: white; padding: 20px; }}
        .score {{ font-size: 48px; font-weight: bold; }}
        .dimension {{ margin: 20px 0; padding: 15px; border-left: 4px solid #3498db; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>AGI Alignment & Ethics Framework Assessment</h1>
        <h2>System: {}</h2>
        <div class="score">{}/255</div>
        <p>Level: {}</p>
        <p>Status: {:?}</p>
    </div>
    <div class="content">
        <h2>Dimension Scores</h2>
        <!-- Dimension details would be rendered here -->
    </div>
</body>
</html>"#,
            result.system_name,
            result.system_name,
            result.composite_score,
            result.level_classification.level,
            result.audit_status
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assessment_creation() {
        let assessment = AGIAEFAssessment::default();
        assert!(assessment.is_ok());
    }

    #[test]
    fn test_domain_specific_assessments() {
        assert!(AGIAEFAssessment::for_medical_domain().is_ok());
        assert!(AGIAEFAssessment::for_financial_domain().is_ok());
        assert!(AGIAEFAssessment::for_autonomous_vehicles().is_ok());
        assert!(AGIAEFAssessment::for_critical_infrastructure().is_ok());
    }
}
