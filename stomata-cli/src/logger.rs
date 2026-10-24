use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::{Mutex, OnceLock},
};

static LOG: OnceLock<Mutex<File>> = OnceLock::new();

pub fn init_log(tty_path: &str) {
    if !cfg!(debug_assertions) {
        return;
    }
    let f = OpenOptions::new().write(true).open(tty_path).unwrap();
    LOG.set(Mutex::new(f)).ok();
}

pub fn write_log(msg: &str) {
    if let Some(m) = LOG.get() {
        if let Ok(mut f) = m.lock() {
            let _ = writeln!(f, "{}", msg);
        }
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::logger::write_log(&format!($($arg)*))
    };
}
