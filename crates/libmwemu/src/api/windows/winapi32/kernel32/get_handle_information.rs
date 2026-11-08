use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetHandleInformation(emu: &mut emu::Emu) {
    let handle = arg(emu, 0) as u64;
    let flags_ptr = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!GetHandleInformation h:0x{:x} flags_ptr:0x{:x}",
        handle,
        flags_ptr
    );
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
