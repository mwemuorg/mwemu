use crate::emu;

pub fn ConvertStringSidToSidW(emu: &mut emu::Emu) {
    let str_sid_ptr = emu.regs().rcx;
    let sid_out_ptr = emu.regs().rdx;
    let s = if str_sid_ptr != 0 && str_sid_ptr > 0xffff {
        emu.maps.read_wide_string(str_sid_ptr)
    } else {
        format!("0x{:x}", str_sid_ptr)
    };
    log_red!(emu, "advapi32!ConvertStringSidToSidW '{}'", s);
    if sid_out_ptr != 0 {
        let sid_bytes: [u8; 28] = [
            1, 5, 0, 0, 0, 0, 0, 5, 0x15, 0, 0, 0, 0xd2, 0x04, 0, 0, 0xd2, 0x04, 0, 0, 0xd2, 0x04,
            0, 0, 0xe9, 0x03, 0, 0,
        ];
        let alloc_size = sid_bytes.len() as u64;
        if let Some(base) = emu.maps.alloc(alloc_size) {
            let name = format!("sid_{:x}", base);
            emu.maps.create_map(
                &name,
                base,
                alloc_size,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_bytes(base, &sid_bytes);
            emu.maps.write_qword(sid_out_ptr, base);
        }
    }
    emu.regs_mut().rax = 1;
}
