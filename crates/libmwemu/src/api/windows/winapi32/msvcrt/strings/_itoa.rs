use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _itoa(emu: &mut emu::Emu, api: &str) {
    let val = WinApiAbi::arg(emu, 0) as u32;
    let buf = WinApiAbi::arg(emu, 1);
    let radix = WinApiAbi::try_arg(emu, 2).unwrap_or(10) as u32;
    let s = match radix {
        16 => format!("{:x}", val),
        8 => format!("{:o}", val),
        _ => format!("{}", val),
    };
    if buf != 0 {
        emu.maps.write_string(buf, &s);
    }
    log_red!(emu, "msvcrt!{} {} => '{}'", api, val, s);
    let first_param = WinApiAbi::arg(emu, 0);
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = buf;
}
