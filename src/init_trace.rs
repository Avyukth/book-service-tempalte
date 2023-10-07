use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
// use tracing_subscriber::;

pub fn init_tracing() {
    // File appender for logging to files.
    let file_appender = tracing_appender::rolling::minutely("./logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // Configuration for logging to stdout in a pretty format.
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty() // Use pretty format for stdout.
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_ansi(true)
        .with_writer(std::io::stdout);

    // Configuration for logging to file in JSON format.
    let file_layer = tracing_subscriber::fmt::layer()
        .json() // Use JSON format for file.
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_writer(non_blocking);

    // Combine the layers with an environment filter.
    let subscriber = tracing_subscriber::Registry::default()
        // .with(EnvFilter::from_default_env())  // Use TRACE_LEVEL environment variable for dynamic log level configuration.
        .with(stdout_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting initial tracing subscriber failed.");
}
