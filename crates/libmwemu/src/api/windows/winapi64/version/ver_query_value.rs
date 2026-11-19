use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn VerQueryValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _block = api_try!(emu, call.arg(emu, 0));
    let subblock_ptr = api_try!(emu, call.arg(emu, 1));
    let _buffer = api_try!(emu, call.arg(emu, 2));
    let _len = api_try!(emu, call.arg(emu, 3));

    let subblock = emu.maps.read_string(subblock_ptr);
    log_red!(emu, "version!VerQueryValue `{}`", subblock);

    call.set_return(emu, 0);
}
