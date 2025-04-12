use std::fs::OpenOptions;
use std::io::Write;

use crate::LOG_FILE;

pub fn initialize_logger(log_path: &str) -> Result<(), std::io::Error> {
    *LOG_FILE.lock().expect("Failed to lock log file") = Some(OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?);

    Ok(())
}

pub fn log_message(level: &str, message: &str) {
    let mut log_file_guard = LOG_FILE.lock().expect("Failed to lock log file");
    if let Some(file) = log_file_guard.as_mut() {
        let now = chrono::Local::now();
        let log_line = format!(
            "[{}] [{}] {}\n",
            now.format("%Y-%m-%d %H:%M:%S"),
            level,
            message
        );

        if let Err(e) = file.write_all(log_line.as_bytes()) {
            eprintln!("Error writing to log file: {e}");
        }
    } else {
        eprintln!(
            "Logger not initialized.  Message: {message} (Level: {level})"
        );
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log_message("INFO", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log_message("ERROR", &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) { // Conditional compilation for debug builds
            log_message("DEBUG", &format!($($arg)*));
        }
    };
}
