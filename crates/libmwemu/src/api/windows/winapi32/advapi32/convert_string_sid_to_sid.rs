use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn ConvertStringSidToSidA(emu: &mut emu::Emu) {
    convert_string_sid_to_sid(emu, "ConvertStringSidToSidA", false);
}

fn convert_string_sid_to_sid(emu: &mut emu::Emu, api: &str, wide: bool) {
    let str_sid_ptr = WinApiAbi::arg(emu, 0);
    let pp_sid = WinApiAbi::arg(emu, 1);
    let sid_str = if str_sid_ptr != 0 && str_sid_ptr > 0xffff {
        strings::read_text(emu, str_sid_ptr, wide)
    } else {
        String::new()
    };
    // Allocate a mock SID (S-1-5-21-... -> 28 bytes)
    let sid_size = 28u64;
    if let Some(sid_addr) = emu.maps.alloc(sid_size) {
        // Revision=1, SubAuthorityCount=5
        emu.maps.write_byte(sid_addr, 1);
        emu.maps.write_byte(sid_addr + 1, 5);
        // NT Authority {0,0,0,0,0,5}
        emu.maps.write_byte(sid_addr + 7, 5);
        // Sub-authorities: 21, placeholder values
        emu.maps.write_dword(sid_addr + 8, 21);
        emu.maps.write_dword(sid_addr + 12, 1234567890);
        emu.maps.write_dword(sid_addr + 16, 987654321);
        emu.maps.write_dword(sid_addr + 20, 111111111);
        emu.maps.write_dword(sid_addr + 24, 500);
        if pp_sid != 0 {
            emu.maps.write_dword(pp_sid, sid_addr as u32);
        }
    }
    log_red!(emu, "advapi32!{api} '{}'", sid_str);
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn ConvertStringSidToSidW(emu: &mut emu::Emu) {
    convert_string_sid_to_sid(emu, "ConvertStringSidToSidW", true);
}
