use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSANtohs(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let netshort = api_try!(emu, call.arg_u16(emu, 1));
    let result_ptr = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "ws2_32!WSANtohs sock:0x{:x} val:0x{:x}",
        sock,
        netshort
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if result_ptr != 0 {
            emu.maps.write_word(result_ptr, u16::from_be(netshort));
        }
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 3);
}
