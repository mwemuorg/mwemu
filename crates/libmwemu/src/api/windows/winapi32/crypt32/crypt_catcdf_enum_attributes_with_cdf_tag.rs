use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCATCDFEnumAttributesWithCDFTag(emu: &mut emu::Emu) {
    let p_cdf = WinApiAbi::arg(emu, 0);
    let pwsz_member_tag = WinApiAbi::arg(emu, 1);
    let p_member = WinApiAbi::arg(emu, 2);
    let p_prev_attr = WinApiAbi::arg(emu, 3);
    let pfn_parse_error = WinApiAbi::arg(emu, 4);
    log_red!(
        emu,
        "crypt32!CryptCATCDFEnumAttributesWithCDFTag 0x{:x}",
        p_cdf
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
