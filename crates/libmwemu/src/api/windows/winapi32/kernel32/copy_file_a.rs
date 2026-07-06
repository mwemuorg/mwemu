use crate::emu;

pub fn CopyFileA(emu: &mut emu::Emu) {
    let src_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("kernel32!CopyFileA cannot read src_ptr") as u64;
    let dst_ptr = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("kernel32!CopyFileA cannot read dst_ptr") as u64;
    let do_fail = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("kernel32!CopyFileA cannot read do_fail");

    let src = emu.maps.read_string(src_ptr);
    let dst = emu.maps.read_string(dst_ptr);

    log_red!(emu, "kernel32!CopyFileA `{}` to `{}`", src, dst);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
