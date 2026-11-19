use super::EINVAL;
use super::open;
use super::read_text;
use super::set_errno;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fopen_s(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let out = api_try!(emu, call.arg(emu, 0));
    let path_ptr = api_try!(emu, call.arg(emu, 1));
    let mode_ptr = api_try!(emu, call.arg(emu, 2));
    if out == 0 || path_ptr == 0 || mode_ptr == 0 {
        set_errno(emu, EINVAL);
        call.set_return(emu, EINVAL as u64);
        return;
    }
    let path = read_text(emu, path_ptr, wide);
    let mode = read_text(emu, mode_ptr, wide);
    let result = open(emu, &path, &mode);
    log_red!(emu, "wincrt!{} '{}' '{}' = {:x?}", api, path, mode, result);
    match result {
        Ok(stream) => {
            emu.maps.write_qword(out, stream);
            call.set_return(emu, 0);
        }
        Err(errno) => {
            emu.maps.write_qword(out, 0);
            set_errno(emu, errno);
            call.set_return(emu, errno as u64);
        }
    }
}
