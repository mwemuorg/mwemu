use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDuplicateKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    let _reserved = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let ph_key = emu.regs().r9;
    let new_handle = helper::handler_create("crypt_key_dup");
    crypto_state::create_key(new_handle, 0x6801, 128);
    log_red!(
        emu,
        "advapi32!CryptDuplicateKey src:0x{:x} => 0x{:x}",
        hkey,
        new_handle
    );
    if ph_key != 0 {
        emu.maps.write_qword(ph_key, new_handle);
    }
    emu.regs_mut().rax = 1;
}
