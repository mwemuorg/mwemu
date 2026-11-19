use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn WinHttpReadData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let request = api_try!(emu, call.arg_u32(emu, 0));
    let bytes_read_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    log_red!(emu, "winhttp!WinHttpReadData request: 0x{:x}", request);
    if bytes_read_ptr != 0 {
        emu.maps.write_dword(bytes_read_ptr, 0); // orderly end of data
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
