use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PropVariantClear(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pvar = api_try!(emu, call.arg(emu, 0));
    if pvar != 0 {
        emu.maps.write_word(pvar, 0); // VT_EMPTY
    }
    log_red!(emu, "ole32!PropVariantClear ptr:0x{:x}", pvar);
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
