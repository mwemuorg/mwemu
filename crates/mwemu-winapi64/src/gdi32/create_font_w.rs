use crate::emu;
use crate::winapi::helper;

pub fn CreateFontW(emu: &mut emu::Emu) {
    let height = emu.regs().rcx as i32;
    let width = emu.regs().rdx as i32;
    let escapement = emu.regs().r8 as i32;
    let orientation = emu.regs().r9 as i32;
    let weight = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let italic = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    let underline = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let strikeout = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as u32;
    let charset = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as u32;
    let out_precision = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0) as u32;
    let clip_precision = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0) as u32;
    let quality = emu.maps.read_qword(emu.regs().rsp + 0x60).unwrap_or(0) as u32;
    let pitch_and_family = emu.maps.read_qword(emu.regs().rsp + 0x68).unwrap_or(0) as u32;
    let face_ptr = emu.maps.read_qword(emu.regs().rsp + 0x70).unwrap_or(0);
    let face_name = if face_ptr != 0 {
        emu.maps.read_wide_string(face_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "gdi32!CreateFontW h:{} w:{} weight:{} italic:{} charset:{} quality:{} face:'{}'",
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
