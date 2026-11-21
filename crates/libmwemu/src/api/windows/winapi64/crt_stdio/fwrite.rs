use super::EINVAL;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use std::io::SeekFrom;

pub(super) fn fwrite(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let buf = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let file = api_try!(emu, call.arg(emu, 3));
    let total = size.saturating_mul(count) as usize;
    let data = emu.maps.try_read_bytes(buf, total).map(|b| b.to_vec());
    let append = with_stream(file, |s| s.append).unwrap_or(false);
    let written = match (stream_handle(file), data) {
        (Some(handle), Some(data)) => {
            emu.handle_management
                .get_mut_file_handle(handle)
                .and_then(|fh| {
                    if append {
                        fh.seek(SeekFrom::End(0)).ok()?;
                    }
                    fh.write(&data).ok()
                })
        }
        _ => None,
    };
    log_red!(
        emu,
        "wincrt!fwrite buf:0x{:x} size:{} count:{} fp:0x{:x} = {:?}",
        buf,
        size,
        count,
        file,
        written
    );
    match written {
        Some(n) if size != 0 => call.set_return(emu, n as u64 / size),
        Some(_) => call.set_return(emu, 0),
        None => {
            with_stream(file, |s| s.error = true);
            set_errno(emu, EINVAL);
            call.set_return(emu, 0);
        }
    }
}
