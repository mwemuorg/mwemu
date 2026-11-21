use super::EINVAL;
use super::EOF;
use super::SEEK_CUR;
use super::SEEK_END;
use super::SEEK_SET;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use std::io::SeekFrom;

pub(super) fn fseek(emu: &mut emu::Emu, api: &str, offset64: bool) {
    let call = WinApi(Stdcall);
    let raw_offset = api_try!(emu, call.arg(emu, 1));
    let offset = if offset64 {
        raw_offset as i64
    } else {
        raw_offset as i32 as i64
    };
    let file = api_try!(emu, call.arg(emu, 0));
    let origin = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let target = match origin {
        SEEK_SET if offset >= 0 => Some(SeekFrom::Start(offset as u64)),
        SEEK_CUR => Some(SeekFrom::Current(offset)),
        SEEK_END => Some(SeekFrom::End(offset)),
        _ => None,
    };
    let pos = match (stream_handle(file), target) {
        (Some(handle), Some(target)) => emu
            .handle_management
            .get_mut_file_handle(handle)
            .and_then(|fh| fh.seek(target).ok()),
        _ => None,
    };
    log_red!(
        emu,
        "wincrt!{} 0x{:x} {} {} = {:?}",
        api,
        file,
        offset,
        origin,
        pos
    );
    if pos.is_some() {
        with_stream(file, |s| s.eof = false);
        call.set_return(emu, 0);
    } else {
        set_errno(emu, EINVAL);
        call.set_return(emu, EOF);
    }
}
