use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn DialogBoxIndirectParamA(emu: &mut emu::Emu) {
    dialog_box_indirect_param(emu, "DialogBoxIndirectParamA", false);
}

fn dialog_box_indirect_param(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = WinApiAbi::arg(emu, 4);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "user32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
}

pub fn DialogBoxIndirectParamW(emu: &mut emu::Emu) {
    dialog_box_indirect_param(emu, "DialogBoxIndirectParamW", true);
}
