use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptHashSessionKey(emu: &mut emu::Emu) {
    let hhash = WinApiAbi::arg(emu, 0);
    let hkey = WinApiAbi::arg(emu, 1);
    let _flags = WinApiAbi::arg(emu, 2) as u32;
    let placeholder = vec![0x41u8; 16];
    crypto_state::hash_data(hhash, &placeholder);
    log_red!(
        emu,
        "advapi32!CryptHashSessionKey hash:0x{:x} key:0x{:x}",
        hhash,
        hkey
    );
    for _ in 0..3u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
