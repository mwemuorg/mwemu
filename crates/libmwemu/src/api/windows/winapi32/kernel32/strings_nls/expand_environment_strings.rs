use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsA", false);
}

fn expand_environment_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src_ptr = WinApiAbi::arg(emu, 0);
    let dst_ptr = WinApiAbi::arg(emu, 1);
    let size = WinApiAbi::arg(emu, 2) as u32;

    let src = strings::read_text(emu, src_ptr, wide);

    log_red!(emu, "kernel32!{api} `{}`", src);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;

    //TODO: implement expand
}

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsW", true);
}
