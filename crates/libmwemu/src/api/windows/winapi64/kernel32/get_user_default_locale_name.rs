use crate::emu;

pub fn GetUserDefaultLocaleName(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let buf_len = emu.regs().rdx as u32;
    log_red!(
        emu,
        "kernel32!GetUserDefaultLocaleName ptr:0x{:x} len:{}",
        buf_ptr,
        buf_len
    );
    if buf_ptr != 0 && buf_len >= 6 {
        emu.maps.write_wide_string(buf_ptr, "en-US");
    }
    emu.regs_mut().rax = 6;
}
