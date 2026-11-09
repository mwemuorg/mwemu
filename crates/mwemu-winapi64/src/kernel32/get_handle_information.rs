use crate::emu;

pub fn GetHandleInformation(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let flags_ptr = emu.regs().rdx;
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
}
