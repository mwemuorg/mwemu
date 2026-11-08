use crate::emu;

pub fn MoveFileA(emu: &mut emu::Emu) {
    let src_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dst_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;

    let src = emu.maps.read_string(src_ptr);
    let dst = emu.maps.read_string(dst_ptr);

    log_red!(emu, "kernel32!MoveFileA `{}` to `{}`", src, dst);

    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = 1;
}
