use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetScrollRange(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hwnd = api_try!(emu, call.arg(emu, 0));
    let n_bar = api_try!(emu, call.arg_i32(emu, 1));
    let n_min_pos = api_try!(emu, call.arg_i32(emu, 2));
    let n_max_pos = api_try!(emu, call.arg_i32(emu, 3));
    let b_redraw = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "user32!SetScrollRange hwnd:0x{:x} bar:{} min:{} max:{} redraw:{}",
        hwnd,
        n_bar,
        n_min_pos,
        n_max_pos,
        b_redraw
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
