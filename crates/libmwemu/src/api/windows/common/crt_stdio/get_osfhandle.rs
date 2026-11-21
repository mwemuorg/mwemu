use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::common::crt::Runtime;
use crate::winapi::common::crt_stdio::{fd_handle, set_errno};
use crate::winapi::common::handles;
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::*;
use crate::windows::constants::files::{STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

pub(super) fn get_osfhandle(emu: &mut emu::Emu, runtime: Runtime) {
    let call = WinApi(Cdecl);
    let fd = api_try!(emu, call.arg_i32(emu, 0));
    let handle = fd_handle(emu, runtime, fd).map(u64::from).or_else(|| {
        let index = usize::try_from(fd).ok()?;
        let selector = *[STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE].get(index)?;
        if runtime.io(emu).closed_standard[index] {
            return None;
        }
        handles::standard_console(emu, selector)
    });
    log_red!(
        emu,
        "{}!_get_osfhandle fd {} = {:?}",
        runtime.library(),
        fd,
        handle
    );
    match handle {
        Some(handle) => call.set_return(emu, handle),
        None => {
            set_errno(emu, runtime, EBADF);
            call.set_return(
                emu,
                if emu.cfg.arch.is_64bits() {
                    u64::MAX
                } else {
                    u32::MAX as u64
                },
            );
        }
    }
}
