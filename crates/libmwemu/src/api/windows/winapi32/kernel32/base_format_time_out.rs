use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn BaseFormatTimeOut(emu: &mut emu::Emu) {
    let param1 = arg(emu, 0) as u64;
    let param2 = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!BaseFormatTimeOut param1:0x{:x} param2:0x{:x}",
        param1,
        param2
    );
    let src = arg(emu, 0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
