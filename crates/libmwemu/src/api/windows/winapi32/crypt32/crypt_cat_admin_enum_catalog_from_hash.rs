use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminEnumCatalogFromHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_admin = api_try!(emu, call.arg(emu, 0));
    let pb_hash = api_try!(emu, call.arg(emu, 1));
    let cb_hash = api_try!(emu, call.arg(emu, 2));
    let dw_flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptCATAdminEnumCatalogFromHash 0x{:x}",
        h_cat_admin
    );
    let hndl = helper::handler_create("CryptCATAdminEnumCatalogFromHash");
    call.set_return(emu, hndl);
    call.cleanup_args(emu, 5);
}
