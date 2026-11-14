use crate::emu;

pub fn __stdio_common_vfprintf(emu: &mut emu::Emu) {
    let _options = emu.regs().rcx;
    let _stream = emu.regs().rdx;
    let fmt_ptr = emu.regs().r8;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!__stdio_common_vfprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
