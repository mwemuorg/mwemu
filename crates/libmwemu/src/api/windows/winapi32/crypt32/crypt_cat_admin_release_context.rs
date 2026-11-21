use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminReleaseContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_admin = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "crypt32!CryptCATAdminReleaseContext h_cat_admin:0x{:x} dw_flags:0x{:x}",
        h_cat_admin,
        dw_flags
    );
    let h = api_try!(emu, call.arg(emu, 0));
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 2);
}
