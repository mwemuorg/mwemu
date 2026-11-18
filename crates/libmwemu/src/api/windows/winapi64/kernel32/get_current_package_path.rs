use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetCurrentPackagePath(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_length_ptr = api_try!(emu, call.arg(emu, 0));
    let path_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernel32!GetCurrentPackagePath length_ptr:0x{:x} path_ptr:0x{:x}",
        path_length_ptr,
        path_ptr
    );
    if path_length_ptr != 0 {
        emu.maps.write_dword(path_length_ptr, 0);
    }
    call.set_return(emu, 0x80073D54); // APPMODEL_ERROR_NO_PACKAGE
}
