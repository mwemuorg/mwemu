use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTextFaceA(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let count = arg(emu, 1);
    let buf = arg(emu, 2) as u64;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_string(buf, face);
    }
    log_red!(emu, "gdi32!GetTextFaceA => '{}'", face);
    emu.regs_mut().rax = face.len() as u64;
    stdcall_ret(emu, 3);
}

pub fn GetTextFaceW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let count = arg(emu, 1);
    let buf = arg(emu, 2) as u64;
    let face = "Arial";
    if buf != 0 && count > 0 {
        emu.maps.write_wide_string(buf, face);
    }
    log_red!(emu, "gdi32!GetTextFaceW => '{}'", face);
    emu.regs_mut().rax = face.len() as u64;
    stdcall_ret(emu, 3);
}
