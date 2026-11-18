use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSANtohl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let netlong = api_try!(emu, call.arg_u32(emu, 1));
    let result_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(emu, "ws2_32!WSANtohl sock:0x{:x} val:0x{:x}", sock, netlong);
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if result_ptr != 0 {
            emu.maps.write_dword(result_ptr, u32::from_be(netlong));
        }
        call.set_return(emu, 0);
    }
}
