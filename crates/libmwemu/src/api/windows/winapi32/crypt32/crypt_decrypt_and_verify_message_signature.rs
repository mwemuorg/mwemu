use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptDecryptAndVerifyMessageSignature(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_decrypt_para = api_try!(emu, call.arg(emu, 0));
    let p_verify_para = api_try!(emu, call.arg(emu, 1));
    let dw_signer_index = api_try!(emu, call.arg(emu, 2));
    let pb_encrypted_blob = api_try!(emu, call.arg(emu, 3));
    let cb_encrypted_blob = api_try!(emu, call.arg(emu, 4));
    let ppb_decrypted = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "crypt32!CryptDecryptAndVerifyMessageSignature 0x{:x}",
        p_decrypt_para
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 9);
}
