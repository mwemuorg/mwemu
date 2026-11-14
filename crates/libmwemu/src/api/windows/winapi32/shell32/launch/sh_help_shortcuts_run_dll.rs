use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn SHHelpShortcuts_RunDLL(emu: &mut emu::Emu) {
    let arg1 = WinApiAbi::arg(emu, 0);
    let arg2 = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "shell32!SHHelpShortcuts_RunDLL arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn SHHelpShortcuts_RunDLLA(emu: &mut emu::Emu) {
    sh_help_shortcuts_run_dll(emu, "SHHelpShortcuts_RunDLLA", false);
}

fn sh_help_shortcuts_run_dll(emu: &mut emu::Emu, api: &str, wide: bool) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "shell32!{api} '{}'  p1:0x{:x}", s, p1);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn SHHelpShortcuts_RunDLLW(emu: &mut emu::Emu) {
    sh_help_shortcuts_run_dll(emu, "SHHelpShortcuts_RunDLLW", true);
}
