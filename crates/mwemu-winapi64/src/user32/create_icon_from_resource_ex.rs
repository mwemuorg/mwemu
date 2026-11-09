use crate::emu;
use crate::winapi::helper;

pub fn CreateIconFromResourceEx(emu: &mut emu::Emu) {
    let presbits = emu.regs().rcx;
    let dw_res_size = emu.regs().rdx as u32;
    let is_icon = emu.regs().r8 as u32;
    let dw_ver = emu.regs().r9 as u32;
    let cx = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let cy = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
    let flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
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
