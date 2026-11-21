use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn VariantInit(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let variant_ptr = api_try!(emu, call.arg_u32(emu, 0)) as u64;

    log_red!(emu, "oleaut32!VariantInit ptr:0x{:x}", variant_ptr);

    if variant_ptr != 0 {
        for i in 0..4u64 {
            emu.maps.write_dword(variant_ptr + i * 4, 0);
        }
    }

    call.cleanup_args(emu, 1);
}
