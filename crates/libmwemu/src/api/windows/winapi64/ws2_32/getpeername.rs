use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn getpeername(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let namelen_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "ws2_32!getpeername sock: 0x{:x} name: 0x{:x}",
        sock,
        name_ptr
    );
    if !helper::socket_exist(sock) {
        log_red!(emu, "  invalid socket");
        call.set_return(emu, 0xFFFFFFFF); // SOCKET_ERROR
    } else {
        if name_ptr != 0 {
            // Write dummy sockaddr_in: AF_INET, port 80, 127.0.0.1
            emu.maps.write_word(name_ptr, 2); // AF_INET
            emu.maps.write_word(name_ptr + 2, 80u16.to_be()); // port
            emu.maps.write_dword(name_ptr + 4, 0x0100007f); // 127.0.0.1
            for i in 0..8u64 {
                emu.maps.write_byte(name_ptr + 8 + i, 0); // padding
            }
        }
        if namelen_ptr != 0 {
            emu.maps.write_dword(namelen_ptr, 16);
        }
        call.set_return(emu, 0); // success
    }
}
