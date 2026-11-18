use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_prov = api_try!(emu, call.arg(emu, 0));
    let algid = api_try!(emu, call.arg_u32(emu, 1));
    let h_key = api_try!(emu, call.arg(emu, 2));
    let _flags = api_try!(emu, call.arg_u32(emu, 3));
    let h_hash_ptr = api_try!(emu, call.arg(emu, 4));
    let alg_name = match algid {
        0x8001 => "MD2",
        0x8002 => "MD4",
        0x8003 => "MD5",
        0x8004 => "SHA1",
        0x800C => "SHA256",
        0x800D => "SHA384",
        0x800E => "SHA512",
        _ => "UNKNOWN",
    };
    call.cleanup_args(emu, 5);
    log_red!(
        emu,
        "crypt32!CryptCreateHash prov: 0x{:x} alg: 0x{:x} ({}) key: 0x{:x}",
        h_prov,
        algid,
        alg_name,
        h_key
    );
    let h_hash = helper::handler_create(&format!("hash:{}", alg_name));
    crypto_state::create_hash(h_hash, algid);
    if h_hash_ptr != 0 {
        emu.maps.write_dword(h_hash_ptr, h_hash as u32);
    }
    call.set_return(emu, 1); // TRUE
}
