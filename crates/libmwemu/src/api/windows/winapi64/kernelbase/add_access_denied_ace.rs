use crate::emu;

pub fn AddAccessDeniedAce(emu: &mut emu::Emu) {
    let acl_ptr = emu.regs().rcx;
    let ace_revision = emu.regs().rdx as u32;
    let access_mask = emu.regs().r8 as u32;
    let sid_ptr = emu.regs().r9;
    if acl_ptr != 0 && sid_ptr != 0 {
        let sub_count = emu.maps.read_byte(sid_ptr + 1).unwrap_or(1) as u32;
        let sid_len = 8 + sub_count * 4;
        let ace_size = (8 + sid_len) as u16;
        let acl_size = emu.maps.read_word(acl_ptr + 2).unwrap_or(8) as u64;
        let ace_count = emu.maps.read_word(acl_ptr + 4).unwrap_or(0);
        let mut offset = 8u64;
        for _ in 0..ace_count {
            let sz = emu.maps.read_word(acl_ptr + offset + 2).unwrap_or(0) as u64;
            offset += sz;
        }
        if offset + ace_size as u64 <= acl_size {
            emu.maps.write_byte(acl_ptr + offset, 1); // ACCESS_DENIED_ACE_TYPE
            emu.maps.write_byte(acl_ptr + offset + 1, 0);
            emu.maps.write_word(acl_ptr + offset + 2, ace_size);
            emu.maps.write_dword(acl_ptr + offset + 4, access_mask);
            for i in 0..sid_len as u64 {
                let b = emu.maps.read_byte(sid_ptr + i).unwrap_or(0);
                emu.maps.write_byte(acl_ptr + offset + 8 + i, b);
            }
            emu.maps.write_word(acl_ptr + 4, ace_count + 1);
        }
    }
    log_red!(
        emu,
        "kernelbase!AddAccessDeniedAce acl:0x{:x} rev:{} mask:0x{:x} sid:0x{:x}",
        acl_ptr,
        ace_revision,
        access_mask,
        sid_ptr
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
