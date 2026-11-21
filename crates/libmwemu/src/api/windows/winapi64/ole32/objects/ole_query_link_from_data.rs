use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn OleQueryLinkFromData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pdata_src = api_try!(emu, call.arg(emu, 0));
    log_red!(
        emu,
        "ole32!OleQueryLinkFromData pSrcDataObj:0x{:x}",
        pdata_src
    );
    if pdata_src != 0 && emu.maps.is_mapped(pdata_src) {
        let _ = emu.maps.read_qword(pdata_src).unwrap_or(0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
