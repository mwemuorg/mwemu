use crate::emu;

pub fn ExpandEnvironmentStringsW(emu: &mut emu::Emu) {
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!ExpandEnvironmentStringsW cannot read src") as u64;
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!ExpandEnvironmentStringsW cannot read dst") as u64;
    let size = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!ExpandEnvironmentStringsW cannot read size");

    let src = emu.maps.read_wide_string(src_ptr);

    log_red!(emu, "kernel32!ExpandEnvironmentStringsW `{}`", src);

    //TODO: implement expand

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
