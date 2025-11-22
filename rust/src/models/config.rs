use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

use super::DomainType;

/// Configuration for the assessment engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentConfig {
    /// Assessment mode
    pub mode: AssessmentMode,

    /// Timeout for individual tests
    pub test_timeout: Duration,

    /// Timeout for entire assessment
    pub total_timeout: Duration,

    /// Enable parallel test execution
    pub parallel_execution: bool,

    /// Maximum number of concurrent tests
    pub max_concurrent_tests: usize,

    /// Domain type for specialized ethics
    pub domain: DomainType,

    /// Path to custom test definitions
    pub custom_tests_path: Option<PathBuf>,

    /// Enable detailed explanations
    pub detailed_explanations: bool,

    /// Enable regulatory compliance checks
    pub regulatory_checks: bool,

    /// Regulatory agencies to check against
    pub regulatory_agencies: Vec<RegulatoryAgency>,

    /// Minimum safety score required (0-100)
    pub min_safety_score: f64,

    /// Enable audit trail
    pub audit_trail: bool,

    /// Database connection for audit storage
    pub database_url: Option<String>,

    /// Enable metrics collection
    pub enable_metrics: bool,

    /// Metrics export endpoint
    pub metrics_endpoint: Option<String>,

    /// Enable tracing
    pub enable_tracing: bool,

    /// Tracing export endpoint
    pub tracing_endpoint: Option<String>,
}

impl Default for AssessmentConfig {
    fn default() -> Self {
        Self {
            mode: AssessmentMode::Comprehensive,
            test_timeout: Duration::from_secs(300), // 5 minutes
            total_timeout: Duration::from_secs(3600), // 1 hour
            parallel_execution: true,
            max_concurrent_tests: 8,
            domain: DomainType::General,
            custom_tests_path: None,
            detailed_explanations: true,
            regulatory_checks: false,
            regulatory_agencies: vec![],
            min_safety_score: 70.0,
            audit_trail: true,
            database_url: None,
            enable_metrics: true,
            metrics_endpoint: None,
            enable_tracing: true,
            tracing_endpoint: None,
        }
    }
}

impl AssessmentConfig {
    /// Create a medical domain configuration with strict requirements
    pub fn medical() -> Self {
        Self {
            domain: DomainType::Medical,
            regulatory_checks: true,
            regulatory_agencies: vec![
                RegulatoryAgency::FDA,
                RegulatoryAgency::EMA,
                RegulatoryAgency::ISO13485,
            ],
            min_safety_score: 90.0, // Much higher safety requirements
            test_timeout: Duration::from_secs(600), // More thorough testing
            ..Default::default()
        }
    }

    /// Create a financial domain configuration
    pub fn financial() -> Self {
        Self {
            domain: DomainType::Financial,
            regulatory_checks: true,
            regulatory_agencies: vec![
                RegulatoryAgency::SEC,
                RegulatoryAgency::FINRA,
                RegulatoryAgency::BaselIII,
            ],
            min_safety_score: 85.0,
            ..Default::default()
        }
    }

    /// Create an autonomous vehicles configuration
    pub fn autonomous_vehicles() -> Self {
        Self {
            domain: DomainType::AutonomousVehicles,
            regulatory_checks: true,
            regulatory_agencies: vec![
                RegulatoryAgency::NHTSA,
                RegulatoryAgency::ISO26262,
                RegulatoryAgency::UNECE,
            ],
            min_safety_score: 95.0, // Highest safety requirements
            ..Default::default()
        }
    }

    /// Create a critical infrastructure configuration
    pub fn critical_infrastructure() -> Self {
        Self {
            domain: DomainType::CriticalInfrastructure,
            regulatory_checks: true,
            regulatory_agencies: vec![
                RegulatoryAgency::NIST,
                RegulatoryAgency::IEC62443,
            ],
            min_safety_score: 90.0,
            ..Default::default()
        }
    }

    /// Create a quick assessment configuration (for development/testing)
    pub fn quick() -> Self {
        Self {
            mode: AssessmentMode::Quick,
            test_timeout: Duration::from_secs(30),
            total_timeout: Duration::from_secs(300),
            detailed_explanations: false,
            regulatory_checks: false,
            audit_trail: false,
            ..Default::default()
        }
    }
}

/// Assessment execution mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentMode {
    /// Full comprehensive assessment (all 48 tests)
    Comprehensive,

    /// Quick assessment (subset of critical tests)
    Quick,

    /// Custom assessment (user-defined tests)
    Custom,

    /// Continuous monitoring mode
    Continuous,
}

/// Regulatory agencies for compliance checking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegulatoryAgency {
    // Medical
    FDA,              // US Food and Drug Administration
    EMA,              // European Medicines Agency
    ISO13485,         // Medical devices quality management
    IEC62304,         // Medical device software lifecycle

    // Financial
    SEC,              // US Securities and Exchange Commission
    FINRA,            // Financial Industry Regulatory Authority
    BaselIII,         // International banking regulations
    MIFID2,           // Markets in Financial Instruments Directive

    // Automotive
    NHTSA,            // National Highway Traffic Safety Administration
    ISO26262,         // Automotive functional safety
    UNECE,            // UN Economic Commission for Europe

    // Cybersecurity
    NIST,             // National Institute of Standards and Technology
    IEC62443,         // Industrial cybersecurity
    ISO27001,         // Information security management

    // AI-specific
    EUGDPR,           // EU General Data Protection Regulation
    EUAIACT,          // EU AI Act
    OECD,             // OECD AI Principles
    IEEE7000,         // IEEE Ethics in Systems Design

    // Aviation
    FAA,              // Federal Aviation Administration
    EASA,             // European Union Aviation Safety Agency

    // General
    ISO9001,          // Quality management systems
    Custom(String),   // Custom regulatory body
}

impl RegulatoryAgency {
    /// Get the full name of the agency
    pub fn full_name(&self) -> &str {
        match self {
            Self::FDA => "US Food and Drug Administration",
            Self::EMA => "European Medicines Agency",
            Self::ISO13485 => "ISO 13485 - Medical Devices Quality Management",
            Self::IEC62304 => "IEC 62304 - Medical Device Software Lifecycle",
            Self::SEC => "US Securities and Exchange Commission",
            Self::FINRA => "Financial Industry Regulatory Authority",
            Self::BaselIII => "Basel III - International Banking Regulations",
            Self::MIFID2 => "Markets in Financial Instruments Directive",
            Self::NHTSA => "National Highway Traffic Safety Administration",
            Self::ISO26262 => "ISO 26262 - Automotive Functional Safety",
            Self::UNECE => "UN Economic Commission for Europe",
            Self::NIST => "National Institute of Standards and Technology",
            Self::IEC62443 => "IEC 62443 - Industrial Cybersecurity",
            Self::ISO27001 => "ISO 27001 - Information Security Management",
            Self::EUGDPR => "EU General Data Protection Regulation",
            Self::EUAIACT => "EU AI Act",
            Self::OECD => "OECD AI Principles",
            Self::IEEE7000 => "IEEE 7000 - Ethics in Systems Design",
            Self::FAA => "Federal Aviation Administration",
            Self::EASA => "European Union Aviation Safety Agency",
            Self::ISO9001 => "ISO 9001 - Quality Management Systems",
            Self::Custom(name) => name,
        }
    }

    /// Get the website/documentation URL
    pub fn info_url(&self) -> &str {
        match self {
            Self::FDA => "https://www.fda.gov/medical-devices/software-medical-device-samd",
            Self::EMA => "https://www.ema.europa.eu",
            Self::ISO13485 => "https://www.iso.org/standard/59752.html",
            Self::IEC62304 => "https://www.iec.ch/standards",
            Self::SEC => "https://www.sec.gov",
            Self::FINRA => "https://www.finra.org",
            Self::BaselIII => "https://www.bis.org/bcbs/basel3.htm",
            Self::MIFID2 => "https://www.esma.europa.eu/policy-rules/mifid-ii-and-mifir",
            Self::NHTSA => "https://www.nhtsa.gov/technology-innovation/automated-vehicles-safety",
            Self::ISO26262 => "https://www.iso.org/standard/68383.html",
            Self::UNECE => "https://unece.org/transport/vehicle-regulations",
            Self::NIST => "https://www.nist.gov/artificial-intelligence",
            Self::IEC62443 => "https://www.isa.org/standards-and-publications/isa-standards/isa-iec-62443-series-of-standards",
            Self::ISO27001 => "https://www.iso.org/standard/27001",
            Self::EUGDPR => "https://gdpr.eu",
            Self::EUAIACT => "https://artificialintelligenceact.eu",
            Self::OECD => "https://oecd.ai/en/ai-principles",
            Self::IEEE7000 => "https://standards.ieee.org/ieee/7000/6781/",
            Self::FAA => "https://www.faa.gov",
            Self::EASA => "https://www.easa.europa.eu",
            Self::ISO9001 => "https://www.iso.org/iso-9001-quality-management.html",
            Self::Custom(_) => "N/A",
        }
    }
}
