use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn FillConsoleOutputCharacterA(emu: &mut emu::Emu) {
    fill_console_output_character(emu, "FillConsoleOutputCharacterA", false);
}

fn fill_console_output_character(emu: &mut emu::Emu, api: &str, wide: bool) {
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
        "kernelbase!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn FillConsoleOutputCharacterW(emu: &mut emu::Emu) {
    fill_console_output_character(emu, "FillConsoleOutputCharacterW", true);
}
