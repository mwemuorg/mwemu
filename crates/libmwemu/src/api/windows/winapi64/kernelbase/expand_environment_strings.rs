use super::expand_env_vars;
use crate::emu;
use crate::winapi::common::strings;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsA", false);
}

fn expand_environment_strings(emu: &mut emu::Emu, api: &str, wide: bool) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let size = emu.regs().r8;
    let src = strings::read_text(emu, src_ptr, wide);
    let expanded = expand_env_vars(&src);
    if dst_ptr != 0 && size > expanded.len() as u64 {
        strings::write_string(emu, dst_ptr, &expanded, wide);
    }
    log_red!(emu, "kernelbase!{api} '{}' => '{}'", src, expanded);
    emu.regs_mut().rax = expanded.len() as u64 + 1;
}

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    expand_environment_strings(emu, "ExpandEnvironmentStringsW", true);
}
