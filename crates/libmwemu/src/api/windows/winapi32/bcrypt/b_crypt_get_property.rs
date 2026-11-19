use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptGetProperty(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_object = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let psz_property = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let pb_output = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let cb_output = api_try!(emu, call.arg_u32(emu, 3)) as usize;
    let pcb_result = api_try!(emu, call.arg_u32(emu, 4)) as u64;
    let _flags = api_try!(emu, call.arg_u32(emu, 5));

    let prop_name = emu.maps.read_wide_string(psz_property);

    let algo_uri = helper::handler_get_uri(h_object);
    let algo_name = algo_uri
        .strip_prefix("bcrypt://")
        .or_else(|| algo_uri.strip_prefix("bcrypthash://"))
        .unwrap_or(&algo_uri);

    let value: u32 = match prop_name.as_str() {
        "HashDigestLength" => match algo_name.to_uppercase().as_str() {
            "SHA256" => 32,
            "SHA1" => 20,
            "MD5" => 16,
            _ => 0,
        },
        "ObjectLength" => 128,
        "BlockLength" => match algo_name.to_uppercase().as_str() {
            "SHA256" => 64,
            "SHA1" => 64,
            "MD5" => 64,
            _ => 8,
        },
        _ => 0,
    };

    if pb_output != 0 && cb_output >= 4 {
        emu.maps.write_dword(pb_output, value);
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 4);
    }

    log_red!(
        emu,
        "bcrypt!BCryptGetProperty obj=0x{:x} prop={} => {}",
        h_object,
        prop_name,
        value
    );

    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 6);
}
