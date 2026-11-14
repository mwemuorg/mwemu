use crate::emu;

pub fn _putenv(emu: &mut emu::Emu) {
    let str_ptr = emu.regs().rcx;
    let s = if str_ptr != 0 {
        emu.maps.read_string(str_ptr)
    } else {
        String::new()
    };
    log_red!(emu, "wincrt!_putenv '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
