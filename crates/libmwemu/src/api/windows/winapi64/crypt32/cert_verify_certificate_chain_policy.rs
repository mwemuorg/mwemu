use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CertVerifyCertificateChainPolicy(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _policy_oid = api_try!(emu, call.arg(emu, 0));
    let _p_chain_context = api_try!(emu, call.arg(emu, 1));
    let _p_policy_para = api_try!(emu, call.arg(emu, 2));
    let p_policy_status = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "crypt32!CertVerifyCertificateChainPolicy status_out:0x{:x}",
        p_policy_status
    );

    if p_policy_status != 0 {
        emu.maps.write_dword(p_policy_status, 4);
        emu.maps.write_dword(p_policy_status + 4, 0);
        emu.maps.write_dword(p_policy_status + 8, 0);
        emu.maps.write_dword(p_policy_status + 12, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
