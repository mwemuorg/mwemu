use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegQueryInfoKeyA(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyA", false);
}

fn reg_query_info_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let type_ptr = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
    stdcall_ret(emu, 12);
}

pub fn RegQueryInfoKeyW(emu: &mut emu::Emu) {
    reg_query_info_key(emu, "RegQueryInfoKeyW", true);
}
