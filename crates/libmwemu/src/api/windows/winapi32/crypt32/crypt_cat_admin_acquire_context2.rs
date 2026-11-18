use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminAcquireContext2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let ph_cat_admin = api_try!(emu, call.arg(emu, 0));
    let pg_subsystem = api_try!(emu, call.arg(emu, 1));
    let pwsz_hash_alg = api_try!(emu, call.arg(emu, 2));
    let p_strong_hash_policy = api_try!(emu, call.arg(emu, 3));
    let dw_flags = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptCATAdminAcquireContext2 0x{:x}",
        ph_cat_admin
    );
    let hndl = helper::handler_create("CryptCATAdminAcquireContext2");
    if ph_cat_admin != 0 {
        emu.maps.write_dword(ph_cat_admin, hndl as u32);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
