use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn VariantChangeType(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dst = api_try!(emu, call.arg(emu, 0));
    let src = api_try!(emu, call.arg(emu, 1));
    let _lcid = api_try!(emu, call.arg(emu, 2));
    let vt_new = api_try!(emu, call.arg(emu, 3));
    if dst != 0 {
        emu.maps.write_word(dst, vt_new as u16);
    }
    log_red!(
        emu,
        "oleaut32!VariantChangeType vt:{} dst:0x{:x}",
        vt_new,
        dst
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
