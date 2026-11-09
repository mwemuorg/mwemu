use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CreateHatchBrush(emu: &mut emu::Emu) {
    let style = arg(emu, 0);
    let color = arg(emu, 1);
    log_red!(
        emu,
        "gdi32!CreateHatchBrush style:{} color:0x{:06x}",
        style,
        color
    );
    let hndl = helper::handler_create("gdi32://CreateHatchBrush");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 2);
}
