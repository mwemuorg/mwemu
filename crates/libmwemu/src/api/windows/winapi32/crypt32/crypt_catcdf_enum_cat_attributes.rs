use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATCDFEnumCatAttributes(emu: &mut emu::Emu) {
    let p_cdf = WinApiAbi::arg(emu, 0);
    let p_prev_attr = WinApiAbi::arg(emu, 1);
    let pfn_parse_error = WinApiAbi::arg(emu, 2);
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumCatAttributes p_cdf:0x{:x} p_prev_attr:0x{:x} pfn_parse_error:0x{:x}",
        p_cdf,
        p_prev_attr,
        pfn_parse_error
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
