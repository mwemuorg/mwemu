use crate::emu;

pub fn _itoa(emu: &mut emu::Emu, api: &str) {
    let val = emu.regs().rcx as u32;
    let buf = emu.regs().rdx;
    let radix = emu.regs().r8 as u32;
    let s = match radix {
        16 => format!("{:x}", val),
        8 => format!("{:o}", val),
        _ => format!("{}", val),
    };
    if buf != 0 {
        emu.maps.write_string(buf, &s);
    }
    log_red!(emu, "msvcrt!{} {} => '{}'", api, val, s);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = buf;
}
