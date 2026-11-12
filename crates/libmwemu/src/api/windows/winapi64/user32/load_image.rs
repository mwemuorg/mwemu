use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn LoadImageA(emu: &mut emu::Emu) {
    load_image(emu, "LoadImageA", false);
}

fn load_image(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hinstance = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let img_type = emu.regs().r8 as u32;
    let cx = emu.regs().r9 as i32;
    let cy = WinApiAbi::arg(emu, 4) as i32;
    let fu_load = WinApiAbi::arg(emu, 5) as u32;
    let name = if name_ptr > 0xffff {
        strings::read_text(emu, name_ptr, wide)
    } else {
        format!("#{}", name_ptr)
    };
    log_red!(
        emu,
        "user32!{api} hinst:0x{:x} name:'{}' type:{} cx:{} cy:{} flags:0x{:x}",
        hinstance,
        name,
        img_type,
        cx,
        cy,
        fu_load
    );
    let uri = format!("user32://{api}/{}", name);
    let hndl = helper::handler_create(&uri);
    emu.regs_mut().rax = hndl;
}

pub fn LoadImageW(emu: &mut emu::Emu) {
    load_image(emu, "LoadImageW", true);
}
