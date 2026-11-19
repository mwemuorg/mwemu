use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn PeekMessageA(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageA");
}

fn peek_message(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let msg_ptr = api_try!(emu, call.arg(emu, 0));
    let hwnd = api_try!(emu, call.arg(emu, 1));
    let msg_filter_min = api_try!(emu, call.arg(emu, 2));
    let msg_filter_max = api_try!(emu, call.arg(emu, 3));
    let remove_msg = api_try!(emu, call.arg(emu, 4));
    if msg_ptr != 0 {
        for i in 0..7u64 {
            emu.maps.write_qword(msg_ptr + i * 8, 0);
        }
    }
    log_red!(
        emu,
        "user32!{api} msg:0x{:x} hwnd:0x{:x} min:{} max:{} rm:0x{:x}",
        msg_ptr,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    call.set_return(emu, 0);
}

pub(super) fn PeekMessageW(emu: &mut emu::Emu) {
    peek_message(emu, "PeekMessageW");
}
