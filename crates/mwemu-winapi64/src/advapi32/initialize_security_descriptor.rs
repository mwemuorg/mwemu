use crate::emu;

pub fn InitializeSecurityDescriptor(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    let revision = emu.regs().rdx as u32;
    if sd_ptr != 0 {
        for i in 0..20u64 {
            emu.maps.write_byte(sd_ptr + i, 0);
        }
        emu.maps.write_byte(sd_ptr, revision as u8);
    }
    log_red!(
        emu,
        "advapi32!InitializeSecurityDescriptor sd:0x{:x} rev:{}",
        sd_ptr,
        revision
    );
    emu.regs_mut().rax = 1;
}
