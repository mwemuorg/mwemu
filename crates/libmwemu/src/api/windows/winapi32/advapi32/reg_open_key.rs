use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyA", false);
}

fn reg_open_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let sam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let result_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let subkey = strings::read_text(emu, subkey_ptr, wide);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    if result_ptr != 0 {
        emu.maps.write_dword(result_ptr, hndl as u32);
    }
    log_red!(
        emu,
        "advapi32!{api} hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

pub fn RegOpenKeyW(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyW", true);
}
