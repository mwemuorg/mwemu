use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCommModemStatus(emu: &mut emu::Emu) {
    let h_file = arg(emu, 0) as u64;
    let status_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetCommModemStatus h:0x{:x} ptr:0x{:x}",
        h_file,
        status_ptr
    );
    if status_ptr != 0 {
        emu.maps.write_dword(status_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
