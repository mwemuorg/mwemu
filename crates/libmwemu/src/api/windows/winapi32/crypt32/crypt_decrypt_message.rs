use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptDecryptMessage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_decrypt_para = api_try!(emu, call.arg(emu, 0));
    let pb_encrypted_blob = api_try!(emu, call.arg(emu, 1));
    let cb_encrypted_blob = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "crypt32!CryptDecryptMessage p_decrypt_para:0x{:x} pb_encrypted_blob:0x{:x} cb_encrypted_blob:0x{:x}",
        p_decrypt_para,
        pb_encrypted_blob,
        cb_encrypted_blob
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
