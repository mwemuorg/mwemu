use crate::emu;
use crate::winapi::common::strings;

pub fn GetTextFaceA(emu: &mut emu::Emu) {
    get_text_face(emu, "GetTextFaceA", false);
}

fn get_text_face(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdc = emu.regs().rcx;
    let count = emu.regs().rdx;
    let buf = emu.regs().r8;
    let face = "Arial";
    if buf != 0 && count > 0 {
        strings::write_string(emu, buf, face, wide);
    }
    log_red!(
        emu,
        "gdi32!{api} hdc:0x{:x} count:{} => '{}'",
        hdc,
        count,
        face
    );
    emu.regs_mut().rax = face.len() as u64;
}

pub fn GetTextFaceW(emu: &mut emu::Emu) {
    get_text_face(emu, "GetTextFaceW", true);
}
