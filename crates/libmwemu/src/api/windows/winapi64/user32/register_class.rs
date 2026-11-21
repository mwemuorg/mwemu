use super::register_class;
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn RegisterClassA(emu: &mut emu::Emu) {
    register_class_text(emu, "RegisterClassA", false);
}

pub(super) fn RegisterClassW(emu: &mut emu::Emu) {
    register_class_text(emu, "RegisterClassW", true);
}

fn register_class_text(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let lpwndclass = api_try!(emu, call.arg(emu, 0));
    let atom = 0xc000u64 + (lpwndclass & 0xfff);
    log_red!(
        emu,
        "user32!{api} wndclass:0x{:x} => atom:0x{:x}",
        lpwndclass,
        atom
    );
    emu.set_last_error(0);
    call.set_return(emu, atom);
    log_red!(emu, "** {} user32!{api} {}", emu.pos, lpwndclass);
    register_class(emu, lpwndclass, wide);
}
