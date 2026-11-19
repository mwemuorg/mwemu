use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAConnect(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let name_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let namelen = api_try!(emu, call.arg_u32(emu, 2));
    let caller_data_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let callee_data_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let sqos_ptr = api_try!(emu, call.arg_u32(emu, 5)) as u64;
    let gqos_ptr = api_try!(emu, call.arg_u32(emu, 6)) as u64;
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
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if callee_data_ptr != 0 {
            emu.maps.write_dword(callee_data_ptr, 0);
            emu.maps.write_dword(callee_data_ptr + 4, 0);
        }
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 7);
}
