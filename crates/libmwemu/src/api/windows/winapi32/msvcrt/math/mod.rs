use crate::emu::Emu;
use crate::winapi::winapi32::msvcrt;

#[path = "../__setusermatherr.rs"]
mod __setusermatherr;
pub use __setusermatherr::__setusermatherr;

#[path = "../_controlfp.rs"]
mod _controlfp;
pub use _controlfp::_controlfp;

#[path = "../_ftol.rs"]
mod _ftol;
pub use _ftol::_ftol;

#[path = "../srand.rs"]
mod srand;
pub use srand::srand;

#[path = "../rand.rs"]
mod rand;
pub use rand::rand;

#[path = "../_ftol2.rs"]
mod _ftol2;
pub use _ftol2::_ftol2;

#[path = "../abs.rs"]
mod abs;
pub use abs::abs;

#[path = "../ceil.rs"]
mod ceil;
pub use ceil::ceil;

#[path = "../floor.rs"]
mod floor;
pub use floor::floor;

#[path = "../sqrt.rs"]
mod sqrt;
pub use sqrt::sqrt;

#[path = "../pow.rs"]
mod pow;
pub use pow::pow;

#[path = "../log.rs"]
mod log;
pub use log::log;

#[path = "../log10.rs"]
mod log10;
pub use log10::log10;

#[path = "../exp.rs"]
mod exp;
pub use exp::exp;

#[path = "../sin.rs"]
mod sin;
pub use sin::sin;

#[path = "../cos.rs"]
mod cos;
pub use cos::cos;

#[path = "../tan.rs"]
mod tan;
pub use tan::tan;

#[path = "../atan.rs"]
mod atan;
pub use atan::atan;

#[path = "../atan2.rs"]
mod atan2;
pub use atan2::atan2;

#[path = "../fabs.rs"]
mod fabs;
pub use fabs::fabs;

#[path = "../fmod.rs"]
mod fmod;
pub use fmod::fmod;

#[path = "../_controlfp_s.rs"]
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
