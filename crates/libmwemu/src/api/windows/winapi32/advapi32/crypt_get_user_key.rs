use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptGetUserKey(emu: &mut emu::Emu) {
    let _hprov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let ph_user_key = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let handle = helper::handler_create("crypt_user_key");
    crypto_state::create_key(handle, 0x6801, 128);
    log_red!(
        emu,
        "advapi32!CryptGetUserKey spec:{} => 0x{:x}",
        key_spec,
        handle
    );
    if ph_user_key != 0 {
        emu.maps.write_dword(ph_user_key, handle as u32);
    }
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
