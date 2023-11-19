use opentelemetry::{global, trace, KeyValue};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::runtime::TokioCurrentThread;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::{propagation::TraceContextPropagator, trace::config};
use std::io::stdout;
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

pub fn init_telemetry() -> anyhow::Result<()> {
    LogTracer::init()?;
    let app_name = env!("CARGO_PKG_NAME");

    global::set_text_map_propagator(TraceContextPropagator::new());

    let otlp_exporter = opentelemetry_otlp::new_exporter().tonic();

    let trace_config =
        config().with_resource(Resource::new(vec![KeyValue::new("service.name", app_name)]));

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(otlp_exporter)
        .with_trace_config(trace_config)
        .install_simple()?;

    let stdout_log = tracing_subscriber::fmt::layer()
        .pretty()
        .with_writer(stdout);

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(stdout_log)
        .with(telemetry);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
