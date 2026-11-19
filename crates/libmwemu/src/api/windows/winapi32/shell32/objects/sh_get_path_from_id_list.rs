use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn SHGetPathFromIDList(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let psz_path = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "shell32!SHGetPathFromIDList pidl:0x{:x} path:0x{:x}",
        pidl,
        psz_path
    );
    if psz_path != 0 {
        emu.maps.write_string(psz_path, "C:\\Users\\User\\Desktop");
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else {
        emu.set_last_error(0);
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 2);
}

pub fn SHGetPathFromIDListA(emu: &mut emu::Emu) {
    sh_get_path_from_id_list(emu, "SHGetPathFromIDListA", false);
}

fn sh_get_path_from_id_list(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let psz_path = api_try!(emu, call.arg(emu, 1));
    log_red!(emu, "shell32!{api} pidl:0x{:x} path:0x{:x}", pidl, psz_path);
    if psz_path != 0 {
        strings::write_string(emu, psz_path, "C:\\Users\\User\\Desktop", wide);
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else {
        emu.set_last_error(0);
        call.set_return(emu, 0);
    }
    call.cleanup_args(emu, 2);
}

pub fn SHGetPathFromIDListW(emu: &mut emu::Emu) {
    sh_get_path_from_id_list(emu, "SHGetPathFromIDListW", true);
}
