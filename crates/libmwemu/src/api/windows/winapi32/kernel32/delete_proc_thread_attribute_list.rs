use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
pub fn DeleteProcThreadAttributeList(emu: &mut emu::Emu) {
    let attr_list = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!DeleteProcThreadAttributeList list:0x{:x}",
        attr_list
    );
    let h = arg(emu, 0) as u64;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    stdcall_ret(emu, 1);
}
