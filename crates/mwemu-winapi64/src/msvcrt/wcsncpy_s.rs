use crate::emu;

pub fn wcsncpy_s(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let _dst_size = emu.regs().rdx;
    let src = emu.regs().r8;
    let n = emu.regs().r9 as usize;
    let s = emu.maps.read_wide_string(src);
    let truncated: String = s.chars().take(n).collect();
    if dst != 0 {
        emu.maps.write_wide_string(dst, &truncated);
    }
    log_red!(
        emu,
        "msvcrt!wcsncpy_s dst:0x{:x} '{}' n:{}",
        dst,
        truncated,
        n
    );
    emu.regs_mut().rax = 0;
}
