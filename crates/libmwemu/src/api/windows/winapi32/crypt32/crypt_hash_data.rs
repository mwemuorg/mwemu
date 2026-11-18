use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptHashData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg(emu, 0));
    let pb_data = api_try!(emu, call.arg(emu, 1));
    let dw_data_len = api_try!(emu, call.arg_u32(emu, 2));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "crypt32!CryptHashData hash:0x{:x} data:0x{:x} len:{} flags:0x{:x}",
        h_hash,
        pb_data,
        dw_data_len,
        dw_flags
    );
    if pb_data != 0 && dw_data_len > 0 && dw_data_len < 0x100000 {
        let data = emu.maps.read_bytes(pb_data, dw_data_len as usize).to_vec();
        crypto_state::hash_data(h_hash, &data);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
