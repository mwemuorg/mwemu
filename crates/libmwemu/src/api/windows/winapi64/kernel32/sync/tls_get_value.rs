use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub fn TlsGetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg(emu, 0)) as usize; // Parameter passed in RCX in x64

    let val = if idx < emu.tls64().len() {
        emu.set_last_error(ERROR_SUCCESS);
        emu.tls64()[idx]
    } else {
        emu.set_last_error(ERROR_INVALID_PARAMETER);
        0
    };

    call.set_return(emu, val);

    log_red!(
        emu,
        "** {} kernel32!TlsGetValue idx: {} =0x{:x}",
        emu.pos,
        idx,
        val
    );
}
