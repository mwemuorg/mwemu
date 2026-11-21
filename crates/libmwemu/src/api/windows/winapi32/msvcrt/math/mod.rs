use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

mod __setusermatherr;
pub use __setusermatherr::__setusermatherr;

mod _controlfp;
pub use _controlfp::_controlfp;

mod _ftol;
pub use _ftol::_ftol;

mod srand;
pub use srand::srand;

mod rand;
pub use rand::rand;

mod _ftol2;
pub use _ftol2::_ftol2;

mod abs;
pub use abs::abs;

mod ceil;
pub use ceil::ceil;

mod floor;
pub use floor::floor;

mod sqrt;
pub use sqrt::sqrt;

mod pow;
pub use pow::pow;

mod log;
pub use log::log;

mod log10;
pub use log10::log10;

mod exp;
pub use exp::exp;

mod sin;
pub use sin::sin;

mod cos;
pub use cos::cos;

mod tan;
pub use tan::tan;

mod atan;
pub use atan::atan;

mod atan2;
pub use atan2::atan2;

mod fabs;
pub use fabs::fabs;

mod fmod;
pub use fmod::fmod;

mod _controlfp_s;
pub use _controlfp_s::_controlfp_s;

pub(super) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    match api {
        "__setusermatherr" => msvcrt::__setusermatherr(emu),
        "_controlfp" => msvcrt::_controlfp(emu),
        "_ftol" => msvcrt::_ftol(emu),
        "srand" => msvcrt::srand(emu),
        "rand" => msvcrt::rand(emu),
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
        "_controlfp_s" => msvcrt::_controlfp_s(emu),
        _ => return false,
    }
    true
}
