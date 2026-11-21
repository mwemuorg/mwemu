use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VariantInit(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pvarg = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "oleaut32!VariantInit 0x{:x}", pvarg);

    if pvarg != 0 {
        // Zero out the 16-byte VARIANT structure
        for i in 0..16u64 {
            emu.maps.write_byte(pvarg + i, 0);
        }
    }
    // VariantInit has no return value (void), but we set rax=0 for safety
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
