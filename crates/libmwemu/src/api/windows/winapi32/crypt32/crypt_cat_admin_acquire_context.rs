use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminAcquireContext(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ph_cat_admin = api_try!(emu, call.arg(emu, 0));
    let pg_subsystem = api_try!(emu, call.arg(emu, 1));
    let dw_flags = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptCATAdminAcquireContext ph_cat_admin:0x{:x} pg_subsystem:0x{:x} dw_flags:0x{:x}",
        ph_cat_admin,
        pg_subsystem,
        dw_flags
    );
    let hndl = helper::handler_create("CryptCATAdminAcquireContext");
    if ph_cat_admin != 0 {
        emu.maps.write_dword(ph_cat_admin, hndl as u32);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 3);
}
