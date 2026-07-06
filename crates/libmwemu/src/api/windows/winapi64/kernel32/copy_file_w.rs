use crate::emu;

pub fn CopyFileW(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let do_fail = emu.regs().r8;

    let src = emu.maps.read_wide_string(src_ptr);
    let dst = emu.maps.read_wide_string(dst_ptr);

    log_red!(emu, "kernel32!CopyFileW `{}` to `{}`", src, dst);

    emu.regs_mut().rax = 1;
}
