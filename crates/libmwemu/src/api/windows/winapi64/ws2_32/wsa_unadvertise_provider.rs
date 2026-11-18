use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn WSAUnadvertiseProvider(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let provider_id_ptr = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "ws2_32!WSAUnadvertiseProvider id:0x{:x}",
        provider_id_ptr
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
