use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Add span events
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        // Display debug-level info
        .with_max_level(tracing_subscriber::filter::LevelFilter::DEBUG)
        // Build the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
