use crate::emu;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let size = emu.regs().r8;

    let src = emu.maps.read_string(src_ptr);

    log_red!(emu, "kernel32!ExpandEnvironmentStringsA `{}`", src);
    // TODO: expand typical environment varsl.
    emu.regs_mut().rax = 1;
}
