use anyhow::{Context, Error};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

pub const OP_LABEL: &str = "op";
pub const METRICS_NAME: &str = "myapp_request_duration_seconds";

/// Sets up the Prometheus metrics recorder.
pub fn setup_metrics_recorder() -> Result<PrometheusHandle, Error> {
    const BUCKETS: &[f64] = &[
        0.0001, 0.00015, 0.0002, 0.00025, 0.0003, 0.00035, 0.0004, 0.00045, 0.0005, 0.00055,
        0.0006, 0.00065, 0.0007, 0.00075, 0.0008, 0.00085, 0.0009, 0.00095, 0.001, 0.0015, 0.002,
        0.0025, 0.003, 0.0035, 0.004, 0.0045, 0.005, 0.0055, 0.006, 0.0065, 0.007, 0.0075, 0.008,
        0.0085, 0.009, 0.0095, 0.01, 0.015, 0.02, 0.025, 0.03, 0.035, 0.04, 0.045, 0.05, 0.055,
        0.06, 0.065, 0.07, 0.075, 0.08, 0.085, 0.09, 0.095, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4,
        0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8, 0.85, 0.9, 0.95, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5,
        4.0, 4.5, 5.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(Matcher::Full(METRICS_NAME.to_string()), BUCKETS)
        .context("failed to add buckets for metrics")?
        .install_recorder()
        .context("failed to install recorder for metrics")
}