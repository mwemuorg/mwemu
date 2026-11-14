use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTextFaceA(emu: &mut emu::Emu) {
    get_text_face(emu, "GetTextFaceA", false);
}

fn get_text_face(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hdc = arg(emu, 0);
    let count = arg(emu, 1);
    let buf = arg(emu, 2) as u64;
    let face = "Arial";
    if buf != 0 && count > 0 {
        strings::write_string(emu, buf, face, wide);
    }
    log_red!(emu, "gdi32!{api} => '{}'", face);
    emu.regs_mut().rax = face.len() as u64;
    stdcall_ret(emu, 3);
}

pub fn GetTextFaceW(emu: &mut emu::Emu) {
    get_text_face(emu, "GetTextFaceW", true);
}
