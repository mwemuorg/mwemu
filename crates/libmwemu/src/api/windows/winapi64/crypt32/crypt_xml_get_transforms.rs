use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptXmlGetTransforms(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pp_transforms = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "crypt32!CryptXmlGetTransforms ppTransforms:0x{:x}",
        pp_transforms
    );
    if pp_transforms != 0 {
        emu.maps.write_qword(pp_transforms, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
