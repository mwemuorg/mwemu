use crate::emu;
use crate::winapi::helper;

pub fn LoadImageW(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let img_type = emu.regs().r8 as u32;
    let cx = emu.regs().r9 as i32;
    let cy = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let fu_load = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    let name = if name_ptr > 0xffff {
        emu.maps.read_wide_string(name_ptr)
    } else {
        format!("#{}", name_ptr)
    };
    log_red!(
        emu,
        "user32!LoadImageW hinst:0x{:x} name:'{}' type:{} cx:{} cy:{} flags:0x{:x}",
        hinstance,
        name,
        img_type,
        cx,
        cy,
        fu_load
    );
    let uri = format!("user32://LoadImageW/{}", name);
    let hndl = helper::handler_create(&uri);
    emu.regs_mut().rax = hndl;
}
