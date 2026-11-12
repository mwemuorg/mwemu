use crate::emu;

pub fn sprintf(emu: &mut emu::Emu) {
    let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let fmt = emu.maps.read_string(fmt_ptr);
    if buf != 0 {
        emu.maps.write_string(buf, &fmt);
    }
    log_red!(emu, "msvcrt!sprintf fmt:'{}'", fmt);

    emu.regs_mut().rax = fmt.len() as u64;
}
