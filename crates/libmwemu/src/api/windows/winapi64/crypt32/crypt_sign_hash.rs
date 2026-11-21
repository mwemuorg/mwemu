use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptSignHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg(emu, 0));
    let key_spec = api_try!(emu, call.arg_u32(emu, 1));
    let _desc_ptr = api_try!(emu, call.arg(emu, 2));
    let flags = api_try!(emu, call.arg_u32(emu, 3));
    let sig_ptr = api_try!(emu, call.arg(emu, 4));
    let sig_len_ptr = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "crypt32!CryptSignHash hash:0x{:x} spec:{} flags:0x{:x}",
        h_hash,
        key_spec,
        flags
    );
    let sig_size = 128usize;
    if let Some(signature) = crypto_state::sign_hash(h_hash, sig_size) {
        if sig_ptr == 0 {
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, signature.len() as u32);
            }
        } else {
            let write_len = if sig_len_ptr != 0 {
                let buf_sz = emu.maps.read_dword(sig_len_ptr).unwrap_or(0) as usize;
                signature.len().min(buf_sz)
            } else {
                signature.len()
            };
            emu.maps.write_bytes(sig_ptr, &signature[..write_len]);
            if sig_len_ptr != 0 {
                emu.maps.write_dword(sig_len_ptr, write_len as u32);
            }
        }
    } else if sig_ptr == 0 {
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, sig_size as u32);
        }
    } else {
        for i in 0..sig_size as u64 {
            emu.maps.write_byte(sig_ptr + i, 0xff);
        }
        if sig_len_ptr != 0 {
            emu.maps.write_dword(sig_len_ptr, sig_size as u32);
        }
    }
    call.set_return(emu, 1);
}
