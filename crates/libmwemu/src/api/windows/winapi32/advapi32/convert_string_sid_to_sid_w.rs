use crate::emu;

pub fn ConvertStringSidToSidW(emu: &mut emu::Emu) {
    let str_sid_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pp_sid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let sid_str = if str_sid_ptr != 0 && str_sid_ptr > 0xffff {
        emu.maps.read_wide_string(str_sid_ptr)
    } else {
        String::new()
    };
    let sid_size = 28u64;
    if let Some(sid_addr) = emu.maps.alloc(sid_size) {
        emu.maps.write_byte(sid_addr, 1);
        emu.maps.write_byte(sid_addr + 1, 5);
        emu.maps.write_byte(sid_addr + 7, 5);
        emu.maps.write_dword(sid_addr + 8, 21);
        emu.maps.write_dword(sid_addr + 12, 1234567890);
        emu.maps.write_dword(sid_addr + 16, 987654321);
        emu.maps.write_dword(sid_addr + 20, 111111111);
        emu.maps.write_dword(sid_addr + 24, 500);
        if pp_sid != 0 {
            emu.maps.write_dword(pp_sid, sid_addr as u32);
        }
    }
    log_red!(emu, "advapi32!ConvertStringSidToSidW '{}'", sid_str);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
