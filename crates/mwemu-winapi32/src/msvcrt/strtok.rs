use crate::emu;

pub(super) fn strtok(emu: &mut emu::Emu) {
    let str_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let delim_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            emu.regs_mut().rax = 0;
        }
    } else {
        log_red!(emu, "msvcrt!strtok(NULL) => 0");
        emu.regs_mut().rax = 0;
    }
}
