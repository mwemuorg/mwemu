use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn ExtractIconEx(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let file_ptr = api_try!(emu, call.arg(emu, 0));
    let icon_index = api_try!(emu, call.arg_i32(emu, 1));
    let large_ptr = api_try!(emu, call.arg(emu, 2));
    let small_ptr = api_try!(emu, call.arg(emu, 3));
    let file = if file_ptr != 0 {
        emu.maps.read_string(file_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "shell32!ExtractIconEx file:'{}' index:{}",
        file,
        icon_index
    );
    let handle = helper::handler_create("icon");
    if large_ptr != 0 {
        emu.maps.write_dword(large_ptr, handle as u32);
    }
    if small_ptr != 0 {
        emu.maps.write_dword(small_ptr, handle as u32);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}

pub fn ExtractIconExA(emu: &mut emu::Emu) {
    extract_icon_ex(emu, "ExtractIconExA", false);
}

fn extract_icon_ex(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let file_ptr = api_try!(emu, call.arg(emu, 0));
    let icon_index = api_try!(emu, call.arg_i32(emu, 1));
    let large_ptr = api_try!(emu, call.arg(emu, 2));
    let small_ptr = api_try!(emu, call.arg(emu, 3));
    let file = if file_ptr != 0 {
        strings::read_text(emu, file_ptr, wide)
    } else {
        String::new()
    };
    log_red!(emu, "shell32!{api} file:'{}' index:{}", file, icon_index);
    let handle = helper::handler_create("icon");
    if large_ptr != 0 {
        emu.maps.write_dword(large_ptr, handle as u32);
    }
    if small_ptr != 0 {
        emu.maps.write_dword(small_ptr, handle as u32);
    }
    call.set_return(emu, 1);
    call.cleanup_args(emu, 5);
}

pub fn ExtractIconExW(emu: &mut emu::Emu) {
    extract_icon_ex(emu, "ExtractIconExW", true);
}
