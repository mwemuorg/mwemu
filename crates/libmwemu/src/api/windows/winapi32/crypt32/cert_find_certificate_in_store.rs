use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertFindCertificateInStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_store = api_try!(emu, call.arg_u32(emu, 0));
    let _encoding_type = api_try!(emu, call.arg_u32(emu, 1));
    let _find_flags = api_try!(emu, call.arg_u32(emu, 2));
    let find_type = api_try!(emu, call.arg_u32(emu, 3));

    log_red!(
        emu,
        "crypt32!CertFindCertificateInStore store: 0x{:x} find_type: 0x{:x}",
        h_store,
        find_type
    );

    call.set_return(emu, 0);
    call.cleanup_args(emu, 6);
}
