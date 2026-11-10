use super::expand_env_vars;
use crate::emu;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let size = emu.regs().r8;
    let src = emu.maps.read_string(src_ptr);
    let expanded = expand_env_vars(&src);
    if dst_ptr != 0 && size > expanded.len() as u64 {
        emu.maps.write_string(dst_ptr, &expanded);
    }
    log_red!(
        emu,
        "kernelbase!ExpandEnvironmentStringsA '{}' => '{}'",
        src,
        expanded
    );
    emu.regs_mut().rax = expanded.len() as u64 + 1;
}

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let size = emu.regs().r8;
    let src = emu.maps.read_wide_string(src_ptr);
    let expanded = expand_env_vars(&src);
    if dst_ptr != 0 && size > expanded.len() as u64 {
        emu.maps.write_wide_string(dst_ptr, &expanded);
    }
    log_red!(
        emu,
        "kernelbase!ExpandEnvironmentStringsW '{}' => '{}'",
        src,
        expanded
    );
    emu.regs_mut().rax = expanded.len() as u64 + 1;
}
