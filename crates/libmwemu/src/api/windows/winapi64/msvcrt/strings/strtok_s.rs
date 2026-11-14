use crate::emu;

pub fn strtok_s(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let delim_ptr = emu.regs().rdx;
    let context_ptr = emu.regs().r8;
    let delim = emu.maps.read_string(delim_ptr);
    let s = emu.maps.read_string(str_ptr);
    if let Some(pos) = s.find(|c: char| delim.contains(c)) {
        emu.maps.write_byte(str_ptr + pos as u64, 0);
        if context_ptr != 0 {
            emu.maps.write_qword(context_ptr, str_ptr + pos as u64 + 1);
        }
        log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => token", s, delim);
        emu.regs_mut().rax = str_ptr;
    } else {
        log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => null", s, delim);
        emu.set_last_error(0);
        emu.regs_mut().rax = 0;
    }
}
