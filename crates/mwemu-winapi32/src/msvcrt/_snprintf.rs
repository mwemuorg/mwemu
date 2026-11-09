use crate::emu;

pub fn _snprintf(emu: &mut emu::Emu, api: &str) {
    let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _size = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }
    emu.regs_mut().rax = fmt.len() as u64;
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
}
