use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn OleGetPackageClipboardOwner(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ppunk = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ole32!OleGetPackageClipboardOwner ppUnk:0x{:x}", ppunk);
    if ppunk != 0 {
        emu.maps.write_qword(ppunk, 0);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
