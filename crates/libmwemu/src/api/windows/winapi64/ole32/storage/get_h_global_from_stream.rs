use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn GetHGlobalFromStream(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pstm = api_try!(emu, call.arg(emu, 0));
    let phglobal = api_try!(emu, call.arg(emu, 1));
    let hndl = helper::handler_create("ole32://HGlobal");
    if phglobal != 0 {
        emu.maps.write_dword(phglobal, hndl as u32);
    }
    log_red!(
        emu,
        "ole32!GetHGlobalFromStream pstm:0x{:x} => 0x{:x}",
        pstm,
        hndl
    );
    call.set_return(emu, 0); // S_OK
}
