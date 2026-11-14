use crate::emu;

pub fn GetWindowThreadProcessId(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let pid_ptr = emu.regs().rdx;
    log_red!(emu, "user32!GetWindowThreadProcessId hwnd:0x{:x}", hwnd);
    if pid_ptr != 0 {
        emu.maps.write_dword(pid_ptr, 0x123);
    }
    emu.regs_mut().rax = 0x123;
}
