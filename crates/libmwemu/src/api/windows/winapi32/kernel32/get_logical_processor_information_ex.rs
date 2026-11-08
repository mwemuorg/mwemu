use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetLogicalProcessorInformationEx(emu: &mut emu::Emu) {
    let relation = arg(emu, 0);
    let buffer_ptr = arg(emu, 1) as u64;
    let length_ptr = arg(emu, 2) as u64;
    log_red!(
        emu,
        "kernel32!GetLogicalProcessorInformationEx rel:{} buf:0x{:x} len:0x{:x}",
        relation,
        buffer_ptr,
        length_ptr
    );
    if length_ptr != 0 {
        emu.maps.write_dword(length_ptr, 112);
    }
    emu.regs_mut().rax = 0;
    emu.set_last_error(122);
    stdcall_ret(emu, 3);
}
