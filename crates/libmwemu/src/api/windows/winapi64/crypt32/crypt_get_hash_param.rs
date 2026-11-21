use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub(super) fn CryptGetHashParam(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_hash = api_try!(emu, call.arg(emu, 0));
    let param = api_try!(emu, call.arg_u32(emu, 1));
    let data_ptr = api_try!(emu, call.arg(emu, 2));
    let data_len_ptr = api_try!(emu, call.arg(emu, 3));

    log_red!(
        emu,
        "crypt32!CryptGetHashParam hash: 0x{:x} param: {} data: 0x{:x}",
        h_hash,
        param,
        data_ptr
    );

    match param {
        1 => {
            // HP_ALGID — return algorithm ID
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, 0x8004); // CALG_SHA1 as default
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 4);
            }
            call.set_return(emu, 1);
        }
        2 => {
            // HP_HASHSIZE — return hash size
            if data_ptr != 0 {
                emu.maps.write_dword(data_ptr, 20); // SHA1 = 20 bytes
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, 4);
            }
            call.set_return(emu, 1);
        }
        3 => {
            // HP_HASHVAL — return the hash value
            let hash_size: u32 = 20; // SHA1 default
            if data_ptr != 0 {
                // Write deterministic hash bytes
                for i in 0..hash_size {
                    let byte = ((h_hash as u32).wrapping_mul(31).wrapping_add(i)) as u8;
                    emu.maps.write_byte(data_ptr + i as u64, byte);
                }
            }
            if data_len_ptr != 0 {
                emu.maps.write_dword(data_len_ptr, hash_size);
            }
            call.set_return(emu, 1);
        }
        _ => {
            log_red!(emu, "  unsupported param {}", param);
            call.set_return(emu, 0);
        }
    }
}
