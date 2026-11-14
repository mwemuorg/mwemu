use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub(super) fn strtok(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let delim_ptr = WinApiAbi::arg(emu, 1);
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
