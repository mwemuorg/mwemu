use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSetKeyParam(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let param = api_try!(emu, call.arg_u32(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "advapi32!CryptSetKeyParam key:0x{:x} param:0x{:x} data:0x{:x}",
        hkey,
        param,
        data_ptr
    );
    let mut data = Vec::new();
    if data_ptr != 0 && emu.maps.is_mapped(data_ptr) {
        for i in 0..16u64 {
            if let Some(b) = emu.maps.read_byte(data_ptr + i) {
                data.push(b);
            } else {
                break;
            }
        }
    }
    crypto_state::set_key_param(hkey, param, &data);
    call.cleanup_args(emu, 4);
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
