pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    GetConsoleCP, GetConsoleOutputCP, WriteConsoleA, WriteConsoleW,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

pub use crate::winapi::common::handles::GetConsoleMode;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetConsoleCP" => kernel32::GetConsoleCP(emu),
        "GetConsoleMode" => kernel32::GetConsoleMode(emu),
        "GetConsoleOutputCP" => kernel32::GetConsoleOutputCP(emu),
        "WriteConsoleA" => kernel32::WriteConsoleA(emu),
        "WriteConsoleW" => kernel32::WriteConsoleW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
