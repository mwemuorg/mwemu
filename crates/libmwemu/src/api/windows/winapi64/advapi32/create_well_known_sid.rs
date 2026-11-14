use crate::emu;

pub fn CreateWellKnownSid(emu: &mut emu::Emu) {
    let sid_type = emu.regs().rcx as u32;
    let _domain_sid = emu.regs().rdx;
    let sid_ptr = emu.regs().r8;
    let cb_sid_ptr = emu.regs().r9;
    let sid_bytes: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 0x12, 0, 0, 0];
    if cb_sid_ptr != 0 {
        emu.maps.write_dword(cb_sid_ptr, 12);
    }
    if sid_ptr != 0 {
        emu.maps.write_bytes(sid_ptr, &sid_bytes);
    }
    log_red!(
        emu,
        "advapi32!CreateWellKnownSid type:{} sid:0x{:x}",
        sid_type,
        sid_ptr
    );
    emu.regs_mut().rax = 1;
}
