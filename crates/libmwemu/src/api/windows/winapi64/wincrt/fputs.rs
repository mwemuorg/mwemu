use crate::emu;

pub fn fputs(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let stream = emu.regs().rdx;
    let s = if str_ptr != 0 {
        emu.maps.read_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wincrt!fputs '{}' stream:0x{:x}", s, stream);
    emu.set_last_error(0);
    emu.regs_mut().rax = s.len() as u64;
}
