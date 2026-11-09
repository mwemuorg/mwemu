use crate::emu;

pub(super) fn crt_wcscat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let dst_ws = emu.maps.read_wide_string(dst);
    let src_ws = emu.maps.read_wide_string(src);
    emu.maps
        .write_wide_string(dst + (dst_ws.len() as u64) * 2, &src_ws);
    log_red!(emu, "wincrt!wcscat '{}' + '{}'", dst_ws, src_ws);
    emu.regs_mut().rax = dst;
}
