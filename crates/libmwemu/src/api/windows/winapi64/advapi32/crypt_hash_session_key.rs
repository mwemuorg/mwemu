use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptHashSessionKey(emu: &mut emu::Emu) {
    let hhash = emu.regs().rcx;
    let hkey = emu.regs().rdx;
    let _flags = emu.regs().r8;
    let placeholder = vec![0x41u8; 16];
    crypto_state::hash_data(hhash, &placeholder);
    log_red!(
        emu,
        "advapi32!CryptHashSessionKey hash:0x{:x} key:0x{:x}",
        hhash,
        hkey
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
