use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminCalcHashFromFileHandle2(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_cat_admin = api_try!(emu, call.arg(emu, 0));
    let h_file = api_try!(emu, call.arg(emu, 1));
    let pcb_hash = api_try!(emu, call.arg(emu, 2));
    let pb_hash = api_try!(emu, call.arg(emu, 3));
    let dw_flags = api_try!(emu, call.arg(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle2 0x{:x}",
        h_cat_admin
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
