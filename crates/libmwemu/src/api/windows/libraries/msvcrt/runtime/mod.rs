use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::crt::{Runtime, dispatch_runtime};
use crate::winapi::policy;

mod system;
pub use system::system;

mod _amsg_exit;
pub use _amsg_exit::_amsg_exit;

mod clock;
pub use clock::{_time32, _time64, clock, time};

pub fn signal(emu: &mut Emu) {
    policy::unsupported_with_abi(emu, "msvcrt", "signal", emu.pc(), WinApi(Cdecl));
}

macro_rules! runtime_entry_points {
    ($($name:ident),+ $(,)?) => {$(pub fn $name(emu:&mut Emu) {
        dispatch_runtime(stringify!($name),emu,Runtime::Msvcrt);
    })+};
}

runtime_entry_points!(
    _CxxThrowException,
    _EH_prolog,
    _XcptFilter,
    __CppXcptFilter,
    __set_app_type,
    __setusermatherr,
    _lock,
    _set_app_type,
    _set_error_mode,
    _set_fmode,
    _set_invalid_parameter_handler,
    _set_thread_local_invalid_parameter_handler,
    _unlock,
    ptr,
);

pub fn _EH_prolog3(emu: &mut Emu, api: &str) {
    dispatch_runtime(api, emu, Runtime::Msvcrt);
}

pub fn _except_handler3(emu: &mut Emu, api: &str) {
    dispatch_runtime(api, emu, Runtime::Msvcrt);
}

pub fn _except_handler4(emu: &mut Emu, api: &str) {
    dispatch_runtime(api, emu, Runtime::Msvcrt);
}

pub fn _set_se_translator(emu: &mut Emu, api: &str) {
    dispatch_runtime(api, emu, Runtime::Msvcrt);
}
