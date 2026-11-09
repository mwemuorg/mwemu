use crate::emu;

pub(super) fn crt_strnicmp(emu: &mut emu::Emu) {
    let s1 = emu.regs().rcx;
    let s2 = emu.regs().rdx;
    let n = emu.regs().r8 as usize;
    let str1 = emu.maps.read_string(s1).to_ascii_lowercase();
    let str2 = emu.maps.read_string(s2).to_ascii_lowercase();
    let a = &str1[..str1.len().min(n)];
    let b = &str2[..str2.len().min(n)];
    let res = a.cmp(b) as i32;
    emu.regs_mut().rax = res as u64;
}
