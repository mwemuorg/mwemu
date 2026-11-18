use super::EINVAL;
use super::EOF;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fgetc(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    let Some(handle) = stream_handle(file) else {
        set_errno(emu, EINVAL);
        call.set_return(emu, EOF);
        return;
    };
    let mut byte = [0u8; 1];
    let read = emu
        .handle_management
        .get_mut_file_handle(handle)
        .and_then(|fh| fh.read(&mut byte).ok());
    log_red!(
        emu,
        "wincrt!{} 0x{:x} = {:?}",
        api,
        file,
        read.map(|_| byte[0])
    );
    match read {
        Some(1) => call.set_return(emu, byte[0] as u64),
        _ => {
            with_stream(file, |s| s.eof = true);
            call.set_return(emu, EOF);
        }
    }
}
