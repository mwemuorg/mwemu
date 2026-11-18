use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SHGetFolderPathEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let rfid = api_try!(emu, call.arg(emu, 0));
    let dw_flags = api_try!(emu, call.arg_u32(emu, 1));
    let h_token = api_try!(emu, call.arg_u32(emu, 2));
    let psz_path = api_try!(emu, call.arg(emu, 3));
    let cch_path = api_try!(emu, call.arg_u32(emu, 4));
    if psz_path != 0 && cch_path > 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\mwemu_user");
    }
    log_red!(
        emu,
        "shell32!SHGetFolderPathEx rfid:0x{:x} flags:0x{:x} => C:\\Users\\mwemu_user",
        rfid,
        dw_flags
    );
    call.cleanup_args(emu, 5);
    emu.set_last_error(0);
    call.set_return(emu, 0); // S_OK
}
