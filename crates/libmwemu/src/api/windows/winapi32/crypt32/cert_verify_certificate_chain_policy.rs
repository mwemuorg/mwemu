use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertVerifyCertificateChainPolicy(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let policy_oid = api_try!(emu, call.arg(emu, 0));
    let chain_ctx = api_try!(emu, call.arg(emu, 1));
    let policy_para = api_try!(emu, call.arg(emu, 2));
    let policy_status = api_try!(emu, call.arg(emu, 3));
    if policy_status != 0 {
        emu.maps.write_dword(policy_status, 0); // cbSize
        emu.maps.write_dword(policy_status + 4, 0); // dwError = 0 (valid)
    }
    log_red!(
        emu,
        "crypt32!CertVerifyCertificateChainPolicy chain:0x{:x} => valid",
        chain_ctx
    );
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
