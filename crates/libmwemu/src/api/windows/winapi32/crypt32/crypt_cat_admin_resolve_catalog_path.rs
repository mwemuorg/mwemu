use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminResolveCatalogPath(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_admin = api_try!(emu, call.arg(emu, 0));
    let pwsz_catalog_file = api_try!(emu, call.arg(emu, 1));
    let p_resolve_info = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATAdminResolveCatalogPath h_cat_admin:0x{:x} pwsz_catalog_file:0x{:x} p_resolve_info:0x{:x}",
        h_cat_admin,
        pwsz_catalog_file,
        p_resolve_info
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
