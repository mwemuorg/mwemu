use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WinHttpReadData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_request = api_try!(emu, call.arg(emu, 0));
    let _buffer = api_try!(emu, call.arg(emu, 1));
    let _bytes_to_read = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_read_ptr = api_try!(emu, call.arg(emu, 3));

    log_red!(emu, "winhttp!WinHttpReadData request: 0x{:x}", h_request);

    if bytes_read_ptr != 0 {
        emu.maps.write_dword(bytes_read_ptr, 0);
    }

    call.set_return(emu, 1);
}
