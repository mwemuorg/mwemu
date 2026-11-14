use crate::emu;

pub fn __stdio_common_vsprintf_s(emu: &mut emu::Emu) {
    let _options = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let _buf_size = emu.regs().r8;
    let fmt_ptr = emu.regs().r9;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsprintf_s buf:0x{:x} fmt:'{}'",
        buf,
        fmt
    );
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }
    emu.regs_mut().rax = fmt.len() as u64;
}
