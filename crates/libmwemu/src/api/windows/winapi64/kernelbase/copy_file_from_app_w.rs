use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CopyFileFromAppW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let src_ptr = api_try!(emu, call.arg(emu, 0));
    let dst_ptr = api_try!(emu, call.arg(emu, 1));
    let flags = api_try!(emu, call.arg(emu, 2));
    let src = emu.maps.read_wide_string(src_ptr);
    let dst = emu.maps.read_wide_string(dst_ptr);
    log_red!(emu, "kernelbase!CopyFileFromAppW '{}' -> '{}'", src, dst);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
