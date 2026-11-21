use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn CryptGetHashParam(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    let param = api_try!(emu, call.arg_u32(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let len_ptr = api_try!(emu, call.arg(emu, 3));
    let flags = api_try!(emu, call.arg_u32(emu, 4));

    log_red!(
        emu,
        "advapi32!CryptGetHashParam hash:0x{:x} param:0x{:x}",
        hhash,
        param
    );

    if let Some(result) = crypto_state::get_hash_param(hhash, param) {
        if data_ptr != 0 {
            let buf_sz = if len_ptr != 0 {
                emu.maps.read_dword(len_ptr).unwrap_or(0) as usize
            } else {
                result.len()
            };
            let write_len = result.len().min(buf_sz);
            emu.maps.write_bytes(data_ptr, &result[..write_len]);
        }
        if len_ptr != 0 {
            emu.maps.write_dword(len_ptr, result.len() as u32);
        }
        call.set_return(emu, 1);
    } else {
        call.set_return(emu, 0);
    }

    call.cleanup_args(emu, 5);
}
