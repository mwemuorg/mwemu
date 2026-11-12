use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn LookupAccountNameA(emu: &mut emu::Emu) {
    lookup_account_name(emu, "LookupAccountNameA", false);
}

fn lookup_account_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _system_ptr = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let sid_ptr = WinApiAbi::arg(emu, 2);
    let sid_sz_ptr = WinApiAbi::arg(emu, 3);
    let domain_ptr = WinApiAbi::arg(emu, 4);
    let domain_sz_ptr = WinApiAbi::arg(emu, 5);
    let use_ptr = WinApiAbi::arg(emu, 6);
    let name = strings::read_text(emu, name_ptr, wide);
    // Write a mock SID (28 bytes = S-1-5-21-...)
    if sid_ptr != 0 {
        emu.maps.write_byte(sid_ptr, 1); // revision
        emu.maps.write_byte(sid_ptr + 1, 5); // sub-auth count
        emu.maps.write_byte(sid_ptr + 7, 5); // NT Authority
        emu.maps.write_dword(sid_ptr + 8, 21);
        emu.maps.write_dword(sid_ptr + 12, 1234567890);
        emu.maps.write_dword(sid_ptr + 16, 987654321);
        emu.maps.write_dword(sid_ptr + 20, 111111111);
        emu.maps.write_dword(sid_ptr + 24, 500);
    }
    if sid_sz_ptr != 0 {
        emu.maps.write_dword(sid_sz_ptr, 28);
    }
    if domain_ptr != 0 {
        strings::write_string(emu, domain_ptr, "DESKTOP", wide);
    }
    if domain_sz_ptr != 0 {
        emu.maps.write_dword(domain_sz_ptr, 7);
    }
    if use_ptr != 0 {
        emu.maps.write_dword(use_ptr, 1); // SidTypeUser
    }
    log_red!(emu, "advapi32!{api} '{}'", name);
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}

pub fn LookupAccountNameW(emu: &mut emu::Emu) {
    lookup_account_name(emu, "LookupAccountNameW", true);
}
