use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

#[path = "../__getmainargs.rs"]
mod __getmainargs;
pub use __getmainargs::__getmainargs;

#[path = "../_xcpt_filter.rs"]
mod _xcpt_filter;
pub use _xcpt_filter::_XcptFilter;

#[path = "../__dllonexit.rs"]
mod __dllonexit;
pub use __dllonexit::__dllonexit;

#[path = "../_except_handler3.rs"]
mod _except_handler3;
pub use _except_handler3::_except_handler3;

#[path = "../_eh_prolog.rs"]
mod _eh_prolog;
pub use _eh_prolog::_EH_prolog;

#[path = "../_cxx_throw_exception.rs"]
mod _cxx_throw_exception;
pub use _cxx_throw_exception::_CxxThrowException;

#[path = "../time.rs"]
mod time;
pub use time::time;

#[path = "../localtime.rs"]
mod localtime;
pub use localtime::localtime;

#[path = "../strftime.rs"]
mod strftime;
pub use strftime::strftime;

#[path = "../getenv.rs"]
mod getenv;
pub use getenv::getenv;

#[path = "../_wsystem.rs"]
mod _wsystem;
pub use _wsystem::_wsystem;

#[path = "../__1type_info__uae_xz.rs"]
mod __1type_info__uae_xz;
pub use __1type_info__uae_xz::__1type_info__UAE_XZ;

#[path = "../__0exception__qae_abv0__z.rs"]
mod __0exception__qae_abv0__z;
pub use __0exception__qae_abv0__z::__0exception__QAE_ABV0__Z;

#[path = "../_cexit.rs"]
mod _cexit;
pub use _cexit::_cexit;

#[path = "../_beginthread.rs"]
mod _beginthread;
pub use _beginthread::_beginthread;

#[path = "../_beginthreadex.rs"]
mod _beginthreadex;
pub use _beginthreadex::_beginthreadex;

#[path = "../_endthread.rs"]
mod _endthread;
pub use _endthread::_endthread;

#[path = "../system.rs"]
mod system;
pub use system::system;

#[path = "../_putenv.rs"]
mod _putenv;
pub use _putenv::_putenv;

#[path = "../__wgetmainargs.rs"]
mod __wgetmainargs;
pub use __wgetmainargs::__wgetmainargs;

#[path = "../_wcmdln.rs"]
mod _wcmdln;
pub use _wcmdln::_wcmdln;

#[path = "../_except_handler4.rs"]
mod _except_handler4;
pub use _except_handler4::_except_handler4;

#[path = "../_set_se_translator.rs"]
mod _set_se_translator;
pub use _set_se_translator::_set_se_translator;

#[path = "../_eh_prolog3.rs"]
mod _eh_prolog3;
pub use _eh_prolog3::_EH_prolog3;

#[path = "../clock.rs"]
mod clock;
pub use clock::clock;

#[path = "../_time64.rs"]
mod _time64;
pub use _time64::_time64;

#[path = "../_unlock.rs"]
mod _unlock;
pub use _unlock::_unlock;

#[path = "../_configthreadlocale.rs"]
mod _configthreadlocale;
pub use _configthreadlocale::_configthreadlocale;

#[path = "../_amsg_exit.rs"]
mod _amsg_exit;
pub use _amsg_exit::_amsg_exit;

#[path = "../_crt_atexit.rs"]
mod _crt_atexit;
pub use _crt_atexit::_crt_atexit;

#[path = "../_initialize_onexit_table.rs"]
mod _initialize_onexit_table;
pub use _initialize_onexit_table::_initialize_onexit_table;

#[path = "../__cpp_xcpt_filter.rs"]
mod __cpp_xcpt_filter;
pub use __cpp_xcpt_filter::__CppXcptFilter;

#[path = "../__p__environ.rs"]
mod __p__environ;
pub use __p__environ::__p__environ;

#[path = "../_configure_narrow_argv.rs"]
mod _configure_narrow_argv;
pub use _configure_narrow_argv::_configure_narrow_argv;

#[path = "../_set_errno.rs"]
mod _set_errno;
pub use _set_errno::_set_errno;

#[path = "../_get_errno.rs"]
mod _get_errno;
pub use _get_errno::_get_errno;

#[path = "../_get_initial_narrow_environment.rs"]
mod _get_initial_narrow_environment;
pub use _get_initial_narrow_environment::_get_initial_narrow_environment;

#[path = "../_initialize_narrow_environment.rs"]
mod _initialize_narrow_environment;
pub use _initialize_narrow_environment::_initialize_narrow_environment;

#[path = "../_localtime64.rs"]
mod _localtime64;
pub use _localtime64::_localtime64;

#[path = "../_set_app_type.rs"]
mod _set_app_type;
pub use _set_app_type::_set_app_type;

#[path = "../_set_invalid_parameter_handler.rs"]
mod _set_invalid_parameter_handler;
pub use _set_invalid_parameter_handler::_set_invalid_parameter_handler;

#[path = "../_set_thread_local_invalid_parameter_handler.rs"]
mod _set_thread_local_invalid_parameter_handler;
pub use _set_thread_local_invalid_parameter_handler::_set_thread_local_invalid_parameter_handler;

#[path = "../_time32.rs"]
mod _time32;
pub use _time32::_time32;

#[path = "../abort.rs"]
mod abort;
pub use abort::abort;

#[path = "../ptr.rs"]
mod ptr;
pub use ptr::ptr;

#[path = "../signal.rs"]
mod signal;
pub use signal::signal;

#[path = "../wcsftime.rs"]
mod wcsftime;
pub use wcsftime::wcsftime;

#[path = "../__lconv_init.rs"]
mod __lconv_init;
pub use __lconv_init::__lconv_init;

#[path = "../__setlc_active.rs"]
mod __setlc_active;
pub use __setlc_active::__setlc_active;

#[path = "../_set_error_mode.rs"]
mod _set_error_mode;
pub use _set_error_mode::_set_error_mode;

#[path = "../_set_fmode.rs"]
mod _set_fmode;
pub use _set_fmode::_set_fmode;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "_initterm_e" => msvcrt::_initterm_e(emu),
        "_initterm" => msvcrt::_initterm(emu),
        "__p___argv" => msvcrt::__p___argv(emu),
        "__p___argc" => msvcrt::__p___argc(emu),
        "_onexit" => msvcrt::_onexit(emu),
        "_lock" => msvcrt::_lock(emu),
        "exit" | "_exit" => msvcrt::exit(emu),
        "__set_app_type" => msvcrt::__set_app_type(emu),
        "_errno" => msvcrt::_errno(emu),
        "__p__fmode" => msvcrt::__p__global_int(emu, "__p__fmode"),
        "__p__commode" => msvcrt::__p__global_int(emu, "__p__commode"),
        "_adjust_fdiv" => msvcrt::__p__global_int(emu, "_adjust_fdiv"),
        "__getmainargs" => msvcrt::__getmainargs(emu),
        "_acmdln" | "__p__acmdln" => msvcrt::__p__acmdln(emu),
        "_XcptFilter" => msvcrt::_XcptFilter(emu),
        "__dllonexit" => msvcrt::__dllonexit(emu),
        "_except_handler3" | "__CxxFrameHandler" => msvcrt::_except_handler3(emu, api),
        "_EH_prolog" => msvcrt::_EH_prolog(emu),
        "_CxxThrowException" => msvcrt::_CxxThrowException(emu),
        "time" => msvcrt::time(emu),
        "localtime" => msvcrt::localtime(emu),
        "strftime" => msvcrt::strftime(emu),
        "getenv" | "_wgetenv" => msvcrt::getenv(emu, api),
        "_wsystem" => msvcrt::_wsystem(emu),
        "??1type_info@@UAE@XZ" => msvcrt::__1type_info__UAE_XZ(emu),
        "??0exception@@QAE@ABV0@@Z" => msvcrt::__0exception__QAE_ABV0__Z(emu),
        "_cexit" | "_c_exit" => msvcrt::_cexit(emu, api),
        "_beginthread" => msvcrt::_beginthread(emu),
        "_beginthreadex" => msvcrt::_beginthreadex(emu),
        "_endthread" | "_endthreadex" => msvcrt::_endthread(emu, api),
        "system" => msvcrt::system(emu),
        "_putenv" | "_wputenv" | "_putenv_s" => msvcrt::_putenv(emu, api),
        "__wgetmainargs" => msvcrt::__wgetmainargs(emu),
        "_wcmdln" | "__p__wcmdln" => msvcrt::_wcmdln(emu, addr, api),
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
        "_configthreadlocale" => msvcrt::_configthreadlocale(emu),
        "_amsg_exit" => msvcrt::_amsg_exit(emu),
        "_crt_atexit" | "_register_onexit_function" => msvcrt::_crt_atexit(emu, api),
        "_initialize_onexit_table" => msvcrt::_initialize_onexit_table(emu),
        "__CppXcptFilter" => msvcrt::__CppXcptFilter(emu),
        "__p__environ" => msvcrt::__p__environ(emu),
        "_configure_narrow_argv" => msvcrt::_configure_narrow_argv(emu),
        "_set_errno" => msvcrt::_set_errno(emu),
        "_get_errno" => msvcrt::_get_errno(emu),
        "_get_initial_narrow_environment" => msvcrt::_get_initial_narrow_environment(emu),
        "_initialize_narrow_environment" => msvcrt::_initialize_narrow_environment(emu),
        "_localtime64" => msvcrt::_localtime64(emu),
        "_set_app_type" => msvcrt::_set_app_type(emu),
        "_set_invalid_parameter_handler" => msvcrt::_set_invalid_parameter_handler(emu),
        "_set_thread_local_invalid_parameter_handler" => {
            msvcrt::_set_thread_local_invalid_parameter_handler(emu)
        }
        "_time32" => msvcrt::_time32(emu),
        "abort" => msvcrt::abort(emu),
        "ptr" => msvcrt::ptr(emu),
        "signal" => msvcrt::signal(emu),
        "wcsftime" => msvcrt::wcsftime(emu),
        "__lconv_init" => msvcrt::__lconv_init(emu, addr),
        "__setlc_active" | "__unguarded_readlc_active" => msvcrt::__setlc_active(emu, addr, api),
        "_set_error_mode" => msvcrt::_set_error_mode(emu),
        "_set_fmode" => msvcrt::_set_fmode(emu),
        _ => return false,
    }
    true
}
