use crate::emu;

pub fn vfprintf(emu: &mut emu::Emu) {
    let _stream = emu.regs().rcx;
    let fmt_ptr = emu.regs().rdx;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!vfprintf fmt:'{}'", fmt);
    emu.regs_mut().rax = fmt.len() as u64;
}
