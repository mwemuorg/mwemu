use crate::emu;

pub fn CheckForReadOnlyResource(emu: &mut emu::Emu) {
    let h_module = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CheckForReadOnlyResource hModule:0x{:x}",
        h_module
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
