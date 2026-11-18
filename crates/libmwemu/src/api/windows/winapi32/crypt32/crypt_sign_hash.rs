use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSignHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg(emu, 0));
    let key_spec = api_try!(emu, call.arg_u32(emu, 1));
    let _desc_ptr = api_try!(emu, call.arg_u32(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));
    let sig_ptr = api_try!(emu, call.arg(emu, 4));
    let sig_len_ptr = api_try!(emu, call.arg(emu, 5));
    call.cleanup_args(emu, 6);
    log_red!(
        emu,
        "crypt32!CryptSignHash hash: 0x{:x} spec: {}",
        h_hash,
        key_spec
    );
    if sig_ptr == 0 {
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, 128);
        }
    } else {
        for i in 0..128u64 {
            emu.maps.write_byte(sig_ptr + i, 0xAA);
        }
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, 128);
        }
    }
    call.set_return(emu, 1); // TRUE
}
