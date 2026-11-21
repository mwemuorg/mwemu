use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CombineRgn(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dest = api_try!(emu, call.arg(emu, 0));
    let src1 = api_try!(emu, call.arg(emu, 1));
    let src2 = api_try!(emu, call.arg(emu, 2));
    let mode = api_try!(emu, call.arg_u32(emu, 3));
    let mode_name = match mode {
        1 => "RGN_AND",
        2 => "RGN_OR",
        3 => "RGN_XOR",
        4 => "RGN_DIFF",
        5 => "RGN_COPY",
        _ => "UNKNOWN",
    };
    log_red!(
        emu,
        "gdi32!CombineRgn dest:0x{:x} src1:0x{:x} src2:0x{:x} mode:{} ({})",
        dest,
        src1,
        src2,
        mode,
        mode_name
    );
    emu.set_last_error(0);
    call.set_return(emu, 1); // SIMPLEREGION
}
