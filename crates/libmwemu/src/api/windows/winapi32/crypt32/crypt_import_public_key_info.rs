use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptImportPublicKeyInfo(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_crypt_prov = api_try!(emu, call.arg(emu, 0));
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 1));
    let p_info = api_try!(emu, call.arg(emu, 2));
    let ph_key = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!CryptImportPublicKeyInfo 0x{:x}", h_crypt_prov);
    let hndl = helper::handler_create("CryptImportPublicKeyInfo");
    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
