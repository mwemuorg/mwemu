use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetFileTime(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hfile = api_try!(emu, call.arg(emu, 0));
    let creation_ptr = api_try!(emu, call.arg(emu, 1));
    let access_ptr = api_try!(emu, call.arg(emu, 2));
    let write_ptr = api_try!(emu, call.arg(emu, 3));
    let filetime: u64 = 0x01D7_A000_0000_0000;
    if creation_ptr != 0 {
        emu.maps.write_qword(creation_ptr, filetime);
    }
    if access_ptr != 0 {
        emu.maps.write_qword(access_ptr, filetime);
    }
    if write_ptr != 0 {
        emu.maps.write_qword(write_ptr, filetime);
    }
    log_red!(emu, "kernelbase!GetFileTime h:0x{:x}", hfile);
    call.set_return(emu, 1); // TRUE
}
