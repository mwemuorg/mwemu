use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminAddCatalog(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let admin = api_try!(emu, call.arg(emu, 0));
    let catalog_file = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "crypt32!CryptCATAdminAddCatalog admin:0x{:x}", admin);
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptcatadmincatalog");
    call.set_return(emu, handle);
}
