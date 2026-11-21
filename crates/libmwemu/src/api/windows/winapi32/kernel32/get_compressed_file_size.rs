use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::policy::api_try;

pub fn GetCompressedFileSizeA(emu: &mut emu::Emu) {
    get_compressed_file_size(emu, "GetCompressedFileSizeA", false);
}

fn get_compressed_file_size(emu: &mut emu::Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let p0 = api_try!(emu, call.arg(emu, 0));
    let p1 = api_try!(emu, call.arg(emu, 1));
    let s = if p0 != 0 && p0 > 0xffff {
        strings::read_text(emu, p0, wide)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!{api} '{}' p1:0x{:x}", s, p1);
    call.cleanup_args(emu, 2);
    call.set_return(emu, 4096);
}

pub fn GetCompressedFileSizeW(emu: &mut emu::Emu) {
    get_compressed_file_size(emu, "GetCompressedFileSizeW", true);
}
