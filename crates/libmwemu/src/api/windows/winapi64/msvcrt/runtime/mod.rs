use crate::emu::Emu;
use crate::winapi::winapi64::msvcrt;

mod _initterm_e;
pub use _initterm_e::_initterm_e;

mod _initterm;
pub use _initterm::_initterm;

mod __p___argv;
pub use __p___argv::__p___argv;

mod __p___argc;
pub use __p___argc::__p___argc;

mod _onexit;
pub use _onexit::_onexit;

mod _lock;
pub use _lock::_lock;

mod exit;
pub use exit::exit;

mod __set_app_type;
pub use __set_app_type::__set_app_type;

mod _errno;
pub use _errno::_errno;

mod __p__fmode;
pub use __p__fmode::__p__fmode;

mod __getmainargs;
pub use __getmainargs::__getmainargs;

mod _acmdln;
pub use _acmdln::_acmdln;

mod _xcpt_filter;
pub use _xcpt_filter::_XcptFilter;

mod __dllonexit;
pub use __dllonexit::__dllonexit;

mod _except_handler3;
pub use _except_handler3::_except_handler3;

mod _eh_prolog;
pub use _eh_prolog::_EH_prolog;

mod _cxx_throw_exception;
pub use _cxx_throw_exception::_CxxThrowException;

mod time;
pub use time::time;

mod localtime;
pub use localtime::localtime;

mod strftime;
pub use strftime::strftime;

mod getenv;
pub use getenv::getenv;

mod _wsystem;
pub use _wsystem::_wsystem;

mod __1type_info__uae_xz;
pub use __1type_info__uae_xz::__1type_info__UAE_XZ;

mod __0exception__qae_abv0__z;
pub use __0exception__qae_abv0__z::__0exception__QAE_ABV0__Z;

mod _cexit;
pub use _cexit::_cexit;

mod _beginthread;
pub use _beginthread::_beginthread;

mod _beginthreadex;
pub use _beginthreadex::_beginthreadex;

mod _endthread;
pub use _endthread::_endthread;

mod system;
pub use system::system;

mod _putenv;
pub use _putenv::_putenv;

mod __wgetmainargs;
pub use __wgetmainargs::__wgetmainargs;

mod _wcmdln;
pub use _wcmdln::_wcmdln;

mod _except_handler4;
pub use _except_handler4::_except_handler4;

mod _set_se_translator;
pub use _set_se_translator::_set_se_translator;

mod _eh_prolog3;
pub use _eh_prolog3::_EH_prolog3;

mod clock;
pub use clock::clock;

mod _time64;
pub use _time64::_time64;

mod _unlock;
pub use _unlock::_unlock;

mod _configthreadlocale;
pub use _configthreadlocale::_configthreadlocale;

mod _amsg_exit;
pub use _amsg_exit::_amsg_exit;

mod _crt_atexit;
pub use _crt_atexit::_crt_atexit;

mod _initialize_onexit_table;
pub use _initialize_onexit_table::_initialize_onexit_table;

mod __cpp_xcpt_filter;
pub use __cpp_xcpt_filter::__CppXcptFilter;

mod __p__environ;
pub use __p__environ::__p__environ;

mod _configure_narrow_argv;
pub use _configure_narrow_argv::_configure_narrow_argv;

mod _set_errno;
pub use _set_errno::_set_errno;

mod _get_errno;
pub use _get_errno::_get_errno;

mod _get_initial_narrow_environment;
pub use _get_initial_narrow_environment::_get_initial_narrow_environment;

mod _initialize_narrow_environment;
pub use _initialize_narrow_environment::_initialize_narrow_environment;

mod _localtime64;
pub use _localtime64::_localtime64;

mod _set_app_type;
pub use _set_app_type::_set_app_type;

mod _set_invalid_parameter_handler;
pub use _set_invalid_parameter_handler::_set_invalid_parameter_handler;

mod _set_thread_local_invalid_parameter_handler;
pub use _set_thread_local_invalid_parameter_handler::_set_thread_local_invalid_parameter_handler;

mod _time32;
pub use _time32::_time32;

mod abort;
pub use abort::abort;

mod ptr;
pub use ptr::ptr;

mod signal;
pub use signal::signal;

mod wcsftime;
pub use wcsftime::wcsftime;

mod __lconv_init;
pub use __lconv_init::__lconv_init;

mod __setlc_active;
pub use __setlc_active::__setlc_active;

mod _set_error_mode;
pub use _set_error_mode::_set_error_mode;

mod _set_fmode;
pub use _set_fmode::_set_fmode;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
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
        "__p__fmode" | "__p__commode" | "_adjust_fdiv" => msvcrt::__p__fmode(emu, api),
        "__getmainargs" => msvcrt::__getmainargs(emu),
        "_acmdln" | "__p__acmdln" => msvcrt::_acmdln(emu),
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
        "_wcmdln" | "__p__wcmdln" => msvcrt::_wcmdln(emu, api),
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
        "__lconv_init" => msvcrt::__lconv_init(emu),
        "__setlc_active" | "__unguarded_readlc_active" => msvcrt::__setlc_active(emu, api),
        "_set_error_mode" => msvcrt::_set_error_mode(emu),
        "_set_fmode" => msvcrt::_set_fmode(emu),
        _ => return false,
    }
    true
}
