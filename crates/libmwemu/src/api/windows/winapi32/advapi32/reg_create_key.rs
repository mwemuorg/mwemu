use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegCreateKeyA(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0) as u32;
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let phk_result = WinApiAbi::arg(emu, 2);
    let subkey = if subkey_ptr != 0 {
        emu.maps.read_string(subkey_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!RegCreateKeyA key:0x{:x} sub:'{}'",
        hkey,
        subkey
    );
    let new_key = helper::handler_create("reg_key");
    if phk_result != 0 {
        emu.maps.write_dword(phk_result, new_key as u32);
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 0;
}

pub fn RegCreateKeyW(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0);
    let subkey_ptr = WinApiAbi::arg(emu, 1);
    let subkey = emu.maps.read_wide_string(subkey_ptr);
    let hndl = helper::handler_create(&format!("regkey_{}", subkey));
    log_red!(
        emu,
        "advapi32!RegCreateKeyW hkey:0x{:x} '{}' => 0x{:x}",
        hkey,
        subkey,
        hndl
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
