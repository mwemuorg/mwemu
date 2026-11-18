use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminReleaseCatalogContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_admin = api_try!(emu, call.arg(emu, 0));
    let h_cat_info = api_try!(emu, call.arg(emu, 1));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATAdminReleaseCatalogContext admin:0x{:x} info:0x{:x} flags:0x{:x}",
        h_cat_admin,
        h_cat_info,
        dw_flags
    );
    if h_cat_info != 0 {
        helper::handler_close(h_cat_info);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
