use crate::emu;

pub fn GetKeyboardState(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    if buf != 0 {
        for i in 0..256u64 {
            emu.maps.write_byte(buf + i, 0);
        }
    }
    log_red!(emu, "user32!GetKeyboardState buf:0x{:x}", buf);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
