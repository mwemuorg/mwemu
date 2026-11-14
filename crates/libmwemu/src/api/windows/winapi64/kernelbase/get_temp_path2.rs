use crate::emu;
use crate::winapi::common::strings;

pub fn GetTempPath2A(emu: &mut emu::Emu) {
    get_temp_path2(emu, "GetTempPath2A", false);
}

fn get_temp_path2(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernelbase!{api} '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn GetTempPath2W(emu: &mut emu::Emu) {
    get_temp_path2(emu, "GetTempPath2W", true);
}
