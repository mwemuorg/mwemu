use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn StrCmpCA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let str1_ptr = api_try!(emu, call.arg(emu, 0));
    let str2_ptr = api_try!(emu, call.arg(emu, 1));

    call.cleanup_args(emu, 2);

    if str1_ptr == 0 || str2_ptr == 0 {
        call.set_return(emu, 0xffffffff);
        log_red!(emu, "msvcrt!StrCmpA null ptrs");
        return;
    }

    let str1 = emu.maps.read_string(str1_ptr);
    let str2 = emu.maps.read_string(str2_ptr);

    log_red!(emu, "msvcrt!StrCmpA {} == {}", str1, str2);

    if str1 == str2 {
        call.set_return(emu, 0);
    } else {
        call.set_return(emu, 0xffffffff);
    }
}
