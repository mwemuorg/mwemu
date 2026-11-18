use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetWindowTextA(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextA", false);
}

pub(super) fn GetWindowTextW(emu: &mut emu::Emu) {
    get_window_text(emu, "GetWindowTextW", true);
}

fn get_window_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let buf = api_try!(emu, call.arg(emu, 1));
    let max_count = api_try!(emu, call.arg(emu, 2)) as usize;
    log_red!(
        emu,
        "user32!{api} hwnd=0x{:x} buf=0x{:x} max={}",
        hwnd,
        buf,
        max_count
    );
    if wide {
        if buf != 0 && max_count > 0 {
            emu.maps.write_word(buf, 0);
        }
    } else {
        if buf != 0 && max_count > 0 {
            emu.maps.write_byte(buf, 0);
        }
    }

    call.set_return(emu, 0);
}
