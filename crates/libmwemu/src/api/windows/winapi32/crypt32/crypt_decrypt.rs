use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptDecrypt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));
    let h_hash = api_try!(emu, call.arg(emu, 1));
    let is_final = api_try!(emu, call.arg_u32(emu, 2));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 3));
    let pb_data = api_try!(emu, call.arg(emu, 4));
    let pdw_data_len = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "crypt32!CryptDecrypt key:0x{:x} hash:0x{:x} final:{} flags:0x{:x} data:0x{:x}",
        h_key,
        h_hash,
        is_final,
        dw_flags,
        pb_data
    );
    if pb_data != 0 && pdw_data_len != 0 {
        let data_len = emu.maps.read_dword(pdw_data_len).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(pb_data, len).to_vec();
            crypto_state::decrypt_data(h_key, &mut data);
            emu.maps.write_bytes(pb_data, &data);
        }
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
