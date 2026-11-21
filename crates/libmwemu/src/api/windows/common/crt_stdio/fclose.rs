use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::maps::buffers::BufferKind;
use crate::winapi::common::buffers;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{close_descriptor, set_errno};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;

pub(super) fn fclose(emu: &mut emu::Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let file = api_try!(emu, call.arg(emu, 0));
    let closed = close_stream(emu, runtime, file);
    log_red!(emu, "{}!fclose {file:#x} ok={closed}", runtime.library());
    if closed {
        call.set_return(emu, 0);
    } else {
        set_errno(emu, runtime, EBADF);
        call.set_return(emu, EOF);
    }
}

pub(super) fn close_stream(emu: &mut emu::Emu, runtime: Runtime, file: u64) -> bool {
    let fd = {
        let state = runtime.io(emu);
        state
            .streams
            .iter()
            .position(|stream| stream.file == file)
            .map(|index| state.streams.remove(index).fd)
    };
    let closed = fd.is_some_and(|fd| close_descriptor(emu, runtime, fd));
    let base = runtime.io(emu).standard_base;
    let width = runtime.stream_layout(emu).0;
    let standard = base.is_some_and(|base| (base..base + 3 * width).contains(&file));
    if fd.is_some() && !standard {
        buffers::free(emu, file, BufferKind::Crt);
    }
    closed
}
