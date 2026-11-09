use crate::emu;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let _id_authority = emu.regs().rcx;
    let sub_count = (emu.regs().rdx as u8).min(8);
    let sub_auths: [u32; 8] = [
        emu.regs().r8 as u32,
        emu.regs().r9 as u32,
        emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as u32,
        emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32,
        emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32,
        emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32,
        emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as u32,
        emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as u32,
    ];
    let psid_ptr = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);
    let sid_len = 8 + sub_count as u64 * 4;
    if let Some(base) = emu.maps.alloc(sid_len) {
        let name = format!("sid_{:x}", base);
        emu.maps.create_map(
            &name,
            base,
            sid_len,
            crate::maps::mem64::Permission::READ_WRITE,
        );
        emu.maps.write_byte(base, 1);
        emu.maps.write_byte(base + 1, sub_count);
        for (i, &sa) in sub_auths.iter().enumerate().take(sub_count as usize) {
            emu.maps.write_dword(base + 8 + i as u64 * 4, sa);
        }
        if psid_ptr != 0 {
            emu.maps.write_qword(psid_ptr, base);
        }
        log_red!(
            emu,
            "advapi32!AllocateAndInitializeSid count:{} => 0x{:x}",
            sub_count,
            base
        );
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
