use crate::emu;

pub fn CheckForReadOnlyResource(emu: &mut emu::Emu) {
    let h_module = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CheckForReadOnlyResource hModule:0x{:x}",
        h_module
    );
    let _ = emu.maps.is_mapped(emu.regs().rcx);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
