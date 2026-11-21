use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::api_try;
use crate::winapi::winapi64::kernel32::{FindFirstFileA, FindFirstFileW};

pub fn FindFirstFileExA(emu: &mut Emu) {
    invoke(emu, false, "FindFirstFileExA");
}

pub fn FindFirstFileExW(emu: &mut Emu) {
    invoke(emu, true, "FindFirstFileExW");
}

fn invoke(emu: &mut Emu, wide: bool, api: &str) {
    let call = WinApi(Stdcall);
    let argument_0 = api_try!(emu, call.arg(emu, 0));
    let argument_2 = api_try!(emu, call.arg(emu, 2));
    log_red!(
        emu,
        "kernel32!{} pattern:0x{:x} data:0x{:x}",
        api,
        argument_0,
        argument_2
    );
    emu.regs_mut().rdx = api_try!(emu, call.arg(emu, 2));
    if wide {
        FindFirstFileW(emu);
    } else {
        FindFirstFileA(emu);
    }
}
