use crate::emu;

pub fn abs(emu: &mut emu::Emu, api: &str) {
    let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i32;
    let result = val.unsigned_abs();
    log_red!(emu, "msvcrt!{} {} => {}", api, val, result);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = result as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
}
