use crate::emu;

pub fn __stdio_common_vsprintf(emu: &mut emu::Emu) {
    let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsprintf buf:0x{:x} fmt:'{}'",
        buf,
        fmt
    );
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }
    for _ in 0..6u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = fmt.len() as u64;
}
