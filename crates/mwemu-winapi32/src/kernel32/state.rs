use crate::emu;

pub fn set_last_error(emu: &mut emu::Emu, err_code: u32) {
    emu.set_last_error(err_code as u64);
}
