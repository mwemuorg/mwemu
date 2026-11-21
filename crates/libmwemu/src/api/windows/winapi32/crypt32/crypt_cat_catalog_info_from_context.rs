use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATCatalogInfoFromContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_info = api_try!(emu, call.arg(emu, 0));
    let p_cat_info = api_try!(emu, call.arg(emu, 1));
    let dw_flags = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATCatalogInfoFromContext h_cat_info:0x{:x} p_cat_info:0x{:x} dw_flags:0x{:x}",
        h_cat_info,
        p_cat_info,
        dw_flags
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
