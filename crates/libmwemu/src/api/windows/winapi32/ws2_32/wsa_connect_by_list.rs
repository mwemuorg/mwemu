use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSAConnectByList(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let addr_list_ptr = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let local_addr_len_ptr = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let local_addr_ptr = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let remote_addr_len_ptr = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let remote_addr_ptr = api_try!(emu, call.arg_u32(emu, 5)) as u64;
    let timeout_ptr = api_try!(emu, call.arg_u32(emu, 6)) as u64;
    let _reserved = api_try!(emu, call.arg_u32(emu, 7)) as u64;
    log_red!(
        emu,
        "ws2_32!WSAConnectByList sock:0x{:x} addrlist:0x{:x} localaddr:0x{:x} remoteaddr:0x{:x}",
        sock,
        addr_list_ptr,
        local_addr_ptr,
        remote_addr_ptr
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0);
    } else {
        if local_addr_len_ptr != 0 {
            emu.maps.write_dword(local_addr_len_ptr, 16);
        }
        if local_addr_ptr != 0 {
            emu.maps.write_dword(local_addr_ptr, 2);
        }
        if remote_addr_len_ptr != 0 {
            emu.maps.write_dword(remote_addr_len_ptr, 16);
        }
        if remote_addr_ptr != 0 {
            emu.maps.write_dword(remote_addr_ptr, 2);
        }
        call.set_return(emu, 1);
    }
    call.cleanup_args(emu, 8);
}
