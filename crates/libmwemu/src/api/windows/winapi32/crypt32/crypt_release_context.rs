use crate::emu;
use crate::winapi::helper;

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(
        emu,
        "crypt32!CryptReleaseContext prov: 0x{:x} flags: 0x{:x}",
        h_prov,
        dw_flags
    );
    if h_prov != 0 {
        helper::handler_close(h_prov);
    }
    emu.regs_mut().rax = 1;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
