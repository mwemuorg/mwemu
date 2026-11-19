use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CertSrvBackupRead(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hbc = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "crypt32!CertSrvBackupRead hbc:0x{:x}", hbc);
    let result = if helper::handler_exist(hbc) {
        0u64
    } else {
        1u64
    };
    emu.set_last_error(0);
    call.set_return(emu, result);
}
