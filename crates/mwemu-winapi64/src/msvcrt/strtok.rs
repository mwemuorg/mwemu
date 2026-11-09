use crate::emu;

pub fn strtok(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let delim_ptr = emu.regs().rdx;
    let delims = emu.maps.read_string(delim_ptr);
    if str_ptr != 0 {
        let s = emu.maps.read_string(str_ptr);
        if let Some(pos) = s.find(|c: char| delims.contains(c)) {
            emu.maps.write_byte(str_ptr + pos as u64, 0);
            log_red!(
                emu,
                "msvcrt!strtok '{}' delim:'{}' => '{}'",
                s,
                delims,
                &s[..pos]
            );
            emu.regs_mut().rax = str_ptr;
        } else if !s.is_empty() {
            log_red!(emu, "msvcrt!strtok '{}' delim:'{}' => '{}'", s, delims, s);
            emu.regs_mut().rax = str_ptr;
        } else {
            log_red!(emu, "msvcrt!strtok empty");
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
    } else {
        log_red!(emu, "msvcrt!strtok(NULL) => 0");
        emu.regs_mut().rax = 0;
    }
}
