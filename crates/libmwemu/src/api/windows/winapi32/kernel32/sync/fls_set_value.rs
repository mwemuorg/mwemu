use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn FlsSetValue(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let idx = api_try!(emu, call.arg_u32(emu, 0));
    let val = api_try!(emu, call.arg_u32(emu, 1));

    log_red!(emu, "kernel32!FlsSetValue idx: {} val: {}", idx, val);

    if emu.fls().len() > idx as usize {
        emu.fls_mut()[idx as usize] = val as u64;
    } else {
        for _ in 0..=idx {
            emu.fls_mut().push(0);
        }
        emu.fls_mut()[idx as usize] = val as u64;
    }

    call.cleanup_args(emu, 2);
    call.set_return(emu, 1);
}
