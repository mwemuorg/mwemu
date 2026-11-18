use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindFirstFileA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let filename_ptr = api_try!(emu, call.arg(emu, 0));
    let find_data = api_try!(emu, call.arg(emu, 1));
    let pattern = emu.maps.read_string(filename_ptr);
    log_red!(
        emu,
        "kernelbase!FindFirstFileA pattern:'{}' data:0x{:x}",
        pattern,
        find_data
    );
    if find_data != 0 {
        emu.maps.write_dword(find_data, 0x80); // dwFileAttributes = FILE_ATTRIBUTE_NORMAL
        emu.maps.write_dword(find_data + 28, 0); // nFileSizeHigh
        emu.maps.write_dword(find_data + 32, 1024); // nFileSizeLow
        emu.maps.write_string(find_data + 44, "dummy.txt");
    }
    let hndl = helper::handler_create("findfile://search");
    call.set_return(emu, hndl);
}

pub fn FindFirstFileW(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    log_red!(
        emu,
        "kernelbase!FindFirstFileW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let hndl = helper::handler_create("findfile://FindFirstFileW");
    call.set_return(emu, hndl);
}
