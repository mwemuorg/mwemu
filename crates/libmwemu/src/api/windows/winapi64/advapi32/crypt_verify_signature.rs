use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn CryptVerifySignatureA(emu: &mut emu::Emu) {
    crypt_verify_signature(emu, "CryptVerifySignatureA", false);
}

fn crypt_verify_signature(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    let sig_ptr = api_try!(emu, call.arg(emu, 1));
    let sig_len = api_try!(emu, call.arg(emu, 2));
    let hpub_key = api_try!(emu, call.arg(emu, 3));
    let desc_ptr = api_try!(emu, call.arg(emu, 4));
    let _flags = api_try!(emu, call.arg(emu, 5));
    let desc = if desc_ptr != 0 {
        strings::read_text(emu, desc_ptr, wide)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "advapi32!{api} hash:0x{:x} sig:0x{:x} len:{} key:0x{:x} desc:'{}'",
        hhash,
        sig_ptr,
        sig_len,
        hpub_key,
        desc
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}

pub fn CryptVerifySignatureW(emu: &mut emu::Emu) {
    crypt_verify_signature(emu, "CryptVerifySignatureW", true);
}
