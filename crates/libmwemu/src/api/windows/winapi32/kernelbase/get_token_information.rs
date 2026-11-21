use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn GetTokenInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token_information_class = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "kernelbase!GetTokenInformation class: 0x{:x}",
        token_information_class
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
