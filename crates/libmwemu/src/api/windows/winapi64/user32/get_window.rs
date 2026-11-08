use crate::emu;

pub fn GetWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let cmd = emu.regs().rdx;
    log_red!(emu, "user32!GetWindow hwnd:0x{:x} cmd:{}", hwnd, cmd);
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
