use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptExportPublicKeyInfoEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_crypt_prov = api_try!(emu, call.arg(emu, 0));
    let dw_key_spec = api_try!(emu, call.arg(emu, 1));
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 2));
    let psz_public_key_obj_id = api_try!(emu, call.arg(emu, 3));
    let dw_flags = api_try!(emu, call.arg(emu, 4));
    let pv_aux_info = api_try!(emu, call.arg(emu, 5));
    let p_info = api_try!(emu, call.arg(emu, 6));
    log_red!(
        emu,
        "crypt32!CryptExportPublicKeyInfoEx 0x{:x}",
        h_crypt_prov
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 8);
}
