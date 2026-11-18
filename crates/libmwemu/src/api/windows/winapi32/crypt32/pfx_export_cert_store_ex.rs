use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PFXExportCertStoreEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let blob = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!PFXExportCertStoreEx blob:0x{:x}", blob);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
