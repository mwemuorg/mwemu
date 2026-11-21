use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn DragQueryFile(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let hdrop = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let buf_size = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(
        emu,
        "shell32!DragQueryFile hdrop:0x{:x} index:{}",
        hdrop,
        index
    );
    let out_ptr = api_try!(emu, call.arg(emu, 3));
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }
    if index == 0xFFFFFFFF {
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else if buf_ptr != 0 && buf_size > 10 {
        emu.maps.write_string(buf_ptr, "C:\\file.txt");
        call.set_return(emu, 10);
    } else {
        emu.set_last_error(0);
        call.set_return(emu, 10);
    }
}

pub fn DragQueryFileA(emu: &mut emu::Emu) {
    drag_query_file(emu, "DragQueryFileA", false);
}

fn drag_query_file(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let hdrop = api_try!(emu, call.arg(emu, 0));
    let index = api_try!(emu, call.arg_u32(emu, 1));
    let buf_ptr = api_try!(emu, call.arg(emu, 2));
    let buf_size = api_try!(emu, call.arg_u32(emu, 3));
    log_red!(emu, "shell32!{api} hdrop:0x{:x} index:{}", hdrop, index);

    if index == 0xFFFFFFFF {
        emu.set_last_error(0);
        call.set_return(emu, 1);
    } else if buf_ptr != 0 && buf_size > 10 {
        strings::write_string(emu, buf_ptr, "C:\\file.txt", wide);
        call.set_return(emu, 10);
    } else {
        emu.set_last_error(0);
        call.set_return(emu, 10);
    }
}

pub fn DragQueryFileW(emu: &mut emu::Emu) {
    drag_query_file(emu, "DragQueryFileW", true);
}
