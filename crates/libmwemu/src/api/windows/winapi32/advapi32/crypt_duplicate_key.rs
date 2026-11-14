use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptDuplicateKey(emu: &mut emu::Emu) {
    let hkey = WinApiAbi::arg(emu, 0);
    let _reserved = WinApiAbi::arg(emu, 1) as u32;
    let _flags = WinApiAbi::arg(emu, 2) as u32;
    let ph_key = WinApiAbi::arg(emu, 3);
    let new_handle = helper::handler_create("crypt_key_dup");
    crypto_state::create_key(new_handle, 0x6801, 128);
    log_red!(
        emu,
        "advapi32!CryptDuplicateKey src:0x{:x} => 0x{:x}",
        hkey,
        new_handle
    );
    if ph_key != 0 {
        emu.maps.write_dword(ph_key, new_handle as u32);
    }
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
