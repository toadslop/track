//! The telemtry module is responsible for collecting runtime data for the application and
//! transmitting it where it's needed. It routes all information to stdout by default,
//! but can also be configured by environment variable to transmit telemetry data to
//! an Open Telemetry compatible server.

use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::config};
use std::io::stdout;
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

static TRACK_TELEMETRY_CONNECTION_STRING: &str = "TRACK__TELEMETRY_CONNECTION_STRING";

/// Initialize telemetry for the application. If the environment variable `TRACK__TELEMETRY_CONNECTION_STRING`
/// is provided, the application will attempt to connect to a Jaeger instance. If the connection fails,
/// the application will fail to start. If the environment variable is not set, then the application will
/// simply output telemetr to the application logs.
pub fn init() -> anyhow::Result<()> {
    LogTracer::init()?;
    let app_name = env!("CARGO_PKG_NAME");

    global::set_text_map_propagator(TraceContextPropagator::new());

    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(stdout);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    let subscriber = Registry::default().with(env_filter).with(stdout_log);

    if let Ok(conn_str) = std::env::var(TRACK_TELEMETRY_CONNECTION_STRING) {
        let otlp_exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(conn_str);

        let trace_config =
            config().with_resource(Resource::new(vec![KeyValue::new("service.name", app_name)]));

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(otlp_exporter)
            .with_trace_config(trace_config)
            .install_simple()?;
        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        let subscriber = subscriber.with(telemetry);

        tracing::subscriber::set_global_default(subscriber)?;
    } else {
        tracing::subscriber::set_global_default(subscriber)?;
    }

    Ok(())
}
