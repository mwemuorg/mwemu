use crate::emu;

pub fn InitializeSid(emu: &mut emu::Emu) {
    let sid_ptr = emu.regs().rcx;
    let authority_ptr = emu.regs().rdx;
    let sub_count = emu.regs().r8 as u8;
    if sid_ptr != 0 {
        emu.maps.write_byte(sid_ptr, 1); // Revision = 1
        emu.maps.write_byte(sid_ptr + 1, sub_count);
        if authority_ptr != 0 {
            for i in 0..6u64 {
                let b = emu.maps.read_byte(authority_ptr + i).unwrap_or(0);
                emu.maps.write_byte(sid_ptr + 2 + i, b);
            }
        }
        for i in 0..sub_count as u64 {
            emu.maps.write_dword(sid_ptr + 8 + i * 4, 0);
        }
    }
    log_red!(
        emu,
        "kernelbase!InitializeSid ptr:0x{:x} auth:0x{:x} subs:{}",
        sid_ptr,
        authority_ptr,
        sub_count
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
