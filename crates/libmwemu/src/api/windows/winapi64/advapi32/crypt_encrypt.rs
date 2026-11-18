use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptEncrypt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let _hhash = api_try!(emu, call.arg(emu, 1));
    let _bfinal = api_try!(emu, call.arg(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let data_len_ptr = api_try!(emu, call.arg(emu, 5));
    let _buff_len = api_try!(emu, call.arg(emu, 6));

    if data_ptr != 0 && data_len_ptr != 0 {
        let data_len = emu.maps.read_dword(data_len_ptr).unwrap_or(0) as usize;
        let len = data_len.min(0x100_0000);
        if len > 0 {
            let mut data = emu.maps.read_bytes(data_ptr, len).to_vec();
            crypto_state::encrypt_data(hkey, &mut data);
            emu.maps.write_bytes(data_ptr, &data);
        }
    }

    log_red!(emu, "advapi32!CryptEncrypt hKey=0x{:x}", hkey);

    call.set_return(emu, 1);
}
