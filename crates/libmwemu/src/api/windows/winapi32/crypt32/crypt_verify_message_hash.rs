use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptVerifyMessageHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_hash_para = api_try!(emu, call.arg(emu, 0));
    let pb_hashed_blob = api_try!(emu, call.arg(emu, 1));
    let cb_hashed_blob = api_try!(emu, call.arg(emu, 2));
    let pb_to_be_hashed = api_try!(emu, call.arg(emu, 3));
    let pcb_to_be_hashed = api_try!(emu, call.arg(emu, 4));
    log_red!(emu, "crypt32!CryptVerifyMessageHash 0x{:x}", p_hash_para);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 7);
}
