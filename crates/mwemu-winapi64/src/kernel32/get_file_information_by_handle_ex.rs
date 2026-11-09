use crate::emu;
use crate::winapi::helper;

pub fn GetFileInformationByHandleEx(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let info_class = emu.regs().rdx as u32;
    let info_ptr = emu.regs().r8;
    let info_len = emu.regs().r9;

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
}
