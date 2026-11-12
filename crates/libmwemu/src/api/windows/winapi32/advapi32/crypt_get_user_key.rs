use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGetUserKey(emu: &mut emu::Emu) {
    let _hprov = WinApiAbi::arg(emu, 0);
    let key_spec = WinApiAbi::arg(emu, 1) as u32;
    let ph_user_key = WinApiAbi::arg(emu, 2);
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
