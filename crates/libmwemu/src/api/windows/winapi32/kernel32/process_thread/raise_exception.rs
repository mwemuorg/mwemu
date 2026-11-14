use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn RaiseException(emu: &mut emu::Emu) {
    let code = WinApiAbi::arg(emu, 0) as u32;
    let flags = WinApiAbi::arg(emu, 1) as u32;
    let num_args = WinApiAbi::arg(emu, 2) as u32;
    let args = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!RaiseException {} {}", code, flags);

    for _ in 0..4 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 0;
    //emu.stop();
}
