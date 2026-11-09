use crate::emu;

pub fn atol(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let s = emu.maps.read_string(str_ptr);
    let val = s.trim().parse::<i32>().unwrap_or(0);
    log_red!(emu, "msvcrt!atol '{}' => {}", s, val);
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = val as u32 as u64;
}
