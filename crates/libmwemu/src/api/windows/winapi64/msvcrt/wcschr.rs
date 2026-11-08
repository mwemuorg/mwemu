use crate::emu;

pub fn wcschr(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;
    let c = emu.regs().rdx as u16;
    let ch = char::from_u32(c as u32).unwrap_or('\0');
    let s = emu.maps.read_wide_string(s_ptr);
    let result = match s.find(ch) {
        Some(pos) => s_ptr + (pos * 2) as u64,
        None => 0,
    };
    log_red!(emu, "msvcrt!wcschr '{}' '{}' => 0x{:x}", s, ch, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
