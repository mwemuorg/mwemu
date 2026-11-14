use crate::emu;

pub fn strncpy(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let s = emu.maps.read_string(src);
    let truncated: String = s.chars().take(n).collect();
    if dst != 0 {
        emu.maps.write_string(dst, &truncated);
    }
    log_red!(
        emu,
        "msvcrt!strncpy dst:0x{:x} '{}' n:{}",
        dst,
        truncated,
        n
    );
    emu.regs_mut().rax = dst;
}
