use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptGetKeyParam(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));
    let param = api_try!(emu, call.arg_u32(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let data_len_ptr = api_try!(emu, call.arg(emu, 3));
    let _flags = api_try!(emu, call.arg_u32(emu, 4));
    log_red!(
        emu,
        "crypt32!CryptGetKeyParam key:0x{:x} param:{} data:0x{:x}",
        h_key,
        param,
        data_ptr
    );
    if let Some(result) = crypto_state::get_key_param(h_key, param) {
        if data_ptr != 0 {
            emu.maps.write_bytes(data_ptr, &result);
        }
        if data_len_ptr != 0 {
            emu.maps.write_dword(data_len_ptr, result.len() as u32);
        }
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 5);
}
