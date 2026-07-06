use crate::emu;

pub fn TlsFree(emu: &mut emu::Emu) {
    let idx = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!TlsFree cannot read idx");

    log_red!(emu, "kernel32!TlsFree idx: {}", idx);

    emu.stack_pop32(false);
    emu.regs_mut().set_eax(1);
}
