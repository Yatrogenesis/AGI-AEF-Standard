/// Monitoring and Observability Module
///
/// Provides comprehensive monitoring with:
/// - Prometheus metrics
/// - OpenTelemetry tracing
/// - Per-dimension visibility
/// - Real-time dashboards

use prometheus::{
    Counter, Gauge, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry,
};
use std::sync::Arc;

pub mod metrics;
pub mod telemetry;
pub mod dashboard;

pub use metrics::MetricsCollector;
pub use telemetry::TelemetryExporter;

/// Main monitoring system
pub struct MonitoringSystem {
    registry: Arc<Registry>,
    metrics: MetricsCollector,
}

impl MonitoringSystem {
    pub fn new() -> Result<Self, prometheus::Error> {
        let registry = Arc::new(Registry::new());
        let metrics = MetricsCollector::new(registry.clone())?;

        Ok(Self { registry, metrics })
    }

    pub fn metrics(&self) -> &MetricsCollector {
        &self.metrics
    }

    pub fn registry(&self) -> Arc<Registry> {
        self.registry.clone()
    }

    /// Export metrics in Prometheus format
    pub fn export_metrics(&self) -> Result<String, prometheus::Error> {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(String::from_utf8(buffer).unwrap())
    }
}

impl Default for MonitoringSystem {
    fn default() -> Self {
        Self::new().expect("Failed to create monitoring system")
    }
}
