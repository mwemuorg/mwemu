use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetVolumeInformationByHandleW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let root_ptr = api_try!(emu, call.arg(emu, 0));
    let vol_name_ptr = api_try!(emu, call.arg(emu, 1));
    let vol_name_size = api_try!(emu, call.arg(emu, 2));
    let serial_ptr = api_try!(emu, call.arg(emu, 3));
    let max_comp_ptr = api_try!(emu, call.arg(emu, 4));
    let flags_ptr = api_try!(emu, call.arg(emu, 5));
    let fs_name_ptr = api_try!(emu, call.arg(emu, 6));
    let fs_name_size = api_try!(emu, call.arg(emu, 7));
    let root = if root_ptr != 0 {
        emu.maps.read_wide_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    if vol_name_ptr != 0 && vol_name_size > 0 {
        emu.maps.write_wide_string(vol_name_ptr, "MWEMU");
    }
    if serial_ptr != 0 {
        emu.maps.write_dword(serial_ptr, 0x12345678);
    }
    if max_comp_ptr != 0 {
        emu.maps.write_dword(max_comp_ptr, 255);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x700ff);
    }
    if fs_name_ptr != 0 && fs_name_size > 0 {
        emu.maps.write_wide_string(fs_name_ptr, "NTFS");
    }
    log_red!(emu, "kernelbase!GetVolumeInformationByHandleW '{}'", root);
    call.set_return(emu, 1);
}
