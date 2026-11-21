use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetFileVersionInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));
    let _handle = api_try!(emu, call.arg(emu, 1));
    let _len = api_try!(emu, call.arg(emu, 2));
    let _data = api_try!(emu, call.arg(emu, 3));

    let filename = emu.maps.read_string(filename_ptr);
    log_red!(emu, "version!GetFileVersionInfo `{}`", filename);

    call.set_return(emu, 0);
}
