use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let sia_ptr = WinApiAbi::arg(emu, 0);
    let sub_auth_count = WinApiAbi::arg(emu, 1) as u32;
    let sa0 = WinApiAbi::arg(emu, 2) as u32;
    let sa1 = WinApiAbi::arg(emu, 3) as u32;
    let sa2 = WinApiAbi::arg(emu, 4) as u32;
    let sa3 = WinApiAbi::arg(emu, 5) as u32;
    let sa4 = WinApiAbi::arg(emu, 6) as u32;
    let sa5 = WinApiAbi::arg(emu, 7) as u32;
    let sa6 = WinApiAbi::arg(emu, 8) as u32;
    let sa7 = WinApiAbi::arg(emu, 9) as u32;
    let pp_sid = WinApiAbi::arg(emu, 10);

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
