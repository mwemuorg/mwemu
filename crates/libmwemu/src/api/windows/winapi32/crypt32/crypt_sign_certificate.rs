use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSignCertificate(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_crypt_prov = api_try!(emu, call.arg(emu, 0));
    let dw_key_spec = api_try!(emu, call.arg(emu, 1));
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 2));
    let pb_encoded_to_be_signed = api_try!(emu, call.arg(emu, 3));
    let cb_encoded_to_be_signed = api_try!(emu, call.arg(emu, 4));
    let p_sign_algorithm = api_try!(emu, call.arg(emu, 5));
    let pv_hash_aux_info = api_try!(emu, call.arg(emu, 6));
    let pb_signature = api_try!(emu, call.arg(emu, 7));
    log_red!(emu, "crypt32!CryptSignCertificate 0x{:x}", h_crypt_prov);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 9);
}
