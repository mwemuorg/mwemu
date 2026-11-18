use super::EINVAL;
use super::set_errno;
use super::stream_handle;
use super::with_stream;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn fgets(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let max = api_try!(emu, call.arg_i32(emu, 1));
    let file = api_try!(emu, call.arg(emu, 2));
    let Some(handle) = stream_handle(file).filter(|_| dst != 0 && max > 0) else {
        set_errno(emu, EINVAL);
        call.set_return(emu, 0);
        return;
    };
    let mut line = Vec::new();
    if let Some(fh) = emu.handle_management.get_mut_file_handle(handle) {
        let mut byte = [0u8; 1];
        while line.len() + 1 < max as usize {
            match fh.read(&mut byte) {
                Ok(1) => {
                    line.push(byte[0]);
                    if byte[0] == b'\n' {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    let at_eof = !line.ends_with(b"\n") && line.len() + 1 < max as usize;
    with_stream(file, |s| s.eof |= at_eof);
    log_red!(
        emu,
        "wincrt!fgets 0x{:x} = {:?}",
        file,
        String::from_utf8_lossy(&line)
    );
    if line.is_empty() {
        call.set_return(emu, 0);
        return;
    }
    line.push(0);
    emu.maps.write_bytes(dst, &line);
    call.set_return(emu, dst);
}
