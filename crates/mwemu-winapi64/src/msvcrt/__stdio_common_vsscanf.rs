use crate::emu;

pub fn __stdio_common_vsscanf(emu: &mut emu::Emu) {
    let _options = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let _buf_size = emu.regs().r8;
    let fmt_ptr = emu.regs().r9;
    let s = emu.maps.read_string(buf);
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(
        emu,
        "msvcrt!__stdio_common_vsscanf buf:'{}' fmt:'{}'",
        s,
        fmt
    );
    emu.regs_mut().rax = 1;
}
