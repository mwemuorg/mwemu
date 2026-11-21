use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSANSPIoctl(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let lookup_handle = api_try!(emu, call.arg(emu, 0));
    let control_code = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "ws2_32!WSANSPIoctl handle:0x{:x} code:0x{:x}",
        lookup_handle,
        control_code
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
