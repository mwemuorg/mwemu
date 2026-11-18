use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn SHGetKnownFolderPath(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let folder_id_ptr = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg_u32(emu, 1));
    let token = api_try!(emu, call.arg_u32(emu, 2));
    let path_out_ptr = api_try!(emu, call.arg(emu, 3));
    let path = "C:\\Users\\mwemu_user\\AppData\\Local";
    let alloc_addr = emu
        .maps
        .alloc(512)
        .expect("shell32!SHGetKnownFolderPath alloc");
    emu.maps.write_wide_string(alloc_addr, path);
    if path_out_ptr != 0 {
        emu.maps.write_dword(path_out_ptr, alloc_addr as u32);
    }
    log_red!(
        emu,
        "shell32!SHGetKnownFolderPath flags:0x{:x} => '{}'",
        flags,
        path
    );
    call.cleanup_args(emu, 4);
    call.set_return(emu, 0);
}
