use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    pub static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    pub static ref LAST_ERROR: Mutex<u32> = Mutex::new(0);
}

pub fn set_last_error(err_code: u32) {
    let mut guard = LAST_ERROR.lock().unwrap();
    *guard = err_code;
}
