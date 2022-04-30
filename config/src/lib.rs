use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::Value;


const PORT: &str = "9000";
const LOG_LEVEL: &str = "DEBUG";

const CONFIG_PATH: &str = "conf.toml";

static mut CONFIG:Option<Config> = None;

#[derive(Debug)]
pub struct Config {
    port: String,
    log_level: String
}

impl Config {
    pub fn get_port(&self) -> &str {
        self.port.as_str()
    }
    pub fn get_log_level(&self) -> &str {
        self.log_level.as_str()
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
                let pv = Value::from(PORT);
                let port = v.get("port").unwrap_or(&pv);
                let lv = Value::from(LOG_LEVEL);
                let log_level = v.get("log_level").unwrap_or(&lv);
                let config = Config {
                    port: port.as_str().unwrap_or(PORT).to_string(),
                    log_level: log_level.as_str().unwrap_or(LOG_LEVEL).to_ascii_uppercase()
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
                log_level: LOG_LEVEL.to_ascii_uppercase()
            };
            CONFIG = Some(config)
        }

        CONFIG.as_ref().unwrap()
    }

}