use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptGetUserKey(emu: &mut emu::Emu) {
    let _hprov = emu.regs().rcx;
    let key_spec = emu.regs().rdx;
    let ph_user_key = emu.regs().r8;
    let handle = helper::handler_create("crypt_user_key");
    crypto_state::create_key(handle, 0x6801, 128);
    log_red!(
        emu,
        "advapi32!CryptGetUserKey spec:{} => 0x{:x}",
        key_spec,
        handle
    );
    if ph_user_key != 0 {
        emu.maps.write_qword(ph_user_key, handle);
    }
    emu.regs_mut().rax = 1;
}
