use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{stream_handle, with_stream};
use crate::winapi::policy::api_try;
use std::io::SeekFrom;

pub(super) fn rewind(emu: &mut emu::Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{}!rewind 0x{:x}", runtime.library(), file);
    if let Some(handle) = stream_handle(emu, runtime, file)
        && let Some(fh) = emu.handle_management.get_mut_file_handle(handle)
    {
        let _ = fh.seek(SeekFrom::Start(0));
    }
    with_stream(emu, runtime, file, |s| {
        s.eof = false;
        s.error = false;
    });
}
