use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyHash(emu: &mut emu::Emu) {
    let hhash = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!CryptDestroyHash hash: 0x{:x}", hhash);
    crypto_state::destroy_hash(hhash);
    helper::handler_close(hhash);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
