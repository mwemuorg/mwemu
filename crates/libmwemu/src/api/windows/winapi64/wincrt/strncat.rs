use crate::emu;

pub fn strncat(emu: &mut emu::Emu) {
    let dst = emu.regs().rcx;
    let src = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let existing = emu.maps.read_string(dst);
    let append = emu.maps.read_string(src);
    let truncated: String = append.chars().take(n).collect();
    let result = format!("{}{}", existing, truncated);
    if dst != 0 {
        emu.maps.write_string(dst, &result);
    }
    log_red!(emu, "wincrt!strncat => '{}' n:{}", result, n);
    emu.regs_mut().rax = dst;
}
