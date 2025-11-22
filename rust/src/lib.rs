// AGI Alignment and Ethics Framework - Rust Implementation
// Copyright (c) 2025 AGI-AEF Contributors

//! # AGI-AEF-Rust
//!
//! A comprehensive Rust implementation of the AGI Alignment and Ethics Framework (AEF)
//! with enterprise-grade monitoring, security, and regulatory compliance features.
//!
//! ## Features
//!
//! - **256-level scoring system** (0-255) across 12 critical dimensions
//! - **48 granular tests** (4 per dimension) for comprehensive evaluation
//! - **REST API** with authentication, rate limiting, and CORS support
//! - **Real-time monitoring** using Prometheus and OpenTelemetry
//! - **Domain-specific ethics** frameworks (medical, financial, autonomous systems, etc.)
//! - **Regulatory compliance** integration (FDA, EMA, ISO standards)
//! - **Detailed explanations** for every score and recommendation
//! - **Audit trail** with complete traceability
//! - **CLI interface** with interactive reporting
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use agi_aef_rust::{AGIAEFAssessment, AssessmentConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = AssessmentConfig::default();
//!     let assessment = AGIAEFAssessment::new(config)?;
//!
//!     let result = assessment.run_comprehensive_assessment(
//!         "MyAGISystem",
//!         my_agi_implementation
//!     ).await?;
//!
//!     println!("Composite Score: {}/255", result.composite_score);
//!     println!("Classification: {}", result.level_classification);
//!
//!     Ok(())
//! }
//! ```

pub mod api;
pub mod assessment;
pub mod cli;
pub mod ethics;
pub mod models;
pub mod monitoring;
pub mod regulatory;
pub mod tests;

// Re-export main types
pub use assessment::{AGIAEFAssessment, AssessmentEngine};
pub use models::{
    AGIAEFResult, AssessmentConfig, DimensionScore, TestResult,
    AuditStatus, LevelClassification, DomainType
};
pub use ethics::EthicsFramework;
pub use regulatory::RegulatoryCompliance;

/// Version of the framework
pub const FRAMEWORK_VERSION: &str = "1.0.0";

/// Number of evaluation dimensions
pub const DIMENSION_COUNT: usize = 12;

/// Number of tests per dimension
pub const TESTS_PER_DIMENSION: usize = 4;

/// Total number of tests
pub const TOTAL_TESTS: usize = DIMENSION_COUNT * TESTS_PER_DIMENSION;

/// Maximum possible score
pub const MAX_SCORE: u8 = 255;
