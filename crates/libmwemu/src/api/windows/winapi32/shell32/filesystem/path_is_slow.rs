use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn PathIsSlowA(emu: &mut emu::Emu) {
    path_is_slow(emu, "PathIsSlowA");
}

fn path_is_slow(emu: &mut emu::Emu, api: &str) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    let h = WinApiAbi::arg(emu, 0);
    emu.regs_mut().rax = 1;

    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}

pub fn PathIsSlowW(emu: &mut emu::Emu) {
    path_is_slow(emu, "PathIsSlowW");
}
