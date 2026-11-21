use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptUIDlgViewSignerInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let psi = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "crypt32!CryptUIDlgViewSignerInfo pSignerInfo:0x{:x}",
        psi
    );
    if psi != 0 && emu.maps.is_mapped(psi) {
        let _ = emu.maps.read_dword(psi).unwrap_or(0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
