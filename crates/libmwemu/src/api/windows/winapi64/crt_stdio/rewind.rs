use super::stream_handle;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use std::io::SeekFrom;

pub(super) fn rewind(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "wincrt!rewind 0x{:x}", file);
    if let Some(handle) = stream_handle(file)
        && let Some(fh) = emu.handle_management.get_mut_file_handle(handle)
    {
        let _ = fh.seek(SeekFrom::Start(0));
    }
    with_stream(file, |s| {
        s.eof = false;
        s.error = false;
    });
}
