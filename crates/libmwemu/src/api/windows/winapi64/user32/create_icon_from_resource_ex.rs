use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateIconFromResourceEx(emu: &mut emu::Emu) {
    let presbits = emu.regs().rcx;
    let dw_res_size = emu.regs().rdx as u32;
    let is_icon = emu.regs().r8 as u32;
    let dw_ver = emu.regs().r9 as u32;
    let cx = WinApiAbi::arg(emu, 4) as i32;
    let cy = WinApiAbi::arg(emu, 5) as i32;
    let flags = WinApiAbi::arg(emu, 6) as u32;
    log_red!(
        emu,
        "user32!CreateIconFromResourceEx data:0x{:x} size:{} icon:{} cx:{} cy:{} flags:0x{:x}",
        presbits,
        dw_res_size,
        is_icon,
        cx,
        cy,
        flags
    );
    let hndl = helper::handler_create("user32://CreateIconFromResourceEx");
    emu.regs_mut().rax = hndl;
}
