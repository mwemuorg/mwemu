use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptDecryptMessage(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_decrypt_para = api_try!(emu, call.arg(emu, 0));
    let pb_encrypted_blob = api_try!(emu, call.arg(emu, 1));
    let cb_encrypted_blob = api_try!(emu, call.arg_u32(emu, 2));
    let pb_decrypted = api_try!(emu, call.arg(emu, 3));
    let pcb_decrypted = api_try!(emu, call.arg(emu, 4));
    let pp_xchg_cert = api_try!(emu, call.arg(emu, 5));

    let blob_data = if pb_encrypted_blob != 0 && cb_encrypted_blob > 0 {
        let len = (cb_encrypted_blob as usize).min(256);
        emu.maps.read_bytes(pb_encrypted_blob, len).to_vec()
    } else {
        Vec::new()
    };

    if pb_decrypted != 0 && !blob_data.is_empty() {
        emu.maps.write_bytes(pb_decrypted, &blob_data);
    }
    if pcb_decrypted != 0 {
        emu.maps.write_dword(pcb_decrypted, blob_data.len() as u32);
    }

    log_red!(
        emu,
        "crypt32!CryptDecryptMessage blob_sz:{} out:0x{:x}",
        cb_encrypted_blob,
        pb_decrypted
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
