use crate::emu;

pub fn FreeLibrary(emu: &mut emu::Emu) {
    let hmod = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FreeLibrary: error reading param") as u64;

    log_red!(emu, "kernel32!FreeLibrary   {:x}", hmod);

    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
}
