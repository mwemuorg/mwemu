use crate::emu;

pub fn FreeLibrary(emu: &mut emu::Emu) {
    let hmodule = emu.regs().rcx;

    log_red!(emu, "kernel32!FreeLibrary hModule: 0x{:x}", hmodule);

    emu.regs_mut().rax = 1;
}
