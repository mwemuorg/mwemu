use crate::emu;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let sia_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sub_auth_count = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let sa0 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let sa1 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let sa2 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let sa3 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let sa4 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);
    let sa5 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0);
    let sa6 = emu.maps.read_dword(emu.regs().get_esp() + 32).unwrap_or(0);
    let sa7 = emu.maps.read_dword(emu.regs().get_esp() + 36).unwrap_or(0);
    let pp_sid = emu.maps.read_dword(emu.regs().get_esp() + 40).unwrap_or(0) as u64;

    // SID: revision(1) + sub_auth_count(1) + authority(6) + sub_auths(4*count)
    let count = std::cmp::min(sub_auth_count, 8) as usize;
    let sid_size = 8 + count * 4;
    if let Some(sid_addr) = emu.maps.alloc(sid_size as u64) {
        // revision=1, sub_auth_count
        emu.maps.write_byte(sid_addr, 1);
        emu.maps.write_byte(sid_addr + 1, count as u8);
        // Copy SIA (6 bytes)
        if sia_ptr != 0 {
            for i in 0..6 {
                let b = emu.maps.read_byte(sia_ptr + i).unwrap_or(0);
                emu.maps.write_byte(sid_addr + 2 + i, b);
            }
        }
        // Write sub-authorities
        let sas = [sa0, sa1, sa2, sa3, sa4, sa5, sa6, sa7];
        for (i, &sa) in sas.iter().enumerate().take(count) {
            emu.maps.write_dword(sid_addr + 8 + (i as u64) * 4, sa);
        }
        if pp_sid != 0 {
            emu.maps.write_dword(pp_sid, sid_addr as u32);
        }
        log_red!(
            emu,
            "advapi32!AllocateAndInitializeSid count:{} => 0x{:x}",
            count,
            sid_addr
        );
    } else {
        log_red!(emu, "advapi32!AllocateAndInitializeSid alloc failed");
    }
    for _ in 0..11 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
