use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn VariantClear(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pvarg = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "oleaut32!VariantClear pvarg: 0x{:x}", pvarg);

    // Basic validation
    if pvarg == 0 || !emu.maps.is_mapped(pvarg) {
        log_red!(emu, "VariantClear: Invalid pvarg pointer");
        emu.set_last_error(0);
        call.set_return(emu, HRESULT_E_INVALID_ARG);
        return;
    }

    // Clear the variant by setting vt field to VT_EMPTY (0)
    // The vt field is typically at offset 0 in the VARIANT structure
    emu.maps.write_word(pvarg, 0); // VT_EMPTY = 0

    log_red!(emu, "VariantClear: Cleared variant (set vt to VT_EMPTY)");

    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
