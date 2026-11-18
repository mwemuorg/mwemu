use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn DrawIconEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let x_left = api_try!(emu, call.arg_i32(emu, 1));
    let y_top = api_try!(emu, call.arg_i32(emu, 2));
    let h_icon = api_try!(emu, call.arg(emu, 3));
    let cx_width = api_try!(emu, call.arg_i32(emu, 4));
    let cy_width = api_try!(emu, call.arg_i32(emu, 5));
    let _ist_step_if_ani_cur = api_try!(emu, call.arg(emu, 6));
    let _hbr_flicker_free_draw = api_try!(emu, call.arg(emu, 7));
    let _di_flags = api_try!(emu, call.arg(emu, 8));
    log_red!(
        emu,
        "user32!DrawIconEx hdc:0x{:x} x:{} y:{} icon:0x{:x} cx:{} cy:{}",
        hdc,
        x_left,
        y_top,
        h_icon,
        cx_width,
        cy_width
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
