use crate::emu;

pub(super) fn crt_memset(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let val = emu.regs().rdx as u8;
    let count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "wincrt!memset dst:0x{:x} val:{} count:{}",
        dst,
        val,
        count
    );
    emu.maps.memset(dst, val, count);
    emu.regs_mut().rax = dst;
}
