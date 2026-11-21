use crate::api::dispatch_api;
use crate::emu::Emu;
pub use crate::winapi::libraries::msvcrt::runtime::{
    __CppXcptFilter, __set_app_type, _CxxThrowException, _EH_prolog, _EH_prolog3, _XcptFilter,
    _except_handler3, _except_handler4, _set_app_type, _set_error_mode, _set_fmode,
    _set_invalid_parameter_handler, _set_se_translator,
    _set_thread_local_invalid_parameter_handler, ptr,
};
use crate::winapi::winapi32::msvcrt;

pub use crate::api::windows::libraries::msvcrt::runtime::*;

mod strftime;
pub use strftime::strftime;

mod __1type_info__uae_xz;
pub use __1type_info__uae_xz::__1type_info__UAE_XZ;

mod __0exception__qae_abv0__z;
pub use __0exception__qae_abv0__z::__0exception__QAE_ABV0__Z;

mod abort;
pub use abort::abort;

pub(super) fn dispatch(api: &str, _addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "_lock" => msvcrt::_lock(emu),
        "__set_app_type" => msvcrt::__set_app_type(emu),
        "_XcptFilter" => msvcrt::_XcptFilter(emu),
        "_except_handler3" | "__CxxFrameHandler" => msvcrt::_except_handler3(emu, api),
        "_EH_prolog" => msvcrt::_EH_prolog(emu),
        "_CxxThrowException" => msvcrt::_CxxThrowException(emu),
        "time" => msvcrt::time(emu),
        "strftime" => msvcrt::strftime(emu),
        "??1type_info@@UAE@XZ" => msvcrt::__1type_info__UAE_XZ(emu),
        "??0exception@@QAE@ABV0@@Z" => msvcrt::__0exception__QAE_ABV0__Z(emu),
        "system" => msvcrt::system(emu),
        "_except_handler4"
        | "_except_handler4_common"
        | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => msvcrt::_except_handler4(emu, api),
        "_set_se_translator" | "_set_new_handler" | "_set_new_mode" | "set_terminate"
        | "set_unexpected" => msvcrt::_set_se_translator(emu, api),
        "_EH_prolog3" | "_EH_prolog3_catch" | "_EH_prolog3_GS" => msvcrt::_EH_prolog3(emu, api),
        "clock" => msvcrt::clock(emu),
        "_time64" => msvcrt::_time64(emu),
        "_unlock" => msvcrt::_unlock(emu),
        "_amsg_exit" => msvcrt::_amsg_exit(emu),
        "__CppXcptFilter" => msvcrt::__CppXcptFilter(emu),
        "_set_app_type" => msvcrt::_set_app_type(emu),
        "_set_invalid_parameter_handler" => msvcrt::_set_invalid_parameter_handler(emu),
        "_set_thread_local_invalid_parameter_handler" => msvcrt::_set_thread_local_invalid_parameter_handler(emu),
        "_time32" => msvcrt::_time32(emu),
        "abort" => msvcrt::abort(emu),
        "ptr" => msvcrt::ptr(emu),
        "signal" => msvcrt::signal(emu),
        "_set_error_mode" => msvcrt::_set_error_mode(emu),
        "_set_fmode" => msvcrt::_set_fmode(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
