use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptDecryptAndVerifyMessageSignature(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let p_decrypt_para = api_try!(emu, call.arg(emu, 0));
    let p_verify_para = api_try!(emu, call.arg(emu, 1));
    let dw_signer_index = api_try!(emu, call.arg_u32(emu, 2));
    let pb_encrypted_blob = api_try!(emu, call.arg(emu, 3));
    let cb_encrypted_blob = api_try!(emu, call.arg_u32(emu, 4));
    let ppb_decrypted = api_try!(emu, call.arg(emu, 5));
    let pcb_decrypted = api_try!(emu, call.arg(emu, 6));
    let pp_xchg_cert = api_try!(emu, call.arg(emu, 7));
    let pp_signer_cert = api_try!(emu, call.arg(emu, 8));

    let blob_data = if pb_encrypted_blob != 0 && cb_encrypted_blob > 0 {
        let len = (cb_encrypted_blob as usize).min(256);
        emu.maps.read_bytes(pb_encrypted_blob, len).to_vec()
    } else {
        Vec::new()
    };

    if ppb_decrypted != 0 && !blob_data.is_empty() {
        let out_ptr = emu.maps.alloc(blob_data.len() as u64).expect("alloc");
        emu.maps.write_bytes(out_ptr, &blob_data);
        emu.maps.write_qword(ppb_decrypted, out_ptr);
        if pcb_decrypted != 0 {
            emu.maps.write_dword(pcb_decrypted, blob_data.len() as u32);
        }
    }

    log_red!(
        emu,
        "crypt32!CryptDecryptAndVerifyMessageSignature blob_sz:{}",
        cb_encrypted_blob
    );
    emu.set_last_error(0);
    call.set_return(emu, 1);
}
