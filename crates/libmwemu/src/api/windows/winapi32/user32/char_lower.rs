use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CharLowerA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "user32!CharLowerA arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 1);
}

pub(super) fn CharLowerW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ptr_str = api_try!(emu, call.arg(emu, 0));

    let s = emu.maps.read_wide_string(ptr_str);

    log_red!(emu, "user32!CharLowerW(`{}`)", s);

    emu.maps.write_wide_string(ptr_str, &s.to_lowercase());

    call.cleanup_args(emu, 1);
    call.set_return(emu, ptr_str);
}
