use crate::emu;

pub fn wcsncpy(emu: &mut emu::Emu) {
    let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let n = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
    let s = emu.maps.read_wide_string(src);
    let truncated: String = s.chars().take(n).collect();
    if dst != 0 {
        emu.maps.write_wide_string(dst, &truncated);
    }
    log_red!(
        emu,
        "msvcrt!wcsncpy dst:0x{:x} '{}' n:{}",
        dst,
        truncated,
        n
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = dst;
}
