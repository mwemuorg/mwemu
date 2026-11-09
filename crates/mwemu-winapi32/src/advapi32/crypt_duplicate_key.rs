use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDuplicateKey(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let _reserved = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let ph_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
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
