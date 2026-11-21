use super::EINVAL;
use super::EOF;
use super::set_errno;
use super::stream_handle;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn ftell(emu: &mut emu::Emu, api: &str, long: bool) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let pos = stream_handle(file).and_then(|handle| {
        emu.handle_management
            .get_mut_file_handle(handle)
            .map(|fh| fh.get_position())
    });
    log_red!(emu, "wincrt!{} 0x{:x} = {:?}", api, file, pos);
    match pos {
        Some(pos) if long && pos > i32::MAX as u64 => {
            set_errno(emu, EINVAL);
            call.set_return(emu, EOF);
        }
        Some(pos) => call.set_return(emu, pos),
        None => {
            set_errno(emu, EINVAL);
            call.set_return(emu, if long { EOF } else { u64::MAX });
        }
    }
}
