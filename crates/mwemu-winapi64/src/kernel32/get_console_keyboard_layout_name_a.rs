use crate::emu;

pub fn GetConsoleKeyboardLayoutNameA(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!GetConsoleKeyboardLayoutNameA ptr:0x{:x}",
        buf_ptr
    );
    if emu.regs().rcx != 0 {
        emu.maps.write_string(emu.regs().rcx, "00000409");
    }
    if buf_ptr != 0 {
        emu.maps.write_string(buf_ptr, "00000409");
    }
    emu.regs_mut().rax = 1;
}
