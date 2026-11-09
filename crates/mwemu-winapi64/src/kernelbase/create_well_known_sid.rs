use crate::emu;

pub fn CreateWellKnownSid(emu: &mut emu::Emu) {
    let well_known_sid_type = emu.regs().rcx as u32;
    let domain_sid = emu.regs().rdx;
    let p_sid = emu.regs().r8;
    let cb_sid = emu.regs().r9;

    log_red!(
        emu,
        "kernelbase!CreateWellKnownSid type:{} buf:0x{:x}",
        well_known_sid_type,
        p_sid
    );

    let sid_size: u32 = 28; // 8 + 5*4 = typical SID
    if cb_sid != 0 {
        let provided = emu.maps.read_dword(cb_sid).unwrap_or(0);
        emu.maps.write_dword(cb_sid, sid_size);
        if provided < sid_size {
            emu.set_last_error(122); // ERROR_INSUFFICIENT_BUFFER
            emu.regs_mut().rax = 0;
        } else if p_sid != 0 {
            // Write a mock SID: revision=1, sub_auth_count=5, NT authority
            emu.maps.write_byte(p_sid, 1); // revision
            emu.maps.write_byte(p_sid + 1, 5); // sub-authority count
            // identifier authority: NT Authority {0,0,0,0,0,5}
            for i in 0..5u64 {
                emu.maps.write_byte(p_sid + 2 + i, 0);
            }
            emu.maps.write_byte(p_sid + 7, 5);
            // sub-authorities: 21, 0, 0, 0, 500 (admin-like)
            emu.maps.write_dword(p_sid + 8, 21);
            emu.maps.write_dword(p_sid + 12, 0);
            emu.maps.write_dword(p_sid + 16, 0);
            emu.maps.write_dword(p_sid + 20, 0);
            emu.maps.write_dword(p_sid + 24, 500);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        } else {
            emu.set_last_error(87); // ERROR_INVALID_PARAMETER
            emu.regs_mut().rax = 0;
        }
    } else {
        emu.set_last_error(87);
        emu.regs_mut().rax = 0;
    }
}
