use crate::emu;
use crate::winapi::common::strings;

pub fn FillConsoleOutputCharacterA(emu: &mut emu::Emu) {
    fill_console_output_character(emu, "FillConsoleOutputCharacterA", false);
}

fn fill_console_output_character(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernel32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn FillConsoleOutputCharacterW(emu: &mut emu::Emu) {
    fill_console_output_character(emu, "FillConsoleOutputCharacterW", true);
}
