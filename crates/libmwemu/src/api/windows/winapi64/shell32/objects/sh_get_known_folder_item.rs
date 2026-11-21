use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHGetKnownFolderItem(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rfid = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let htoken = api_try!(emu, call.arg(emu, 2));
    let riid = api_try!(emu, call.arg(emu, 3));
    let ppv = api_try!(emu, call.arg(emu, 4));
    let hndl = helper::handler_create("IShellItem://knownfolder");
    if ppv != 0 {
        emu.maps.write_qword(ppv, hndl);
    }
    log_red!(
        emu,
        "shell32!SHGetKnownFolderItem flags:0x{:x} =0x{:x}",
        flags,
        hndl
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
