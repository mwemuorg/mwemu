use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetNumberOfConsoleMouseButtons(emu: &mut emu::Emu) {
    let buttons_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetNumberOfConsoleMouseButtons ptr:0x{:x}",
        buttons_ptr
    );
    if buttons_ptr != 0 {
        emu.maps.write_dword(buttons_ptr, 2);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
