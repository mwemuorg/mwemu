use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn ConvertToAutoInheritPrivateObjectSecurity(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "advapi32!ConvertToAutoInheritPrivateObjectSecurity arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let src = api_try!(emu, call.arg(emu, 0));
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }
    for _ in 0..2 {}
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
