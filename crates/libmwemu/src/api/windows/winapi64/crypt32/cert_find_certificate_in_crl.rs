use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertFindCertificateInCRL(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let store = api_try!(emu, call.arg(emu, 0));
    let find_type = api_try!(emu, call.arg_u32(emu, 1));
    let find_para = api_try!(emu, call.arg(emu, 2));
    let prev = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CertFindCertificateInCRL store:0x{:x} type:0x{:x}",
        store,
        find_type
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("certfindcertificateincrl");
    call.set_return(emu, handle);
}
