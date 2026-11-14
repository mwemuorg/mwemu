use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreatePen(emu: &mut emu::Emu) {
    let style = arg(emu, 0);
    let width = arg(emu, 1);
    let color = arg(emu, 2);
    log_red!(
        emu,
        "gdi32!CreatePen style:{} width:{} color:0x{:06x}",
        style,
        width,
        color
    );
    let hndl = helper::handler_create("gdi32://CreatePen");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 3);
}
