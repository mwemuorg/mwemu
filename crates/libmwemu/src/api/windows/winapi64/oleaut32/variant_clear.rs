use crate::emu;
use crate::windows::constants;

pub fn VariantClear(emu: &mut emu::Emu) {
    let pvarg = emu.regs().rcx;

    log_red!(emu, "oleaut32!VariantClear pvarg: 0x{:x}", pvarg);

    // Basic validation
    if pvarg == 0 || !emu.maps.is_mapped(pvarg) {
        log_red!(emu, "VariantClear: Invalid pvarg pointer");
        emu.set_last_error(0);
        emu.regs_mut().rax = constants::HRESULT_E_INVALID_ARG;
        return;
    }

    // Clear the variant by setting vt field to VT_EMPTY (0)
    // The vt field is typically at offset 0 in the VARIANT structure
    emu.maps.write_word(pvarg, 0); // VT_EMPTY = 0

    log_red!(emu, "VariantClear: Cleared variant (set vt to VT_EMPTY)");

    emu.set_last_error(0);
    emu.regs_mut().rax = 0; // S_OK
}
