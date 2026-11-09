use crate::emu;

pub fn CopyFileFromAppW(emu: &mut emu::Emu) {
    let src_ptr = emu.regs().rcx;
    let dst_ptr = emu.regs().rdx;
    let flags = emu.regs().r8;
    let src = emu.maps.read_wide_string(src_ptr);
    let dst = emu.maps.read_wide_string(dst_ptr);
    log_red!(emu, "kernelbase!CopyFileFromAppW '{}' -> '{}'", src, dst);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
