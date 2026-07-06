use crate::emu;

pub fn RaiseException(emu: &mut emu::Emu) {
    let code = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!RaiseException cannot read code");
    let flags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!RaiseException cannot read flags");
    let num_args = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!RaiseException cannot read num_args");
    let args = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("kernel32!RaiseException cannot read args");

    log_red!(emu, "kernel32!RaiseException {} {}", code, flags);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 0;
    //emu.stop();
}
