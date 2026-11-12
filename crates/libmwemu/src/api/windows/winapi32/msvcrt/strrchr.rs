use crate::emu;

pub fn strrchr(emu: &mut emu::Emu) {
    let s_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let c = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u8 as char;
    let s = emu.maps.read_string(s_ptr);
    let found = s.rfind(c);
    let result = match found {
        Some(pos) => s_ptr + pos as u64,
        None => 0,
    };
    log_red!(emu, "msvcrt!strrchr '{}' '{}' => 0x{:x}", s, c, result);

    emu.regs_mut().rax = result;
}
