use crate::emu;

pub fn ConvertSidToStringSidA(emu: &mut emu::Emu) {
    let _sid = emu.regs().rcx;
    let str_sid_ptr = emu.regs().rdx;
    let sid_str = "S-1-5-21-1234567890-1234567890-1234567890-1001";
    log_red!(emu, "advapi32!ConvertSidToStringSidA -> '{}'", sid_str);
    if str_sid_ptr != 0 {
        let alloc_size = (sid_str.len() + 1) as u64;
        if let Some(base) = emu.maps.alloc(alloc_size) {
            let name = format!("sid_str_{:x}", base);
            emu.maps.create_map(
                &name,
                base,
                alloc_size,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            emu.maps.write_string(base, sid_str);
            emu.maps.write_qword(str_sid_ptr, base);
        }
    }
    emu.regs_mut().rax = 1;
}

pub fn ConvertSidToStringSidW(emu: &mut emu::Emu) {
    let _sid = emu.regs().rcx;
    let str_sid_ptr = emu.regs().rdx;
    let sid_str = "S-1-5-21-1234567890-1234567890-1234567890-1001";
    log_red!(emu, "advapi32!ConvertSidToStringSidW -> '{}'", sid_str);
    if str_sid_ptr != 0 {
        let wlen: Vec<u16> = sid_str.encode_utf16().chain(std::iter::once(0)).collect();
        let alloc_size = (wlen.len() * 2) as u64;
        if let Some(base) = emu.maps.alloc(alloc_size) {
            let name = format!("sid_strw_{:x}", base);
            emu.maps.create_map(
                &name,
                base,
                alloc_size,
                crate::maps::mem64::Permission::READ_WRITE,
            );
            for (i, ch) in wlen.iter().enumerate() {
                emu.maps.write_word(base + (i as u64) * 2, *ch);
            }
            emu.maps.write_qword(str_sid_ptr, base);
        }
    }
    emu.regs_mut().rax = 1;
}
