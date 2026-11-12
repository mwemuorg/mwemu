use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateDIBSection(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0) as u32;
    let arg2 = WinApiAbi::arg(emu, 1) as u32;
    let arg3 = WinApiAbi::arg(emu, 2) as u32;
    let arg4 = WinApiAbi::arg(emu, 3) as u32;
    log_red!(
        emu,
        "gdi32!CreateDIBSection arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    let hndl = helper::handler_create("gdi32://CreateDIBSection");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
}
