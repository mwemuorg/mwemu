use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptReleaseContext(emu: &mut emu::Emu) {
    let h_prov = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1) as u32;
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
