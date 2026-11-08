use crate::emu;

pub(super) fn crt_strcmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let str1 = emu.maps.read_string(s1);
    let str2 = emu.maps.read_string(s2);
    let res = str1.cmp(&str2) as i32;
    log_red!(emu, "wincrt!strcmp '{}' '{}' => {}", str1, str2, res);
    emu.regs_mut().rax = res as u64;
}
