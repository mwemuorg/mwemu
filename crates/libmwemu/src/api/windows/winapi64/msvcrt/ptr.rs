use crate::emu;

pub fn ptr(emu: &mut emu::Emu) {
    let p = emu.regs().rcx;
    let val = if p != 0 {
        emu.maps.read_qword(p).unwrap_or(0)
    } else {
        0
    };
    log_red!(emu, "msvcrt!ptr 0x{:x} => [0x{:x}]", p, val);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = p;
}
