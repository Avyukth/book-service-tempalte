use tracing_appender::rolling;
use tracing_subscriber::fmt::{format::FmtSpan, writer::MakeWriterExt};
pub fn init_tracing() {
    let file_appender = tracing_appender::rolling::minutely("./logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // let stdout = std::io::stdout;

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    let subscriber = tracing_subscriber::fmt()
        // .compact()
        .json()
        .with_writer(stdout.and(non_blocking))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_ansi(true)
        // .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting initial tracing subscriber failed.");
}
