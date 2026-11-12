use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CreateFontA(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontA", false);
}

fn create_font(emu: &mut emu::Emu, api: &str, wide: bool) {
    let height = emu.regs().rcx as i32;
    let width = emu.regs().rdx as i32;
    let escapement = emu.regs().r8 as i32;
    let orientation = emu.regs().r9 as i32;
    let weight = WinApiAbi::arg(emu, 4) as i32;
    let italic = WinApiAbi::arg(emu, 5) as u32;
    let underline = WinApiAbi::arg(emu, 6) as u32;
    let strikeout = WinApiAbi::arg(emu, 7) as u32;
    let charset = WinApiAbi::arg(emu, 8) as u32;
    let out_precision = WinApiAbi::arg(emu, 9) as u32;
    let clip_precision = WinApiAbi::arg(emu, 10) as u32;
    let quality = WinApiAbi::arg(emu, 11) as u32;
    let pitch_and_family = WinApiAbi::arg(emu, 12) as u32;
    let face_ptr = WinApiAbi::arg(emu, 13);
    let face_name = if face_ptr != 0 {
        strings::read_text(emu, face_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!{api} h:{} w:{} weight:{} italic:{} charset:{} quality:{} face:'{}'",
        height,
        width,
        weight,
        italic,
        charset,
        quality,
        face_name
    );
    let _ = (
        escapement,
        orientation,
        underline,
        strikeout,
        out_precision,
        clip_precision,
        pitch_and_family,
    );
    emu.regs_mut().rax = helper::handler_create("gdi32://font");
}

pub fn CreateFontW(emu: &mut emu::Emu) {
    create_font(emu, "CreateFontW", true);
}
