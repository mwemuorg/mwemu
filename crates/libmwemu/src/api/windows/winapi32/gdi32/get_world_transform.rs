use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetWorldTransform(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdc = api_try!(emu, call.arg_u32(emu, 0));
    let xform = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    if xform != 0 {
        // Identity matrix: eM11=1.0, eM12=0, eM21=0, eM22=1.0, eDx=0, eDy=0
        emu.maps.write_dword(xform, 0x3f80_0000); // 1.0f
        emu.maps.write_dword(xform + 4, 0);
        emu.maps.write_dword(xform + 8, 0);
        emu.maps.write_dword(xform + 12, 0x3f80_0000); // 1.0f
        emu.maps.write_dword(xform + 16, 0);
        emu.maps.write_dword(xform + 20, 0);
    }

    log_red!(
        emu,
        "gdi32!GetWorldTransform hdc:0x{:x} xform:0x{:x}",
        hdc,
        xform
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
