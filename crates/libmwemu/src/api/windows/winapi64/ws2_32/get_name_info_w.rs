use crate::emu;

pub fn GetNameInfoW(emu: &mut emu::Emu) {
    let sa_ptr = emu.regs().rcx;
    let salen = emu.regs().rdx as u32;
    let host_ptr = emu.regs().r8;
    let hostlen = emu.regs().r9 as u32;
    log_red!(
        emu,
        "ws2_32!GetNameInfoW sa:0x{:x} host:0x{:x} hostlen:{}",
        sa_ptr,
        host_ptr,
        hostlen
    );
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if host_ptr != 0 && hostlen > 10 {
        emu.maps.write_wide_string(host_ptr, "localhost");
    }
    emu.regs_mut().rax = 0;
}
