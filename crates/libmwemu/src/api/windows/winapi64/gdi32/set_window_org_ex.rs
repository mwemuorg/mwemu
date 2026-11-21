use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetWindowOrgEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    let lp_point = api_try!(emu, call.arg(emu, 3));
    // Write previous origin to lpPoint if non-null
    if lp_point != 0 {
        emu.maps.write_dword(lp_point, 0); // previous x
        emu.maps.write_dword(lp_point + 4, 0); // previous y
    }
    log_red!(
        emu,
        "gdi32!SetWindowOrgEx hdc:0x{:x} x:{} y:{} lpPoint:0x{:x}",
        hdc,
        x,
        y,
        lp_point
    );
    call.set_return(emu, 1);
}
