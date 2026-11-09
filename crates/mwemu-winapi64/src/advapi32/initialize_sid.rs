use crate::emu;

pub fn InitializeSid(emu: &mut emu::Emu) {
    let sid_ptr = emu.regs().rcx;
    let id_authority_ptr = emu.regs().rdx;
    let sub_count = emu.regs().r8 as u8;
    if sid_ptr != 0 {
        emu.maps.write_byte(sid_ptr, 1);
        emu.maps.write_byte(sid_ptr + 1, sub_count);
        if id_authority_ptr != 0 {
            let auth_bytes = emu.maps.read_bytes(id_authority_ptr, 6).to_vec();
            emu.maps.write_bytes(sid_ptr + 2, &auth_bytes);
        }
    }
    log_red!(
        emu,
        "advapi32!InitializeSid sid:0x{:x} count:{}",
        sid_ptr,
        sub_count
    );
    emu.regs_mut().rax = 1;
}
