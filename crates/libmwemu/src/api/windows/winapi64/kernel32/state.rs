use crate::emu;

pub fn clear_last_error(emu: &mut emu::Emu) {
    emu.clear_last_error();
}

pub fn set_last_error(emu: &mut emu::Emu, err_code: u64) {
    emu.set_last_error(err_code);
}
