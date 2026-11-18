use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptDecrypt(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let pb_input = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let cb_input = api_try!(emu, call.arg_u32(emu, 2)) as usize;
    let _pb_padding = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let _pb_iv = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let _cb_iv = api_try!(emu, call.arg_u32(emu, 5)) as usize;
    let pb_output = api_try!(emu, call.arg_u32(emu, 6)) as u64;
    let cb_output = api_try!(emu, call.arg_u32(emu, 7)) as usize;
    let pcb_result = api_try!(emu, call.arg_u32(emu, 8)) as u64;
    let _flags = api_try!(emu, call.arg_u32(emu, 9));

    log_red!(
        emu,
        "bcrypt!BCryptDecrypt key=0x{:x} in_len={} out_buf=0x{:x}",
        h_key,
        cb_input,
        pb_output
    );

    if pb_input != 0 && cb_input > 0 && pb_output != 0 && cb_output >= cb_input {
        let mut data = emu
            .maps
            .read_bytes(pb_input, cb_input.min(0x100_0000))
            .to_vec();
        crypto_state::decrypt_data(h_key, &mut data);
        emu.maps.write_bytes(pb_output, &data);
        if pcb_result != 0 {
            emu.maps.write_dword(pcb_result, data.len() as u32);
        }
    } else if pb_output == 0 && pcb_result != 0 {
        emu.maps.write_dword(pcb_result, cb_input as u32);
    }

    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 10);
}
