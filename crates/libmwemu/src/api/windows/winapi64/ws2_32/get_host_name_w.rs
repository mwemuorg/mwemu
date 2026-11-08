use crate::emu;

pub fn GetHostNameW(emu: &mut emu::Emu) {
    let name_ptr = emu.regs().rcx;
    let namelen = emu.regs().rdx as u32;
    log_red!(
        emu,
        "ws2_32!GetHostNameW buf:0x{:x} len:{}",
        name_ptr,
        namelen
    );
    let out_ptr = emu.regs().rdx;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if name_ptr != 0 && namelen > 8 {
        emu.maps.write_wide_string(name_ptr, "DESKTOP");
    }
    emu.regs_mut().rax = 0;
}
