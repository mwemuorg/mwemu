use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAAdvertiseProvider(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let provider_id_ptr = api_try!(emu, call.arg(emu, 0));
    let routine_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "ws2_32!WSAAdvertiseProvider id:0x{:x} routine:0x{:x}",
        provider_id_ptr,
        routine_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
