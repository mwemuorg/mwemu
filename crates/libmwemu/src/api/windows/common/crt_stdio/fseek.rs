use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{set_errno, stream_handle, with_stream};
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;
use std::io::SeekFrom;

pub(super) fn fseek(emu: &mut emu::Emu, runtime: Runtime, api: &str, offset64: bool) {
    let call = WinApi(Cdecl);
    let offset = if offset64 {
        api_try!(emu, call.arg_i64(emu, 1))
    } else {
        api_try!(emu, call.arg_i32(emu, 1)) as i64
    };
    let file = api_try!(emu, call.arg(emu, 0));
    let origin = api_try!(
        emu,
        call.arg_u32(
            emu,
            if offset64 && !emu.cfg.arch.is_64bits() {
                3
            } else {
                2
            }
        )
    ) as u64;
    let target = match origin {
        SEEK_SET if offset >= 0 => Some(SeekFrom::Start(offset as u64)),
        SEEK_CUR => Some(SeekFrom::Current(offset)),
        SEEK_END => Some(SeekFrom::End(offset)),
        _ => None,
    };
    let pos = match (stream_handle(emu, runtime, file), target) {
        (Some(handle), Some(target)) => emu
            .handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.seek(target).ok()),
        _ => None,
    };
    log_red!(
        emu,
        "{}!{} 0x{:x} {} {} = {:?}",
        runtime.library(),
        api,
        file,
        offset,
        origin,
        pos
    );
    if pos.is_some() {
        with_stream(emu, runtime, file, |s| s.eof = false);
        call.set_return(emu, 0);
    } else {
        set_errno(emu, runtime, EINVAL);
        call.set_return(emu, EOF);
    }
}
