use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyA", false);
}

fn reg_open_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hkey = WinApiAbi::arg(emu, 0);
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let reserved = WinApiAbi::arg(emu, 2);
    let sam = WinApiAbi::arg(emu, 3);
    let result_ptr = WinApiAbi::arg(emu, 4);
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
