use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PatBlt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    let w = api_try!(emu, call.arg_i32(emu, 3));
    let h = api_try!(emu, call.arg_i32(emu, 4));
    let rop = api_try!(emu, call.arg_u32(emu, 5));
    log_red!(
        emu,
        "gdi32!PatBlt hdc:0x{:x} x:{} y:{} w:{} h:{} rop:0x{:x}",
        hdc,
        x,
        y,
        w,
        h,
        rop
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
