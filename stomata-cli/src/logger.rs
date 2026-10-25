use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

static LOG: OnceLock<Mutex<File>> = OnceLock::new();

#[derive(Debug, Clone, Copy)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    fn as_str(&self) -> &'static str {
        match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }
}

fn timestamp() -> String {
    chrono::Local::now().format("%H:%M:%S%.3f").to_string()
}

pub fn init_log(tty_path: &str) {
    if !cfg!(debug_assertions) {
        return;
    }
    let f = OpenOptions::new().write(true).open(tty_path).unwrap();
    LOG.set(Mutex::new(f)).ok();
}

pub fn write_log(level: Level, msg: &str) {
    if let Some(m) = LOG.get() {
        if let Ok(mut f) = m.lock() {
            let _ = writeln!(f, "{} [{:<5}] {}", timestamp(), level.as_str(), msg);
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::write_log($crate::logger::Level::Debug, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::write_log($crate::logger::Level::Info, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::write_log($crate::logger::Level::Warn, &format!($($arg)*))
    };
}

#[macro_export]
macro_rules! Error {
    ($($arg:tt)*) => {
        $crate::logger::write_log($crate::logger::Level::Error, &format!($($arg)*))
    };
}
