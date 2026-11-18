use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn AppPolicyGetProcessTerminationMethod(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let policy_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernelbase!AppPolicyGetProcessTerminationMethod (stub)"
    );
    if policy_ptr != 0 {
        emu.maps.write_dword(policy_ptr, 0);
    }
    call.set_return(emu, 0);
}
