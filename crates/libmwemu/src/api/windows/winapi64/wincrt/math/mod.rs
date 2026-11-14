use crate::emu::Emu;
use crate::winapi::winapi64::wincrt;

mod __setusermatherr;
pub use __setusermatherr::__setusermatherr;

mod atan;
pub use atan::atan;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "abs" | "labs" => wincrt::crt_abs(emu),
        "rand" => wincrt::crt_rand(emu),
        "srand" => wincrt::crt_srand(emu),
        "_ftol" | "_ftol2" | "_ftol2_sse" => wincrt::crt_ftol(emu),
        "_controlfp" => wincrt::crt_controlfp(emu),
        "_controlfp_s" => wincrt::crt_controlfp_s(emu),
        "__setusermatherr"
        | "_set_error_mode"
        | "__set_app_type"
        | "_configthreadlocale"
        | "_set_fmode"
        | "__lconv_init"
        | "_amsg_exit"
        | "__setlc_active"
        | "__unguarded_readlc_active"
        | "_set_new_handler"
        | "_set_se_translator"
        | "set_terminate"
        | "_set_thread_local_invalid_parameter_handler" => wincrt::__setusermatherr(emu, api),
        "atan" | "atan2" | "cos" | "sin" | "tan" | "exp" | "log" | "log10" | "pow" | "sqrt"
        | "ceil" | "floor" | "fabs" | "fmod" => wincrt::atan(emu, api),
        _ => return false,
    }
    true
}
