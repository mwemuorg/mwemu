use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn SHBrowseForFolder(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "shell32!SHBrowseForFolder arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shbrowseforfolder");
    emu.set_last_error(0);
    call.set_return(emu, handle);
}

pub fn SHBrowseForFolderA(emu: &mut emu::Emu) {
    sh_browse_for_folder(emu, "SHBrowseForFolderA", false);
}

pub fn SHBrowseForFolderW(emu: &mut emu::Emu) {
    sh_browse_for_folder(emu, "SHBrowseForFolderW", true);
}

fn sh_browse_for_folder(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    if wide {
        let handle = helper::handler_create("shbrowseforfolderw");
        emu.set_last_error(0);
        call.set_return(emu, handle);
    } else {
        let handle = helper::handler_create("shbrowseforfoldera");
        emu.set_last_error(0);
        call.set_return(emu, handle);
    }
}
