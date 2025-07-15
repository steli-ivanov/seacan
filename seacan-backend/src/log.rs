use std::sync::OnceLock;

use crate::{config, constants};

pub static FILE_STREAM: OnceLock<
    std::sync::Mutex<file_rotate::FileRotate<file_rotate::suffix::AppendTimestamp>>,
> = OnceLock::new();

pub fn initialize() {
    let config = config::object();

    if !std::fs::exists(constants::LOG_DIRECTORY).unwrap_or(false) {
        if let Err(e) = std::fs::create_dir(constants::LOG_DIRECTORY) {
            eprintln!(
                "failed to create the folder {}, error: {}",
                constants::LOG_DIRECTORY,
                e.to_string()
            );

            return;
        }
    }

    let file = file_rotate::FileRotate::new(
        constants::LOG_DIRECTORY.to_owned() + "/log",
        file_rotate::suffix::AppendTimestamp::default(file_rotate::suffix::FileLimit::MaxFiles(
            config.log_days_retention,
        )),
        file_rotate::ContentLimit::Time(file_rotate::TimeFrequency::Daily),
        file_rotate::compression::Compression::None,
        None,
    );

    if FILE_STREAM.set(std::sync::Mutex::new(file)).is_err() {
        panic!("log has been already initialized");
    }
}

#[macro_export]
macro_rules! outputln {
    () => {};
    ($($arg:tt)*) => {{
        use std::io::Write;
        use crate::log::FILE_STREAM;

        let timestamp = chrono::Local::now().timestamp_millis();
        let timestamp = chrono::DateTime::from_timestamp_millis(timestamp).unwrap();
        let timestamp = timestamp.to_rfc3339();
        let message = format!($($arg)*);
        let log_line = timestamp + " MSG :: " + &message + "\n";
        let log_line = log_line.as_bytes();

        let _ = std::io::stdout().write(log_line);

        if let Some(stream) = FILE_STREAM.get() {
            if let Ok(mut stream) = stream.lock() {
                let _ = stream.write(log_line);
            }
        }
    }};
}

#[macro_export]
macro_rules! errorln {
    () => {};
    ($($arg:tt)*) => {{
        use std::io::Write;
        use crate::log::FILE_STREAM;

        let timestamp = chrono::Local::now().timestamp_millis();
        let timestamp = chrono::DateTime::from_timestamp_millis(timestamp).unwrap();
        let timestamp = timestamp.to_rfc3339();
        let message = format!($($arg)*);
        let log_line = timestamp + " ERR :: " + &message + "\n";
        let log_line = log_line.as_bytes();

        let _ = std::io::stdout().write(log_line);

        if let Some(stream) = FILE_STREAM.get() {
            if let Ok(mut stream) = stream.lock() {
                let _ = stream.write(log_line);
            }
        }
    }};
}
