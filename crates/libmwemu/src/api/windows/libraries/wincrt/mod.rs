use crate::api::abi::windows::{
    CallingConvention::{Cdecl, Stdcall},
    WindowsAbi::WinApi,
};
use crate::emu::Emu;
use crate::winapi::common::crt::{self, Runtime::Ucrt};
use crate::winapi::common::crt_stdio;
use crate::winapi::libraries::msvcrt::{math, runtime};
use crate::winapi::policy::{self, api_try};

pub(crate) fn gateway(api: &str, addr: u64, emu: &mut Emu) -> String {
    if crt::dispatch(api, emu)
        || crt::dispatch_runtime(api, emu, Ucrt)
        || crt_stdio::gateway(api, emu, Ucrt)
    {
        return String::new();
    }
    log_red!(emu, "ucrtbase!{api}");
    match api {
        "abs" | "labs" => absolute(emu),
        "atan" => math::atan(emu),
        "atan2" => math::atan2(emu),
        "ceil" => math::ceil(emu),
        "cos" => math::cos(emu),
        "exp" => math::exp(emu),
        "fabs" => math::fabs(emu),
        "floor" => math::floor(emu),
        "fmod" => math::fmod(emu),
        "log" => math::log(emu),
        "log10" => math::log10(emu),
        "pow" => math::pow(emu),
        "sin" => math::sin(emu),
        "sqrt" => math::sqrt(emu),
        "tan" => math::tan(emu),
        "time" | "_time64" => runtime::_time64(emu),
        "_time32" => runtime::_time32(emu),
        "clock" => runtime::clock(emu),
        "abort" => emu.stop(),
        _ => {
            let convention = match api {
                "system"
                | "_wsystem"
                | "signal"
                | "setvbuf"
                | "_setmode"
                | "_wrename"
                | "_putch"
                | "localtime"
                | "_localtime64"
                | "strftime"
                | "wcsftime"
                | "mbstowcs"
                | "wcstombs"
                | "_mbstowcs_l"
                | "_wcstombs_l"
                | "__stdio_common_vsprintf"
                | "__stdio_common_vsscanf"
                | "__stdio_common_vsprintf_s"
                | "__setusermatherr"
                | "_set_error_mode"
                | "_set_new_handler"
                | "_set_se_translator"
                | "set_terminate"
                | "set_unexpected" => Cdecl,
                _ => Stdcall,
            };
            policy::unsupported_with_abi(emu, "ucrtbase", api, addr, WinApi(convention));
            if emu.api_policy.stopped() {
                return api.to_ascii_lowercase();
            }
        }
    }
    String::new()
}

fn absolute(emu: &mut Emu) {
    let call = WinApi(Cdecl);
    let value = api_try!(emu, call.arg_i32(emu, 0));
    call.set_return(emu, u64::from(value.unsigned_abs()));
}
