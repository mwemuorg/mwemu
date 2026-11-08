use crate::emu;

pub(super) fn crt_wcscmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let w1 = emu.maps.read_wide_string(s1);
    let w2 = emu.maps.read_wide_string(s2);
    let res = w1.cmp(&w2) as i32;
    log_red!(emu, "wincrt!wcscmp '{}' '{}' => {}", w1, w2, res);
    emu.regs_mut().rax = res as u64;
}
