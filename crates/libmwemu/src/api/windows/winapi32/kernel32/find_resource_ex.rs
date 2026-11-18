use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindResourceExA(emu: &mut emu::Emu) {
    find_resource_ex(emu, "FindResourceExA", false);
}

fn find_resource_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let p2 = api_try!(emu, call.arg(emu, 2));
    let p3 = api_try!(emu, call.arg(emu, 3));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    let hndl = helper::handler_create("resource");
    log_red!(
        emu,
        "kernel32!{api} '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
        s,
        p1,
        p2,
        p3,
        hndl
    );
    call.cleanup_args(emu, 4);
    call.set_return(emu, hndl);
}

pub fn FindResourceExW(emu: &mut emu::Emu) {
    find_resource_ex(emu, "FindResourceExW", true);
}
