use crate::emu;

pub(super) fn crt_memmove(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8;
    let sz = n.min(0x100_0000) as usize;
    if let Some(bytes) = emu.maps.try_read_bytes(src, sz).map(|b| b.to_vec()) {
        let _ = emu.maps.write_bytes(dst, &bytes);
    }
    emu.regs_mut().rax = dst;
}
