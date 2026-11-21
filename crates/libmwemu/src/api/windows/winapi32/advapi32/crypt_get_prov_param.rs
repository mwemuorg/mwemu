use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptGetProvParam(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _hprov = api_try!(emu, call.arg(emu, 0));
    let param = api_try!(emu, call.arg_u32(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let data_len_ptr = api_try!(emu, call.arg(emu, 3));
    let _flags = api_try!(emu, call.arg_u32(emu, 4));
    log_red!(emu, "advapi32!CryptGetProvParam param:{}", param);
    match param {
        4 => {
            let name = "Microsoft Base Cryptographic Provider v1.0";
            if data_ptr != 0 {
                emu.maps.write_string(data_ptr, name);
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, name.len() as u32 + 1);
            }
        }
        6 => {
            let name = "default";
            if data_ptr != 0 {
                emu.maps.write_string(data_ptr, name);
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, name.len() as u32 + 1);
            }
        }
        _ => {
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, 0);
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 4);
            }
        }
    }
    call.cleanup_args(emu, 5);
    call.set_return(emu, 1);
}
