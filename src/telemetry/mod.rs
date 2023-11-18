use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::runtime::TokioCurrentThread;
use std::io::stdout;
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

pub fn init_telemetry() -> anyhow::Result<()> {
    LogTracer::init()?;
    let app_name = env!("CARGO_PKG_NAME");

    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(app_name)
        .install_batch(TokioCurrentThread)?;

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
