use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptExportKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hkey = api_try!(emu, call.arg(emu, 0));
    let _hexp_key = api_try!(emu, call.arg(emu, 1));
    let blob_type = api_try!(emu, call.arg_u32(emu, 2));
    let _flags = api_try!(emu, call.arg(emu, 3));
    let data_ptr = api_try!(emu, call.arg(emu, 4));
    let data_len_ptr = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "advapi32!CryptExportKey key: 0x{:x} type: {}",
        hkey,
        blob_type
    );
    let key_data = crypto_state::export_key(hkey);
    let blob_len = match &key_data {
        Some(d) => 12 + d.len() as u32,
        None => 12 + 16,
    };
    if data_ptr == 0 && data_len_ptr != 0 {
        emu.maps.write_dword(data_len_ptr, blob_len);
    } else if data_len_ptr != 0 {
        emu.maps.write_byte(data_ptr, blob_type as u8);
        emu.maps.write_byte(data_ptr + 1, 2);
        emu.maps.write_word(data_ptr + 2, 0);
        emu.maps.write_dword(data_ptr + 4, 0x00006801);
        emu.maps.write_dword(data_ptr + 8, 0);
        if let Some(ref kd) = key_data {
            for (i, b) in kd.iter().enumerate() {
                emu.maps.write_byte(data_ptr + 12 + i as u64, *b);
            }
        }
        emu.maps.write_dword(data_len_ptr, blob_len);
    }
    call.set_return(emu, 1);
}
