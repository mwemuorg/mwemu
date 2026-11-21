use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminAddCatalog(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_admin = api_try!(emu, call.arg(emu, 0));
    let pwsz_catalog_file = api_try!(emu, call.arg(emu, 1));
    let pwsz_select_base_name = api_try!(emu, call.arg(emu, 2));
    let dw_flags = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!CryptCATAdminAddCatalog 0x{:x}", h_cat_admin);
    let hndl = helper::handler_create("CryptCATAdminAddCatalog");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 4);
}
