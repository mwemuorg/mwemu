use super::finish;
use super::key_path;
use crate::emu;
use crate::windows::constants::ERROR_SUCCESS;

pub(super) fn flush_key(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let err = key_path(hkey).err().unwrap_or(ERROR_SUCCESS);
    finish(emu, "RegFlushKey", &format!("0x{:x}", hkey), err);
}
