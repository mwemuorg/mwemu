use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegQueryValueA(emu: &mut emu::Emu) {
    reg_query_value(emu, "RegQueryValueA", false);
}

fn reg_query_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0);
    let name_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2);
    let type_ptr = WinApiAbi::arg(emu, 3);
    let name = strings::read_text(emu, name_ptr, wide);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    log_red!(emu, "advapi32!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
    stdcall_ret(emu, 4);
}

pub fn RegQueryValueW(emu: &mut emu::Emu) {
    reg_query_value(emu, "RegQueryValueW", true);
}
