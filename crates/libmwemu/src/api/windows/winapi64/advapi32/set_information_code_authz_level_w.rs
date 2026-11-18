use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SetInformationCodeAuthzLevelW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_wide_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!SetInformationCodeAuthzLevelW '{}'  p1:0x{:x} p2:0x{:x}",
        s,
        p1,
        p2
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
