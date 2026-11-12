use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegSetValueA(emu: &mut emu::Emu) {
    reg_set_value(emu, "RegSetValueA", false);
}

fn reg_set_value(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let name_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let reg_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let name = strings::read_text(emu, name_ptr, wide);
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' type:{}",
        hkey,
        name,
        reg_type
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

pub fn RegSetValueW(emu: &mut emu::Emu) {
    reg_set_value(emu, "RegSetValueW", true);
}
