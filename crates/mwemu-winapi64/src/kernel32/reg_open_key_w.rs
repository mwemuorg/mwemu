
use crate::windows::constants::errors::*;
use crate::emu;
use crate::winapi::helper;

pub fn RegOpenKeyW(emu: &mut emu::Emu) {
    let hKey = emu.regs().rcx;
    let subkey_ptr = emu.regs().rdx;
    let result = emu.regs().r8;

    let subkey = emu.maps.read_wide_string(subkey_ptr);
    emu.maps.write_dword(
        result,
        helper::handler_create(&format!("key://{}", subkey)) as u32,
    );

    log_red!(emu, "kernel32!RegOpenKeyW `{}`", subkey);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
