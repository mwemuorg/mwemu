use crate::emu;

pub fn PropVariantCopy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    if dst != 0 && src != 0 {
        for i in 0..4u64 {
            let dw = emu.maps.read_dword(src + i * 4).unwrap_or(0);
            emu.maps.write_dword(dst + i * 4, dw);
        }
    }
    log_red!(emu, "ole32!PropVariantCopy dst:0x{:x} src:0x{:x}", dst, src);
    emu.regs_mut().rax = 0;
}
