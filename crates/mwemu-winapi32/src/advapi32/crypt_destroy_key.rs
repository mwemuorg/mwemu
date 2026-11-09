use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDestroyKey(emu: &mut emu::Emu) {
    let hkey = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(emu, "advapi32!CryptDestroyKey key: 0x{:x}", hkey);
    crypto_state::destroy_key(hkey);
    helper::handler_close(hkey);
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
