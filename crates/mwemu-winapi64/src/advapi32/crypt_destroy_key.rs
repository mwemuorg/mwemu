use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyKey(emu: &mut emu::Emu) {
    let hkey = emu.regs().rcx;
    log_red!(emu, "advapi32!CryptDestroyKey key: 0x{:x}", hkey);
    crypto_state::destroy_key(hkey);
    helper::handler_close(hkey);
    emu.regs_mut().rax = 1;
}
