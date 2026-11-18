use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertEnumCertificatesInStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store = api_try!(emu, call.arg(emu, 0));
    let prev_ctx = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "crypt32!CertEnumCertificatesInStore store:0x{:x} prev:0x{:x}",
        store,
        prev_ctx
    );
    emu.set_last_error(0);
    call.set_return(emu, 0);
}
