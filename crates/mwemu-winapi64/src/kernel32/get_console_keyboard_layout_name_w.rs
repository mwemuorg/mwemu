use crate::emu;

pub fn GetConsoleKeyboardLayoutNameW(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetConsoleKeyboardLayoutNameW ptr:0x{:x}",
        buf_ptr
    );
    if emu.regs().rcx != 0 {
        emu.maps.write_wide_string(emu.regs().rcx, "00000409");
    }
    if buf_ptr != 0 {
        emu.maps.write_wide_string(buf_ptr, "00000409");
    }
    emu.regs_mut().rax = 1;
}
