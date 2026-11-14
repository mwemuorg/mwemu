use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn BasepQueryAppCompat(emu: &mut emu::Emu) {
    let _a1 = WinApiAbi::arg(emu, 0) as u32;
    let _a2 = WinApiAbi::arg(emu, 1) as u32;
    let _a3 = WinApiAbi::arg(emu, 2) as u32;
    let _a4 = WinApiAbi::arg(emu, 3) as u32;
    let _a5 = WinApiAbi::arg(emu, 4) as u32;
    let _a6 = WinApiAbi::arg(emu, 5) as u32;
    let _a7 = WinApiAbi::arg(emu, 6) as u32;
    let _a8 = WinApiAbi::arg(emu, 7) as u32;
    let _a9 = WinApiAbi::arg(emu, 8) as u32;
    let _a10 = WinApiAbi::arg(emu, 9) as u32;
    let _a11 = WinApiAbi::arg(emu, 10) as u32;
    let _a12 = WinApiAbi::arg(emu, 11) as u32;
    let _a13 = WinApiAbi::arg(emu, 12) as u32;
    let _a14 = WinApiAbi::arg(emu, 13) as u32;
    let _a15 = WinApiAbi::arg(emu, 14) as u32;
    let _a16 = WinApiAbi::arg(emu, 15) as u32;
    let _a17 = WinApiAbi::arg(emu, 16) as u32;
    let _a18 = WinApiAbi::arg(emu, 17) as u32;

    log_red!(emu, "kernel32!BasepQueryAppCompat");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..18 {
        emu.stack_pop32(false);
    }
}
