use crate::emu;

pub fn __stdio_common_vsscanf(emu: &mut emu::Emu) {
    let _options = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _options_hi = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _buf_size = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let fmt_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let s = emu.maps.read_string(buf);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsscanf buf:'{}' fmt:'{}'",
        s,
        fmt
    );

    emu.regs_mut().rax = 1; // number of fields read
}
