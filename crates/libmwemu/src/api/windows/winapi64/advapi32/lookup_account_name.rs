use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn LookupAccountNameA(emu: &mut emu::Emu) {
    lookup_account_name(emu, "LookupAccountNameA", false);
}

fn lookup_account_name(emu: &mut emu::Emu, api: &str, wide: bool) {
    let _system_ptr = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let sid_ptr = emu.regs().r8;
    let sid_sz_ptr = emu.regs().r9;
    let domain_ptr = WinApiAbi::arg(emu, 4);
    let domain_sz_ptr = WinApiAbi::arg(emu, 5);
    let use_ptr = WinApiAbi::arg(emu, 6);
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(emu, "advapi32!{api} '{}'", name);
    if sid_ptr != 0 {
        emu.maps.write_byte(sid_ptr, 1);
        emu.maps.write_byte(sid_ptr + 1, 5);
        emu.maps.write_byte(sid_ptr + 7, 5);
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
        emu.maps.write_dword(use_ptr, 1);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn LookupAccountNameW(emu: &mut emu::Emu) {
    lookup_account_name(emu, "LookupAccountNameW", true);
}
