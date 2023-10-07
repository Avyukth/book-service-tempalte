use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_tracing() {
    let subscriber = tracing_subscriber::fmt()
        // .compact()
        .json()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        // .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting initial tracing subscriber failed.");
}
