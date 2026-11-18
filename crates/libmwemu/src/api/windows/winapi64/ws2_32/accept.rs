use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn accept(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let saddr = api_try!(emu, call.arg(emu, 1));
    let addrlen = api_try!(emu, call.arg(emu, 2));

    log_red!(emu, "ws2_32!accept  sock: 0x{:x}", sock);

    if !helper::socket_exist(sock) {
        log::trace!("\tinvalid socket.");
        call.set_return(emu, !0u64); // INVALID_SOCKET
    } else {
        let new_sock = helper::socket_create();
        if saddr != 0 && addrlen != 0 {
            // Write a dummy sockaddr_in for the accepted connection
            emu.maps.write_word(saddr, 2); // AF_INET
            emu.maps.write_word(saddr + 2, 0x5000); // port 80 in network order
            emu.maps.write_dword(saddr + 4, 0x0100007f); // 127.0.0.1
            emu.maps.write_dword(addrlen, 16);
        }
        call.set_return(emu, new_sock);
    }
}
