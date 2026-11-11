use crate::emu;

pub(super) fn GetWindowTextA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "user32!GetWindowTextA hwnd=0x{:x} buf=0x{:x} max={}",
        hwnd,
        buf,
        max_count
    );
    if buf != 0 && max_count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.regs_mut().rax = 0;
}

pub(super) fn GetWindowTextW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "user32!GetWindowTextW hwnd=0x{:x} buf=0x{:x} max={}",
        hwnd,
        buf,
        max_count
    );
    if buf != 0 && max_count > 0 {
        emu.maps.write_word(buf, 0);
    }
    emu.regs_mut().rax = 0;
}
