use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAAccept(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let addr_ptr = api_try!(emu, call.arg(emu, 1));
    let addrlen_ptr = api_try!(emu, call.arg(emu, 2));
    let condition_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "ws2_32!WSAAccept sock:0x{:x}", sock);
    if !helper::socket_exist(sock) {
        call.set_return(emu, 0xFFFFFFFFFFFFFFFF);
    } else {
        let new_sock = helper::socket_create();
        if addr_ptr != 0 {
            emu.maps.write_word(addr_ptr, 2);
            emu.maps.write_word(addr_ptr + 2, 0x5000);
            emu.maps.write_dword(addr_ptr + 4, 0x0100007f);
        }
        if addrlen_ptr != 0 {
            emu.maps.write_dword(addrlen_ptr, 16);
        }
        call.set_return(emu, new_sock);
    }
}
