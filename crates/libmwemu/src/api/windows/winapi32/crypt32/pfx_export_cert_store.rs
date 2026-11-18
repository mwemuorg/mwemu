use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PFXExportCertStore(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let h_store = api_try!(emu, call.arg(emu, 0));
    let p_pfx = api_try!(emu, call.arg(emu, 1));
    let sz_password = api_try!(emu, call.arg(emu, 2));
    let dw_flags = api_try!(emu, call.arg(emu, 3));
    log_red!(emu, "crypt32!PFXExportCertStore 0x{:x}", h_store);
    let val_ptr = api_try!(emu, call.arg(emu, 1));
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}
