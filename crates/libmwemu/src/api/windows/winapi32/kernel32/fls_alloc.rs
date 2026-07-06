use crate::emu;

pub fn FlsAlloc(emu: &mut emu::Emu) {
    let callback = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!FlsAlloc cannot read callback");

    log_red!(emu, "kernel32!FlsAlloc callback: 0x{:x}", callback);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
