use crate::api::windows::common::crypto_state;
use crate::emu;

pub fn CryptHashSessionKey(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let hkey = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
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
