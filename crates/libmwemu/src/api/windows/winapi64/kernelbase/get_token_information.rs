use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetTokenInformation(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let token_handle = api_try!(emu, call.arg(emu, 0));
    let token_information_class = api_try!(emu, call.arg(emu, 1));
    let token_information = api_try!(emu, call.arg(emu, 2));
    let token_information_length = api_try!(emu, call.arg(emu, 3));
    let return_length = api_try!(emu, call.arg(emu, 4));

    log_red!(
        emu,
        "kernelbase!GetTokenInformation token_information_class: 0x{:x}",
        token_information_class
    );

    call.set_return(emu, 1);
}
