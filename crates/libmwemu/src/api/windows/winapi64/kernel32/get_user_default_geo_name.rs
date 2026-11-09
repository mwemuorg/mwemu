use crate::emu;

pub fn GetUserDefaultGeoName(emu: &mut emu::Emu) {
    let buf_ptr = emu.regs().rcx;
    let buf_len = emu.regs().rdx as u32;
    log_red!(
        emu,
        "kernel32!GetUserDefaultGeoName ptr:0x{:x} len:{}",
        buf_ptr,
        buf_len
    );
    if buf_ptr != 0 && buf_len >= 3 {
        emu.maps.write_wide_string(buf_ptr, "US");
    }
    emu.regs_mut().rax = 3;
}
