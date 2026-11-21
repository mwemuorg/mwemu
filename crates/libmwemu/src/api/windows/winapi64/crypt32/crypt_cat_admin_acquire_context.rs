use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminAcquireContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let cat_admin_ptr = api_try!(emu, call.arg(emu, 0));
    let subsystem_ptr = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "crypt32!CryptCATAdminAcquireContext");
    emu.set_last_error(0);
    let handle = helper::handler_create("cryptcatadminacquirecontext");
    if cat_admin_ptr != 0 {
        emu.maps.write_qword(cat_admin_ptr, handle);
    }
    call.set_return(emu, 1);
}
