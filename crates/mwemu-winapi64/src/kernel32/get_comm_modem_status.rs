use crate::emu;

pub fn GetCommModemStatus(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    let status_ptr = emu.regs().rdx;
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
}
