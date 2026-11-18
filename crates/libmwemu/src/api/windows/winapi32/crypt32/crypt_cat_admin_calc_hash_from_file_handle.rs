use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptCATAdminCalcHashFromFileHandle(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_file = api_try!(emu, call.arg(emu, 0));
    let pcb_hash = api_try!(emu, call.arg(emu, 1));
    let pb_hash = api_try!(emu, call.arg(emu, 2));
    let dw_flags = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptCATAdminCalcHashFromFileHandle 0x{:x}",
        h_file
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
