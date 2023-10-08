// use opentelemetry::global::shutdown_tracer_provider;
// use opentelemetry::sdk::Resource;
// use opentelemetry::trace::TraceError;
// use opentelemetry::trace::Tracer;
// use opentelemetry::{global, sdk::trace as sdktrace};
// use opentelemetry_http::HeaderExtractor;
// use opentelemetry_otlp::WithExportConfig;

// pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
//     opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_env())
//         .with_trace_config(sdktrace::config().with_resource(Resource::default()))
//         .install_batch(opentelemetry::runtime::Tokio)
// }
