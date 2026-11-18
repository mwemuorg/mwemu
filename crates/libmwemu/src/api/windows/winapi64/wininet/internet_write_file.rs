use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetWriteFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    let buffer = api_try!(emu, call.arg(emu, 1));
    let bytes_to_write = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_written_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "wininet!InternetWriteFile handle:0x{:x} buf:0x{:x} sz:{}",
        handle,
        buffer,
        bytes_to_write
    );
    if bytes_written_ptr != 0 {
        emu.maps.write_dword(bytes_written_ptr, bytes_to_write);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
