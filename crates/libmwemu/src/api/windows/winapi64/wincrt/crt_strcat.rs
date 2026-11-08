use crate::emu;

pub(super) fn crt_strcat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let dst_s = emu.maps.read_string(dst);
    let src_s = emu.maps.read_string(src);
    emu.maps.write_string(dst + dst_s.len() as u64, &src_s);
    log_red!(emu, "wincrt!strcat '{}' + '{}'", dst_s, src_s);
    emu.regs_mut().rax = dst;
}
