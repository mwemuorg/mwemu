use crate::emu;

pub fn _wcsnicmp(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let s1: String = emu
        .maps
        .read_wide_string(s1_ptr)
        .to_lowercase()
        .chars()
        .take(n)
        .collect();
    let s2: String = emu
        .maps
        .read_wide_string(s2_ptr)
        .to_lowercase()
        .chars()
        .take(n)
        .collect();
    let result = s1.cmp(&s2) as i32;
    log_red!(
        emu,
        "wincrt!_wcsnicmp '{}' '{}' n:{} => {}",
        s1,
        s2,
        n,
        result
    );
    emu.regs_mut().rax = result as u32 as u64;
}
