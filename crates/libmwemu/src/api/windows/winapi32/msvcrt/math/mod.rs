use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

pub use crate::api::windows::libraries::msvcrt::math::*;

mod _ftol;
pub use _ftol::_ftol;

mod _ftol2;
pub use _ftol2::_ftol2;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "__setusermatherr" => msvcrt::__setusermatherr(emu),
        "_ftol" => msvcrt::_ftol(emu),
        "_ftol2" | "_ftol2_sse" => msvcrt::_ftol2(emu, api),
        "abs" | "labs" => msvcrt::abs(emu, api),
        "ceil" => msvcrt::ceil(emu),
        "floor" => msvcrt::floor(emu),
        "sqrt" => msvcrt::sqrt(emu),
        "pow" => msvcrt::pow(emu),
        "log" => msvcrt::log(emu),
        "log10" => msvcrt::log10(emu),
        "exp" => msvcrt::exp(emu),
        "sin" => msvcrt::sin(emu),
        "cos" => msvcrt::cos(emu),
        "tan" => msvcrt::tan(emu),
        "atan" => msvcrt::atan(emu),
        "atan2" => msvcrt::atan2(emu),
        "fabs" => msvcrt::fabs(emu),
        "fmod" => msvcrt::fmod(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
