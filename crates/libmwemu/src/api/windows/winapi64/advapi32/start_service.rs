use crate::emu;

pub fn StartServiceA(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceA");
}

fn start_service(emu: &mut emu::Emu, api: &str) {
    let hsvc = emu.regs().rcx;
    let num_args = emu.regs().rdx;
    let args_ptr = emu.regs().r8;
    log_red!(emu, "advapi32!{api} svc:0x{:x} nargs:{}", hsvc, num_args);

    for _ in 0..3 {}
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn StartServiceW(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceW");
}
