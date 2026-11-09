use crate::emu;

pub fn ConvertSidToStringSidA(emu: &mut emu::Emu) {
    let str_sid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
            emu.maps.write_dword(str_sid_ptr, base as u32);
        }
    }
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
