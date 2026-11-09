use crate::emu;

pub fn MoveFileExW(emu: &mut emu::Emu) {
    let src = emu.regs().rcx;
    let dst = emu.regs().rdx;
    let src_s = emu.maps.read_wide_string(src);
    let dst_s = emu.maps.read_wide_string(dst);
    log_red!(emu, "kernel32!MoveFileExW '{}' -> '{}'", src_s, dst_s);
    emu.regs_mut().rax = 1;
}
