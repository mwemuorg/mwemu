use crate::emu;

pub fn ExpandEnvironmentStringsA(emu: &mut emu::Emu) {
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ExpandEnvironmentStringsA cannot read src") as u64;
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!ExpandEnvironmentStringsA cannot read dst") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!ExpandEnvironmentStringsA cannot read size");

    let src = emu.maps.read_string(src_ptr);

    log_red!(emu, "kernel32!ExpandEnvironmentStringsA `{}`", src);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;

    //TODO: implement expand
}
