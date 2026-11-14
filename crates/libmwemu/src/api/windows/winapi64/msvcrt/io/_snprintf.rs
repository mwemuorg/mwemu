use crate::emu;

pub fn _snprintf(emu: &mut emu::Emu, api: &str) {
    let buf = emu.regs().rcx;
    let _size = emu.regs().rdx;
    let fmt_ptr = emu.regs().r8;
    let fmt = emu.maps.read_string(fmt_ptr);
    log_red!(emu, "msvcrt!{} buf:0x{:x} fmt:'{}'", api, buf, fmt);
    if buf != 0 && !fmt.is_empty() {
        emu.maps.write_string(buf, &fmt);
    }
    emu.regs_mut().rax = fmt.len() as u64;
}
