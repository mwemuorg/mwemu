use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptDestroyHash(emu: &mut emu::Emu) {
    let h_hash = WinApiAbi::arg(emu, 0);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "crypt32!CryptDestroyHash hash: 0x{:x}", h_hash);
    if h_hash != 0 {
        crypto_state::destroy_hash(h_hash);
        helper::handler_close(h_hash);
    }
    emu.regs_mut().rax = 1; // TRUE
}
