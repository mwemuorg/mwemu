use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;
use crate::windows::constants::network::*;

pub fn WSAConnect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let name_ptr = api_try!(emu, call.arg(emu, 1));
    let namelen = api_try!(emu, call.arg_u32(emu, 2));
    let caller_data_ptr = api_try!(emu, call.arg(emu, 3));
    let callee_data_ptr = api_try!(emu, call.arg(emu, 4));
    let sqos_ptr = api_try!(emu, call.arg(emu, 5));
    let gqos_ptr = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "ws2_32!WSAConnect sock:0x{:x} name:0x{:x} namelen:{} callerdata:0x{:x} calleedata:0x{:x} sqos:0x{:x} gqos:0x{:x}",
        sock,
        name_ptr,
        namelen,
        caller_data_ptr,
        callee_data_ptr,
        sqos_ptr,
        gqos_ptr
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(WSAENOTSOCK);
        call.set_return(emu, SOCKET_ERROR);
    } else {
        if callee_data_ptr != 0 {
            emu.maps.write_qword(callee_data_ptr, 0);
            emu.maps.write_qword(callee_data_ptr + 8, 0);
        }
        call.set_return(emu, 0);
    }
}
