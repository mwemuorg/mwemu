use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::stdcall_ret;

pub fn RegCreateKeyA(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let phk_result = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
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
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let subkey_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
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
