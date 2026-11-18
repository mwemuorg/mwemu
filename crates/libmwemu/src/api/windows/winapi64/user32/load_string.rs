use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn LoadStringA(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringA", false);
}

pub(super) fn LoadStringW(emu: &mut emu::Emu) {
    load_string(emu, "LoadStringW", true);
}

fn load_string(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let _hinst = api_try!(emu, call.arg(emu, 0));
    let id = api_try!(emu, call.arg(emu, 1));
    let buf = api_try!(emu, call.arg(emu, 2));
    let buf_max = api_try!(emu, call.arg(emu, 3)) as usize;
    log_red!(emu, "user32!{api} id:{} buf:0x{:x}", id, buf);
    if wide {
        if buf != 0 && buf_max > 0 {
            emu.maps.write_word(buf, 0);
        }
    } else {
        if buf != 0 && buf_max > 0 {
            emu.maps.write_byte(buf, 0);
        }
    }

    call.set_return(emu, 0);
}
