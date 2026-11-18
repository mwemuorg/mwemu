use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn GetVolumeInformationA(emu: &mut emu::Emu) {
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
        emu.maps.read_string(root_ptr)
    } else {
        String::from("C:\\")
    };
    if vol_name_ptr != 0 && vol_name_size > 0 {
        emu.maps.write_string(vol_name_ptr, "MWEMU");
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
        emu.maps.write_string(fs_name_ptr, "NTFS");
    }
    log_red!(emu, "kernelbase!GetVolumeInformationA '{}'", root);
    call.set_return(emu, 1);
}

pub fn GetVolumeInformationW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let root = api_try!(emu, call.arg(emu, 0));
    let vol_name_buf = api_try!(emu, call.arg(emu, 1));
    let vol_name_sz = api_try!(emu, call.arg_u32(emu, 2));
    let serial_ptr = api_try!(emu, call.arg(emu, 3));
    let max_comp_ptr = api_try!(emu, call.arg(emu, 4));
    let flags_ptr = api_try!(emu, call.arg(emu, 5));
    let fs_name_buf = api_try!(emu, call.arg(emu, 6));
    log_red!(emu, "kernelbase!GetVolumeInformationW");
    let vol = "Windows";
    if vol_name_buf != 0 && vol_name_sz > 0 {
        for (i, ch) in vol.encode_utf16().enumerate() {
            emu.maps.write_word(vol_name_buf + (i as u64) * 2, ch);
        }
        emu.maps
            .write_word(vol_name_buf + (vol.len() as u64) * 2, 0);
    }
    if serial_ptr != 0 {
        emu.maps.write_dword(serial_ptr, 0x12345678);
    }
    if max_comp_ptr != 0 {
        emu.maps.write_dword(max_comp_ptr, 255);
    }
    if flags_ptr != 0 {
        emu.maps.write_dword(flags_ptr, 0x000700FF);
    }
    let fs = "NTFS";
    if fs_name_buf != 0 {
        for (i, ch) in fs.encode_utf16().enumerate() {
            emu.maps.write_word(fs_name_buf + (i as u64) * 2, ch);
        }
        emu.maps.write_word(fs_name_buf + (fs.len() as u64) * 2, 0);
    }
    call.set_return(emu, 1); // TRUE
}
