
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

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(filter)
        .chain(std::io::stdout())
        .chain(fern::log_file(conf.get_log_file_path()).expect("日志文件输出失败"))
        .apply().expect("日志功能启用失败");
}