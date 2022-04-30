
pub fn enable_logging() {
    let conf = config::default();
    let log_level = conf.get_log_level();
    let filter = match log_level {
        "TRACE" => log::LevelFilter::Trace,
        "DEBUG" => log::LevelFilter::Debug,
        "INFO" => log::LevelFilter::Info,
        "WARN" => log::LevelFilter::Warn,
        "ERROR" => log::LevelFilter::Error,
        _ => log::LevelFilter::Debug,
    };
    simple_logger::SimpleLogger::new()
        .with_level(filter)
        .with_module_level("my_crate", filter)
        .init()
        .unwrap();
}