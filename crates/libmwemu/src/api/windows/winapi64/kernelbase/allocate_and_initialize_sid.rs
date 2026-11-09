use crate::emu;

pub fn AllocateAndInitializeSid(emu: &mut emu::Emu) {
    let p_id_authority = emu.regs().rcx;
    let n_sub_authority_count = emu.regs().rdx as u8;
    let sub0 = emu.regs().r8 as u32;
    let sub1 = emu.regs().r9 as u32;
    let sub2 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
    let sub3 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
    let sub4 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
    let sub5 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0) as u32;
    let sub6 = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0) as u32;
    let sub7 = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0) as u32;
    let pp_sid = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0);

    let count = n_sub_authority_count.min(8) as usize;
    let sid_size = (8 + count * 4) as u64;

    log_red!(
        emu,
        "kernelbase!AllocateAndInitializeSid count:{} pp_sid:0x{:x}",
        count,
        pp_sid
    );

    if let Some(base) = emu.maps.alloc(sid_size) {
        let _ = emu.maps.create_map(
            &format!("sid_{:x}", base),
            base,
            sid_size,
            crate::maps::mem64::Permission::READ_WRITE,
        );
        emu.maps.write_byte(base, 1); // revision
        emu.maps.write_byte(base + 1, count as u8);
        // Copy 6-byte identifier authority
        for i in 0..6u64 {
            let b = emu.maps.read_byte(p_id_authority + i).unwrap_or(0);
            emu.maps.write_byte(base + 2 + i, b);
        }
        let subs = [sub0, sub1, sub2, sub3, sub4, sub5, sub6, sub7];
        for (i, &s) in subs.iter().enumerate().take(count) {
            emu.maps.write_dword(base + 8 + (i as u64) * 4, s);
        }
        if pp_sid != 0 {
            emu.maps.write_qword(pp_sid, base);
        }
        emu.set_last_error(0);
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(8); // ERROR_NOT_ENOUGH_MEMORY
        emu.regs_mut().rax = 0;
    }
}
