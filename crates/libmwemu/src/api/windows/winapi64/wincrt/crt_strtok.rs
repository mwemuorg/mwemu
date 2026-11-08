use crate::emu;

pub(super) fn crt_strtok(emu: &mut emu::Emu) {
    let s_ptr = emu.regs().rcx;
    let delim_ptr = emu.regs().rdx;
    let delims = emu.maps.read_string(delim_ptr);
    if s_ptr != 0 {
        let s = emu.maps.read_string(s_ptr);
        if let Some(pos) = s.find(|c: char| delims.contains(c)) {
            emu.maps.write_byte(s_ptr + pos as u64, 0);
            log_red!(
                emu,
                "wincrt!strtok '{}' delim:'{}' => '{}'",
                s,
                delims,
                &s[..pos]
            );
            emu.regs_mut().rax = s_ptr;
        } else if !s.is_empty() {
            log_red!(emu, "wincrt!strtok '{}' delim:'{}' => '{}'", s, delims, s);
            emu.regs_mut().rax = s_ptr;
        } else {
            log_red!(emu, "wincrt!strtok empty");
            emu.regs_mut().rax = 0;
        }
    } else {
        log_red!(emu, "wincrt!strtok(NULL) => 0");
        emu.regs_mut().rax = 0;
    }
}
