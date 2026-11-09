use crate::emu;

pub fn QueryIoRingCapabilities(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!QueryIoRingCapabilities param1:0x{:x}",
        param1
    );
    if emu.regs().rcx != 0 && emu.maps.is_mapped(emu.regs().rcx) {
        emu.maps.write_dword(emu.regs().rcx, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
