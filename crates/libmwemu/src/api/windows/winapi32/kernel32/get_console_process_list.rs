use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetConsoleProcessList(emu: &mut emu::Emu) {
    let list_ptr = arg(emu, 0) as u64;
    let count = arg(emu, 1);
    log_red!(
        emu,
        "kernel32!GetConsoleProcessList ptr:0x{:x} count:{}",
        list_ptr,
        count
    );
    if list_ptr != 0 && count >= 1 {
        emu.maps.write_dword(list_ptr, 0x123);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
