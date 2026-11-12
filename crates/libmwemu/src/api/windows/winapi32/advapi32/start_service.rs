use crate::emu;

pub fn StartServiceA(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceA");
}

fn start_service(emu: &mut emu::Emu, api: &str) {
    let hsvc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let num_args = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let args_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    log_red!(emu, "advapi32!{api} svc:0x{:x} nargs:{}", hsvc, num_args);
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn StartServiceW(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceW");
}
