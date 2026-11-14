use crate::emu;

pub fn strrchr(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;
    let c = emu.regs().rdx as u8 as char;
    let s = emu.maps.read_string(s_ptr);
    let found = s.rfind(c);
    let result = match found {
        Some(pos) => s_ptr + pos as u64,
        None => 0,
    };
    log_red!(emu, "msvcrt!strrchr '{}' '{}' => 0x{:x}", s, c, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
