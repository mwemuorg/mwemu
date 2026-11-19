use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn WSCInstallProvider64_32(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    let arg5 = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "ws2_32!WSCInstallProvider64_32 arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let out_ptr = api_try!(emu, call.arg(emu, 4));
    let handle = helper::handler_create("WSCInstallProvider64_32");
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, handle as u32);
    }
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
