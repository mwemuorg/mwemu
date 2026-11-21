use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::*;

pub(super) fn BCryptExportKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let _h_key = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let _h_export_key = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    let psz_blob_type = api_try!(emu, call.arg_u32(emu, 2)) as u64;
    let pb_output = api_try!(emu, call.arg_u32(emu, 3)) as u64;
    let cb_output = api_try!(emu, call.arg_u32(emu, 4)) as usize;
    let pcb_result = api_try!(emu, call.arg_u32(emu, 5)) as u64;

    let blob_type = emu.maps.read_wide_string(psz_blob_type);
    log_red!(emu, "bcrypt!BCryptExportKey type={}", blob_type);

    if pb_output != 0 && cb_output > 0 {
        emu.maps.memset(pb_output, 0, cb_output.min(1024));
    }
    if pcb_result != 0 {
        emu.maps.write_dword(pcb_result, 0);
    }

    call.set_return(emu, STATUS_SUCCESS);
    call.cleanup_args(emu, 7);
}
