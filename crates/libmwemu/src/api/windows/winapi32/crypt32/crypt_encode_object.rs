use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CryptEncodeObject(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let dw_cert_encoding = api_try!(emu, call.arg(emu, 0));
    let lpsz_struct_type = api_try!(emu, call.arg(emu, 1));
    let pv_struct_info = api_try!(emu, call.arg(emu, 2));
    let pb_encoded = api_try!(emu, call.arg(emu, 3));
    let pcb_encoded = api_try!(emu, call.arg(emu, 4));
    let dw_flags = api_try!(emu, call.arg(emu, 5));
    log_red!(emu, "crypt32!CryptEncodeObject 0x{:x}", dw_cert_encoding);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}
