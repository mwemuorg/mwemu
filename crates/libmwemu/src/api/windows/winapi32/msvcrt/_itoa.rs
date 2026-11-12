use crate::emu;

pub fn _itoa(emu: &mut emu::Emu, api: &str) {
    let val = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let radix = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(10);
    let s = match radix {
        16 => format!("{:x}", val),
        8 => format!("{:o}", val),
        _ => format!("{}", val),
    };
    if buf != 0 {
        emu.maps.write_string(buf, &s);
    }
    log_red!(emu, "msvcrt!{} {} => '{}'", api, val, s);
    let first_param = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.regs_mut().rax = buf;
}
