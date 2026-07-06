use crate::emu;
use crate::windows::constants;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref COUNT_READ: Mutex<u32> = Mutex::new(0);
    pub static ref COUNT_WRITE: Mutex<u32> = Mutex::new(0);
    pub static ref LAST_ERROR: Mutex<u64> = Mutex::new(0);
}

pub fn clear_last_error(_emu: &mut emu::Emu) {
    let mut err = LAST_ERROR.lock().unwrap();
    *err = constants::ERROR_SUCCESS;
}

pub fn set_last_error(err_code: u64) {
    let mut guard = LAST_ERROR.lock().unwrap();
    *guard = err_code;
}
