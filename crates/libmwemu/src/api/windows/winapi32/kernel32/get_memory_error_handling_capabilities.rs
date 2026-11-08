use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetMemoryErrorHandlingCapabilities(emu: &mut emu::Emu) {
    let capabilities_ptr = arg(emu, 0) as u64;
    log_red!(
        emu,
        "kernel32!GetMemoryErrorHandlingCapabilities ptr:0x{:x}",
        capabilities_ptr
    );
    if capabilities_ptr != 0 {
        emu.maps.write_dword(capabilities_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
