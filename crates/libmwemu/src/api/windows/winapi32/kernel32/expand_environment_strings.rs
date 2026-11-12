use crate::emu;
use crate::winapi::common::strings;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsA", false);
}

fn expand_environment_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let size = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

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
