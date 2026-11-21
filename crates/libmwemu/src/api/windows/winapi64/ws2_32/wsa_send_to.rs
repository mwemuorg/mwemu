use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSASendTo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let buffers_ptr = api_try!(emu, call.arg(emu, 1));
    let buffer_count = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_sent_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSASendTo sock:0x{:x} buffers:{}",
        sock,
        buffer_count
    );
    if !helper::socket_exist(sock) {
        emu.set_last_error(10038);
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        if buffers_ptr != 0 && buffer_count > 0 {
            let buf_len = emu.maps.read_dword(buffers_ptr).unwrap_or(0);
            if bytes_sent_ptr != 0 {
                emu.maps.write_dword(bytes_sent_ptr, buf_len);
            }
        }
        call.set_return(emu, 0);
    }
}
