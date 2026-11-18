use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSARecv(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let buffers_ptr = api_try!(emu, call.arg(emu, 1));
    let buffer_count = api_try!(emu, call.arg_u32(emu, 2));
    let bytes_recvd_ptr = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSARecv sock: 0x{:x} buffers: {} bytes_recvd: 0x{:x}",
        sock,
        buffer_count,
        bytes_recvd_ptr
    );
    if !helper::socket_exist(sock) {
        call.set_return(emu, 0xFFFFFFFF);
    } else {
        // Fill first WSABUF with 0x90 (nop sled, same as recv)
        if buffers_ptr != 0 && buffer_count > 0 {
            let buf_len = emu.maps.read_dword(buffers_ptr).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_qword(buffers_ptr + 8).unwrap_or(0);
            if buf_ptr != 0 {
                let fill_len = buf_len.min(1024);
                for i in 0..fill_len {
                    emu.maps.write_byte(buf_ptr + i, 0x90);
                }
                if bytes_recvd_ptr != 0 {
                    emu.maps.write_dword(bytes_recvd_ptr, fill_len as u32);
                }
            }
        }
        call.set_return(emu, 0);
    }
}
