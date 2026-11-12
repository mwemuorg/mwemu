use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::windows::constants::errors::*;

pub fn RegOpenKeyA(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyA", false);
}

fn reg_open_key(emu: &mut emu::Emu, api: &str, wide: bool) {
    let hKey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let result = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    let subkey = strings::read_text(emu, subkey_ptr, wide);
    emu.maps.write_dword(
        result,
        helper::handler_create(&format!("key://{}", subkey)) as u32,
    );

    log_red!(emu, "kernel32!{api} `{}`", subkey);
    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}

pub fn RegOpenKeyW(emu: &mut emu::Emu) {
    reg_open_key(emu, "RegOpenKeyW", true);
}
