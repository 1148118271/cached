use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::Value;

/// 默认端口
const PORT: &str = "9200";
/// 默认日志级别
const LOG_LEVEL: &str = "DEBUG";
/// 默认日志路径
const LOG_FILE_PATH: &str = "cached.log";

const CONFIG_PATH: &str = "conf.toml";

static mut CONFIG:Option<Config> = None;

#[derive(Debug)]
pub struct Config {
    port: String,
    log_level: String,
    log_file_path: String
}

impl Config {
    pub fn get_port(&self) -> &str {
        self.port.as_str()
    }
    pub fn get_log_level(&self) -> &str {
        self.log_level.as_str()
    }
    pub fn get_log_file_path(&self) -> &str {
        self.log_file_path.as_str()
    }
}

pub fn default() -> &'static Config {

    unsafe {
        if CONFIG.is_some() {
            CONFIG.as_ref().unwrap();
        }
    }

    let mut path_buf = PathBuf::from(CONFIG_PATH);
    let args = std::env::args_os();
    if args.len() > 1 {
        let vec = args.collect::<Vec<OsString>>();
        path_buf = PathBuf::from(&vec[1]);
    } else {
        if let Ok(v) = std::env::current_dir() {
            path_buf = v.join(CONFIG_PATH);
        }
    }

    if let Ok(mut f) = File::open(path_buf) {
        let mut toml_str = String::new();
        if let Ok(_) = f.read_to_string(&mut toml_str) {
            if let Ok(v) = toml::from_str::<Value>(&toml_str) {
                // 端口
                let pv = Value::from(PORT);
                let port = v.get("port").unwrap_or(&pv);
                // 日志级别
                let lv = Value::from(LOG_LEVEL);
                let log_level = v.get("log_level").unwrap_or(&lv);
                // 日志路径
                let lfp = Value::from(LOG_FILE_PATH);
                let log_file_path = v.get("log_file_path").unwrap_or(&lfp);

                let config = Config {
                    port: port.as_str().unwrap_or(PORT).to_string(),
                    log_level: log_level.as_str().unwrap_or(LOG_LEVEL).to_ascii_uppercase(),
                    log_file_path: log_file_path.as_str().unwrap_or(LOG_FILE_PATH).to_string()
                };
                unsafe {
                    CONFIG = Some(config)
                }
            }
        }
    }
    unsafe {
        if CONFIG.is_none() {
            let config = Config {
                port: PORT.to_string(),
                log_level: LOG_LEVEL.to_ascii_uppercase(),
                log_file_path: LOG_FILE_PATH.to_string()
            };
            CONFIG = Some(config)
        }

        CONFIG.as_ref().unwrap()
    }

}