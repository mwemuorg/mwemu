use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetPixel(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    let color = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "gdi32!SetPixel hdc:0x{:x} x:{} y:{} color:0x{:06x}",
        hdc,
        x,
        y,
        color
    );
    // Return the RGB color that was set (or CLR_INVALID on failure)
    emu.set_last_error(0);
    call.set_return(emu, color as u64);
}
