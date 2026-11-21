use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetFileVersionInfoSize(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));
    let handle_ptr = api_try!(emu, call.arg(emu, 1));

    let filename = emu.maps.read_string(filename_ptr);
    log_red!(emu, "version!GetFileVersionInfoSize `{}`", filename);

    if handle_ptr != 0 {
        emu.maps.write_dword(handle_ptr, 0);
    }
    call.set_return(emu, 0);
}
