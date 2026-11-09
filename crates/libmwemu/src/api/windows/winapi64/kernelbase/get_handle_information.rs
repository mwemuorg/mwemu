use crate::emu;

pub fn GetHandleInformation(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0); // no flags set
    }
    log_red!(
        emu,
        "kernelbase!GetHandleInformation h:0x{:x} flags_ptr:0x{:x}",
        handle,
        flags_ptr
    );
    emu.regs_mut().rax = 1; // TRUE
}
