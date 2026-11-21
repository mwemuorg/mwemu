use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn PathCchFindExtension(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let path_ptr = api_try!(emu, call.arg(emu, 0));
    let cch_path = api_try!(emu, call.arg(emu, 1)) as usize;
    let ext_out = api_try!(emu, call.arg(emu, 2));
    let path = emu.maps.read_wide_string(path_ptr);
    let ext_offset = if let Some(pos) = path.rfind('.') {
        if path[pos..].contains('\\') {
            path.len()
        } else {
            pos
        }
    } else {
        path.len()
    };
    let result_addr = path_ptr + (ext_offset as u64) * 2;
    if ext_out != 0 {
        emu.maps.write_qword(ext_out, result_addr);
    }
    log_red!(
        emu,
        "kernelbase!PathCchFindExtension '{}' ext_offset:{}",
        path,
        ext_offset
    );
    call.set_return(emu, 0); // S_OK
}
