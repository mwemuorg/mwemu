use crate::emu::Emu;
use crate::winapi::winapi64::wincrt;

#[path = "../_eh_prolog3.rs"]
mod _eh_prolog3;
pub use _eh_prolog3::_EH_prolog3;

#[path = "../_putenv.rs"]
mod _putenv;
pub use _putenv::_putenv;

#[path = "../_wputenv.rs"]
mod _wputenv;
pub use _wputenv::_wputenv;

#[path = "../_eh_prolog.rs"]
mod _eh_prolog;
pub use _eh_prolog::_EH_prolog;

#[path = "../_putenv_s.rs"]
mod _putenv_s;
pub use _putenv_s::_putenv_s;

#[path = "../system.rs"]
mod system;
pub use system::system;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "_initialize_onexit_table" => wincrt::_initialize_onexit_table(emu),
        "_register_onexit_function" => wincrt::_register_onexit_function(emu),
        "_get_initial_narrow_environment" => wincrt::_get_initial_narrow_environment(emu),
        "_initialize_narrow_environment" => wincrt::_initialize_narrow_environment(emu),
        "_configure_narrow_argv" => wincrt::_configure_narrow_argv(emu),
        "_set_invalid_parameter_handler" => wincrt::set_invalid_parameter_handler(emu),
        "_set_app_type" => wincrt::_set_app_type(emu),
        "_crt_atexit" => wincrt::_crt_atexit(emu),
        "__p___argv" => wincrt::__p___argv(emu),
        "__p___argc" => wincrt::__p___argc(emu),
        "__p__environ" => wincrt::__p__environ(emu),
        "__p__commode" => wincrt::__p__commode(emu),
        "__p__fmode" => wincrt::__p__fmode(emu),
        "_set_new_mode" => wincrt::_set_new_mode(emu),
        "abort" => wincrt::abort(emu),
        "signal" => wincrt::signal(emu),
        "exit" | "_exit" | "_cexit" => wincrt::crt_exit(emu, api),
        "getenv" | "_wgetenv" => wincrt::crt_getenv(emu, api),
        "__getmainargs" => wincrt::crt_getmainargs(emu),
        "__wgetmainargs" => wincrt::crt_getmainargs(emu),
        "_initterm" => wincrt::crt_initterm(emu),
        "_initterm_e" => wincrt::crt_initterm_e(emu),
        "_except_handler3"
        | "_except_handler4"
        | "_except_handler4_common"
        | "__CxxFrameHandler"
        | "__CxxFrameHandler3"
        | "__CxxFrameHandler4" => wincrt::crt_except_handler(emu, api),
        "_CxxThrowException" => wincrt::crt_cxx_throw(emu),
        "_XcptFilter" | "__CppXcptFilter" => wincrt::crt_xcpt_filter(emu, api),
        "__dllonexit" => wincrt::crt_dllonexit(emu),
        "_onexit" => wincrt::crt_onexit(emu),
        "clock" => wincrt::crt_clock(emu),
        "time" | "_time64" | "_time32" => wincrt::crt_time(emu),
        "_lock" | "_unlock" => wincrt::crt_lock_noop(emu, api),
        "_beginthread" => wincrt::crt_beginthread(emu),
        "_beginthreadex" => wincrt::crt_beginthreadex(emu),
        "_endthread" | "_endthreadex" => wincrt::crt_endthread(emu, api),
        "localtime" | "_localtime64" => wincrt::crt_localtime(emu),
        "strftime" | "wcsftime" => wincrt::crt_strftime(emu),
        "_acmdln" | "__p__acmdln" | "_wcmdln" | "__p__wcmdln" => wincrt::crt_acmdln(emu, api),
        "_EH_prolog3" | "_EH_prolog3_catch" => wincrt::_EH_prolog3(emu, api),
        "_putenv" => wincrt::_putenv(emu),
        "_wputenv" => wincrt::_wputenv(emu),
        "_EH_prolog" | "_EH_prolog3_GS" | "_adjust_fdiv" | "set_unexpected" | "_c_exit" => {
            wincrt::_EH_prolog(emu, api)
        }
        "_putenv_s" => wincrt::_putenv_s(emu),
        "system" | "_wsystem" => wincrt::system(emu, api),
        _ => return false,
    }
    true
}
