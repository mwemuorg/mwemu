use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn ChangeServiceConfigA(emu: &mut emu::Emu) {
    change_service_config(emu, "ChangeServiceConfigA", false);
}

fn change_service_config(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let p4 = api_try!(emu, call.arg(emu, 4));
    let p5 = api_try!(emu, call.arg(emu, 5));
    let p6 = api_try!(emu, call.arg(emu, 6));
    let p7 = api_try!(emu, call.arg(emu, 7));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "advapi32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );
    call.cleanup_args(emu, 11);
    call.set_return(emu, 1);
}

pub fn ChangeServiceConfigW(emu: &mut emu::Emu) {
    change_service_config(emu, "ChangeServiceConfigW", true);
}
