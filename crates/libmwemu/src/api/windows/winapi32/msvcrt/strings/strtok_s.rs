use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn strtok_s(emu: &mut emu::Emu) {
    let str_ptr = WinApiAbi::arg(emu, 0);
    let delim_ptr = WinApiAbi::arg(emu, 1);
    let context_ptr = WinApiAbi::arg(emu, 2);
    let delim = emu.maps.read_string(delim_ptr);
    let s = emu.maps.read_string(str_ptr);
    if let Some(pos) = s.find(|c: char| delim.contains(c)) {
        emu.maps.write_byte(str_ptr + pos as u64, 0);
        if context_ptr != 0 {
            emu.maps
                .write_dword(context_ptr, (str_ptr + pos as u64 + 1) as u32);
        }
        log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => token", s, delim);
        emu.regs_mut().rax = str_ptr;
    } else {
        log_red!(emu, "msvcrt!strtok_s '{}' delim:'{}' => null", s, delim);
        emu.regs_mut().rax = 0;
    }
}
