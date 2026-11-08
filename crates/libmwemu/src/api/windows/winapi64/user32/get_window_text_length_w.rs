use crate::emu;

pub fn GetWindowTextLengthW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let buf_ptr = emu.regs().rdx;
    let max_count = emu.regs().r8;
    let title = "mwemu_window";
    if buf_ptr != 0 && max_count > 0 {
        emu.maps.write_wide_string(buf_ptr, title);
    }
    log_red!(
        emu,
        "user32!GetWindowTextLengthW hwnd:0x{:x} => '{}'",
        hwnd,
        title
    );
    let out_ptr = emu.regs().r8;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = title.len() as u64;
}
