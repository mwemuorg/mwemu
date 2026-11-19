use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptExportKey(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_key = api_try!(emu, call.arg(emu, 0));
    let h_exp_key = api_try!(emu, call.arg(emu, 1));
    let dw_blob_type = api_try!(emu, call.arg_u32(emu, 2));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 3));
    let pb_data = api_try!(emu, call.arg(emu, 4));
    let pdw_data_len = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "crypt32!CryptExportKey key:0x{:x} exp:0x{:x} type:{} flags:0x{:x}",
        h_key,
        h_exp_key,
        dw_blob_type,
        dw_flags
    );
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 6);
}
