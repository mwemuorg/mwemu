use crate::emu;

pub fn lstrcmpiA(emu: &mut emu::Emu) {
    let str1_ptr = emu.regs().rcx;
    let str2_ptr = emu.regs().rdx;
    let s1 = emu.maps.read_string(str1_ptr);
    let s2 = emu.maps.read_string(str2_ptr);
    let result = s1.to_lowercase().cmp(&s2.to_lowercase()) as i32;
    log_red!(
        emu,
        "kernelbase!lstrcmpiA '{}' vs '{}' => {}",
        s1,
        s2,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
