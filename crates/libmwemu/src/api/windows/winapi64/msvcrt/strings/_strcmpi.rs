use crate::emu;

pub fn _strcmpi(emu: &mut emu::Emu) {
    let s1_ptr = emu.regs().rcx;
    let s2_ptr = emu.regs().rdx;
    let s1 = emu.maps.read_string(s1_ptr).to_lowercase();
    let s2 = emu.maps.read_string(s2_ptr).to_lowercase();
    let result = s1.cmp(&s2) as i32;
    log_red!(emu, "msvcrt!_strcmpi '{}' '{}' => {}", s1, s2, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
}
