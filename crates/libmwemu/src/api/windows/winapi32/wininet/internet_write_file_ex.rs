use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn InternetWriteFileExA(emu: &mut emu::Emu) {
    internet_write_file_ex(emu, "InternetWriteFileExA");
}

fn internet_write_file_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let a0 = api_try!(emu, call.arg_u32(emu, 0)) as u64;
    let a1 = api_try!(emu, call.arg_u32(emu, 1)) as u64;
    log_red!(emu, "wininet!{api} arg0:0x{:x} arg1:0x{:x}", a0, a1);
    emu.set_last_error(0);
    call.set_return(emu, 1);
    call.cleanup_args(emu, 4);
}

pub fn InternetWriteFileExW(emu: &mut emu::Emu) {
    internet_write_file_ex(emu, "InternetWriteFileExW");
}
