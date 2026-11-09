use crate::emu;

pub fn lstrcpynW(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let max_len = emu.regs().r8 as usize;
    let s = emu.maps.read_wide_string(src);
    let truncated: String = s.chars().take(max_len.saturating_sub(1)).collect();
    emu.maps.write_wide_string(dst, &truncated);
    log_red!(
        emu,
        "kernelbase!lstrcpynW dst:0x{:x} '{}' max:{}",
        dst,
        truncated,
        max_len
    );
    emu.regs_mut().rax = dst;
}
