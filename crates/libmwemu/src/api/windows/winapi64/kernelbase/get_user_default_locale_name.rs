use crate::emu;

pub fn GetUserDefaultLocaleName(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let buf_size = emu.regs().rdx;
    if buf_ptr != 0 && buf_size > 0 {
        emu.maps.write_wide_string(buf_ptr, "en-US");
    }
    log_red!(emu, "kernelbase!GetUserDefaultLocaleName => en-US");
    emu.regs_mut().rax = 6;
}
