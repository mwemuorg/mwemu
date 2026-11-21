use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn StretchBlt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc_dest = api_try!(emu, call.arg(emu, 0));
    let x_dest = api_try!(emu, call.arg_i32(emu, 1));
    let y_dest = api_try!(emu, call.arg_i32(emu, 2));
    let w_dest = api_try!(emu, call.arg_i32(emu, 3));
    let h_dest = api_try!(emu, call.arg_i32(emu, 4));
    let hdc_src = api_try!(emu, call.arg(emu, 5));
    let x_src = api_try!(emu, call.arg_i32(emu, 6));
    let y_src = api_try!(emu, call.arg_i32(emu, 7));
    let w_src = api_try!(emu, call.arg_i32(emu, 8));
    let h_src = api_try!(emu, call.arg_i32(emu, 9));
    let rop = api_try!(emu, call.arg_u32(emu, 10));
    log_red!(
        emu,
        "gdi32!StretchBlt dst:0x{:x} x:{} y:{} {}x{} src:0x{:x} x:{} y:{} {}x{} rop:0x{:x}",
        hdc_dest,
        x_dest,
        y_dest,
        w_dest,
        h_dest,
        hdc_src,
        x_src,
        y_src,
        w_src,
        h_src,
        rop
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
