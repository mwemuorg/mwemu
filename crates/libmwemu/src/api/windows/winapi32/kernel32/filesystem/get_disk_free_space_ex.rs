use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetDiskFreeSpaceExA(emu: &mut emu::Emu) {
    get_disk_free_space_ex(emu, "GetDiskFreeSpaceExA", false);
}

fn get_disk_free_space_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let free_caller = api_try!(emu, call.arg(emu, 1));
    let total = api_try!(emu, call.arg(emu, 2));
    let free_total = api_try!(emu, call.arg(emu, 3));

    let path = if path_ptr != 0 {
        strings::read_text(emu, path_ptr, wide)
    } else {
        String::new()
    };

    if free_caller != 0 {
        emu.maps.write_qword(free_caller, 50_000_000_000);
    }
    if total != 0 {
        emu.maps.write_qword(total, 500_000_000_000);
    }
    if free_total != 0 {
        emu.maps.write_qword(free_total, 50_000_000_000);
    }

    log_red!(emu, "kernel32!{api} '{}'", path);

    call.cleanup_args(emu, 4);

    call.set_return(emu, 1);
}

pub fn GetDiskFreeSpaceExW(emu: &mut emu::Emu) {
    get_disk_free_space_ex(emu, "GetDiskFreeSpaceExW", true);
}
