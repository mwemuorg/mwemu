use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn CryptHashData(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hhash = api_try!(emu, call.arg(emu, 0));
    let data_ptr = api_try!(emu, call.arg(emu, 1));
    let data_len = api_try!(emu, call.arg(emu, 2)) as usize;
    let flags = api_try!(emu, call.arg_u32(emu, 3));

    let len = data_len.min(0x100_0000);
    if data_ptr != 0 && len > 0 {
        let data = emu.maps.read_bytes(data_ptr, len);
        crypto_state::hash_data(hhash, &data);
    }

    log_red!(
        emu,
        "advapi32!CryptHashData hHash=0x{:x} len={}",
        hhash,
        data_len
    );

    call.cleanup_args(emu, 4);
    call.set_return(emu, 1);
}
