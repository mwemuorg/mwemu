use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAGetOverlappedResult(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let sock = api_try!(emu, call.arg(emu, 0));
    let overlapped_ptr = api_try!(emu, call.arg(emu, 1));
    let transfer_ptr = api_try!(emu, call.arg(emu, 2));
    let wait = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "ws2_32!WSAGetOverlappedResult sock:0x{:x} wait:{}",
        sock,
        wait
    );
    if transfer_ptr != 0 {
        emu.maps.write_dword(transfer_ptr, 0);
    }
    call.set_return(emu, 1);
}
