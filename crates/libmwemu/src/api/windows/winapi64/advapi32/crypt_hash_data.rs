use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptHashData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    let data_ptr = api_try!(emu, call.arg(emu, 1));
    let data_len = api_try!(emu, call.arg(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));

    let len = (data_len as usize).min(0x100_0000);
    if data_ptr != 0 && len > 0 {
        let data = emu.maps.read_bytes(data_ptr, len);
        crypto_state::hash_data(hhash, &data);
    }

    log_red!(
        emu,
        "advapi32!CryptHashData hHash=0x{:x} data=0x{:x} len={}",
        hhash,
        data_ptr,
        data_len
    );

    call.set_return(emu, 1);
}
