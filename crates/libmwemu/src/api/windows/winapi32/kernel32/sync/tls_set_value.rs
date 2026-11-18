use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn TlsSetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg_u32(emu, 0));
    let val = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!TlsSetValue idx: {} val: 0x{:x}", idx, val);

    if emu.tls32().len() > idx as usize {
        emu.tls32_mut()[idx as usize] = val;
    } else {
        for _ in 0..=idx {
            emu.tls32_mut().push(0);
        }
        emu.tls32_mut()[idx as usize] = val;
    }

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
