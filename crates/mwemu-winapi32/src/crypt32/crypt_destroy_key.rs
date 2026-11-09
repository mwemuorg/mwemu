use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyKey(emu: &mut emu::Emu) {
    let h_key = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "crypt32!CryptDestroyKey key: 0x{:x}", h_key);
    if h_key != 0 {
        crypto_state::destroy_key(h_key);
        helper::handler_close(h_key);
    }
    emu.regs_mut().rax = 1; // TRUE
}
