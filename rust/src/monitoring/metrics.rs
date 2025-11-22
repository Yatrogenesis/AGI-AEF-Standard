use prometheus::{
    Counter, Gauge, Histogram, HistogramOpts, HistogramVec, IntCounter, IntCounterVec,
    IntGauge, IntGaugeVec, Opts, Registry,
};
use std::sync::Arc;

/// Collector for all assessment metrics
pub struct MetricsCollector {
    // Assessment counters
    pub assessments_total: IntCounter,
    pub assessments_by_status: IntCounterVec,
    pub assessments_by_domain: IntCounterVec,

    // Score gauges
    pub composite_score: Gauge,
    pub dimension_scores: IntGaugeVec,

    // Performance metrics
    pub assessment_duration: Histogram,
    pub test_duration: HistogramVec,

    // Safety metrics
    pub safety_violations: IntCounter,
    pub critical_issues: IntCounter,

    // Regulatory metrics
    pub compliance_checks: IntCounterVec,
}

impl MetricsCollector {
    pub fn new(registry: Arc<Registry>) -> Result<Self, prometheus::Error> {
        let assessments_total = IntCounter::new(
            "agi_aef_assessments_total",
            "Total number of assessments performed",
        )?;
        registry.register(Box::new(assessments_total.clone()))?;

        let assessments_by_status = IntCounterVec::new(
            Opts::new(
                "agi_aef_assessments_by_status",
                "Assessments grouped by status",
            ),
            &["status"],
        )?;
        registry.register(Box::new(assessments_by_status.clone()))?;

        let assessments_by_domain = IntCounterVec::new(
            Opts::new(
                "agi_aef_assessments_by_domain",
                "Assessments grouped by domain",
            ),
            &["domain"],
        )?;
        registry.register(Box::new(assessments_by_domain.clone()))?;

        let composite_score = Gauge::new(
            "agi_aef_composite_score",
            "Current composite score (0-255)",
        )?;
        registry.register(Box::new(composite_score.clone()))?;

        let dimension_scores = IntGaugeVec::new(
            Opts::new("agi_aef_dimension_scores", "Scores per dimension"),
            &["dimension"],
        )?;
        registry.register(Box::new(dimension_scores.clone()))?;

        let assessment_duration = Histogram::with_opts(HistogramOpts::new(
            "agi_aef_assessment_duration_seconds",
            "Assessment execution time",
        ))?;
        registry.register(Box::new(assessment_duration.clone()))?;

        let test_duration = HistogramVec::new(
            HistogramOpts::new(
                "agi_aef_test_duration_seconds",
                "Individual test execution time",
            ),
            &["dimension", "test"],
        )?;
        registry.register(Box::new(test_duration.clone()))?;

        let safety_violations = IntCounter::new(
            "agi_aef_safety_violations_total",
            "Total safety violations detected",
        )?;
        registry.register(Box::new(safety_violations.clone()))?;

        let critical_issues = IntCounter::new(
            "agi_aef_critical_issues_total",
            "Total critical issues detected",
        )?;
        registry.register(Box::new(critical_issues.clone()))?;

        let compliance_checks = IntCounterVec::new(
            Opts::new(
                "agi_aef_compliance_checks",
                "Regulatory compliance checks",
            ),
            &["agency", "result"],
        )?;
        registry.register(Box::new(compliance_checks.clone()))?;

        Ok(Self {
            assessments_total,
            assessments_by_status,
            assessments_by_domain,
            composite_score,
            dimension_scores,
            assessment_duration,
            test_duration,
            safety_violations,
            critical_issues,
            compliance_checks,
        })
    }

    /// Record a completed assessment
    pub fn record_assessment(
        &self,
        result: &crate::models::AGIAEFResult,
    ) {
        self.assessments_total.inc();
        self.assessments_by_status
            .with_label_values(&[&format!("{:?}", result.audit_status)])
            .inc();
        self.assessments_by_domain
            .with_label_values(&[&format!("{:?}", result.domain)])
            .inc();

        self.composite_score.set(result.composite_score as f64);

        for (dim_name, score) in &result.dimension_scores {
            self.dimension_scores
                .with_label_values(&[dim_name])
                .set(*score as i64);
        }

        self.assessment_duration
            .observe(result.metadata.total_execution_time_ms as f64 / 1000.0);

        if result.metadata.errors.len() > 0 {
            self.critical_issues.inc_by(result.metadata.errors.len() as u64);
        }
    }
}
