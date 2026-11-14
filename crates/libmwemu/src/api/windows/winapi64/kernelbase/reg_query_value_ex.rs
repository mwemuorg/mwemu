use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper::WinApiAbi;

pub fn RegQueryValueExA(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExA", false);
}

fn reg_query_value_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.regs().rcx;
    let name_ptr = emu.regs().rdx;
    let _reserved = emu.regs().r8;
    let type_ptr = emu.regs().r9;
    let data_ptr = WinApiAbi::arg(emu, 4);
    let size_ptr = WinApiAbi::arg(emu, 5);
    let name = strings::read_text(emu, name_ptr, wide);
    if type_ptr != 0 {
        emu.maps.write_dword(type_ptr, 1); // REG_SZ
    }
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, 0);
    }
    log_red!(emu, "kernelbase!{api} hkey:0x{:x} '{}'", hkey, name);
    emu.regs_mut().rax = 2;
}

pub fn RegQueryValueExW(emu: &mut emu::Emu) {
    reg_query_value_ex(emu, "RegQueryValueExW", true);
}
