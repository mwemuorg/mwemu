use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::helper;
use crate::winapi::policy::api_try;

pub fn FindFirstFileExA(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExA");
}

fn find_first_file_ex(emu: &mut emu::Emu, api: &str) {
    let call = WinApi(Stdcall);
    let arg1 = api_try!(emu, call.arg(emu, 0));
    let arg2 = api_try!(emu, call.arg(emu, 1));
    let arg3 = api_try!(emu, call.arg(emu, 2));
    let arg4 = api_try!(emu, call.arg(emu, 3));
    let arg5 = api_try!(emu, call.arg(emu, 4));
    let arg6 = api_try!(emu, call.arg(emu, 5));
    log_red!(
        emu,
        "kernelbase!{api} arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6
    );
    let hndl = helper::handler_create(&format!("findfile://{api}"));
    call.set_return(emu, hndl);
}

pub fn FindFirstFileExW(emu: &mut emu::Emu) {
    find_first_file_ex(emu, "FindFirstFileExW");
}
