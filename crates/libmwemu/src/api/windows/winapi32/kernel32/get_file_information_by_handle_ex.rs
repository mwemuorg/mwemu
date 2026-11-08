use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetFileInformationByHandleEx(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let info_class = arg(emu, 1);
    let info_ptr = arg(emu, 2) as u64;
    let info_len = arg(emu, 3) as u64;

    log_red!(
        emu,
        "kernel32!GetFileInformationByHandleEx h:0x{:x} class:{} buf:0x{:x} len:{}",
        h_file,
        info_class,
        info_ptr,
        info_len
    );

    if !helper::handler_exist(h_file) {
        emu.set_last_error(6); // ERROR_INVALID_HANDLE
        emu.regs_mut().rax = 0;
        return;
    }

    if info_ptr != 0 && info_len >= 8 {
        emu.maps.memset(info_ptr, 0, info_len as usize);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
