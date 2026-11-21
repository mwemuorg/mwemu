use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CreateStorageItemFromShellItem_FullTrustCaller_ForPackage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg_u32(emu, 1));
    log_red!(
        emu,
        "shell32!CreateStorageItemFromShellItem_FullTrustCaller_ForPackage arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle =
        helper::handler_create("createstorageitemfromshellitem_fulltrustcaller_forpackage");
    call.set_return(emu, handle);
    call.cleanup_args(emu, 2);
}
