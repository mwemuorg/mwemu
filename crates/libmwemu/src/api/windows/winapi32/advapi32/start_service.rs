use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn StartServiceA(emu: &mut emu::Emu) {
    start_service(emu, "StartServiceA");
}

fn start_service(emu: &mut emu::Emu, api: &str) {
    let hsvc = WinApiAbi::arg(emu, 0) as u32;
    let num_args = WinApiAbi::arg(emu, 1) as u32;
    let args_ptr = WinApiAbi::arg(emu, 2) as u32;
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
