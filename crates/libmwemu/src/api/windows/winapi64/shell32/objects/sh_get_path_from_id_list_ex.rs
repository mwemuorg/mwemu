use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SHGetPathFromIDListEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let psz_path = api_try!(emu, call.arg(emu, 1));
    let cch_path = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "shell32!SHGetPathFromIDListEx pidl:0x{:x} path:0x{:x} cch:{}",
        pidl,
        psz_path,
        cch_path
    );

    if psz_path != 0 && cch_path > 0 {
        emu.maps
            .write_wide_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else {
        emu.set_last_error(0);
        call.set_return(emu, 0);
    }
}
