use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
    get_volume_information(emu, "GetVolumeInformationA", false);
}

fn get_volume_information(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let root_ptr = api_try!(emu, call.arg(emu, 0));
    let name_buf = api_try!(emu, call.arg(emu, 1));
    let name_size = api_try!(emu, call.arg_u32(emu, 2));
    let serial = api_try!(emu, call.arg(emu, 3));
    let max_comp = api_try!(emu, call.arg(emu, 4));
    let fs_flags = api_try!(emu, call.arg(emu, 5));
    let fs_name_buf = api_try!(emu, call.arg(emu, 6));
    let fs_name_size = api_try!(emu, call.arg_u32(emu, 7));

    let root = if root_ptr != 0 {
        strings::read_text(emu, root_ptr, wide)
    } else {
        String::new()
    };

    if name_buf != 0 && name_size > 8 {
        strings::write_string(emu, name_buf, "Windows", wide);
    }
    if serial != 0 {
        emu.maps.write_dword(serial, 0x1234_5678);
    }
    if max_comp != 0 {
        emu.maps.write_dword(max_comp, 255);
    }
    if fs_flags != 0 {
        emu.maps.write_dword(fs_flags, 0x000700FF);
    }
    if fs_name_buf != 0 && fs_name_size > 5 {
        strings::write_string(emu, fs_name_buf, "NTFS", wide);
    }

    log_red!(emu, "kernel32!{api} '{}'", root);

    call.cleanup_args(emu, 8);

    call.set_return(emu, 1);
}

pub fn GetVolumeInformationW(emu: &mut emu::Emu) {
    get_volume_information(emu, "GetVolumeInformationW", true);
}
