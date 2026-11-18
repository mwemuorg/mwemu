use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn InternetErrorDlg(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let err = api_try!(emu, call.arg(emu, 0));

    log_red!(emu, "wininet!InternetErrorDlg err: {}", err);

    call.set_return(emu, 0);
}
