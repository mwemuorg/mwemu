use crate::emu;

pub fn GetMemoryErrorHandlingCapabilities(emu: &mut emu::Emu) {
    let capabilities_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetMemoryErrorHandlingCapabilities ptr:0x{:x}",
        capabilities_ptr
    );
    if capabilities_ptr != 0 {
        emu.maps.write_dword(capabilities_ptr, 0);
    }
    emu.regs_mut().rax = 1;
}
