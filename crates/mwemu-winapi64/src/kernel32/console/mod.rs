use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

#[path = "../get_console_cp.rs"]
mod get_console_cp;
pub use get_console_cp::*;

#[path = "../get_console_mode.rs"]
mod get_console_mode;
pub use get_console_mode::*;

#[path = "../get_console_output_cp.rs"]
mod get_console_output_cp;
pub use get_console_output_cp::*;

#[path = "../write_console_w.rs"]
mod write_console_w;
pub use write_console_w::*;

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "GetConsoleCP" => kernel32::GetConsoleCP(emu),
        "GetConsoleMode" => kernel32::GetConsoleMode(emu),
        "GetConsoleOutputCP" => kernel32::GetConsoleOutputCP(emu),
        "WriteConsoleW" => kernel32::WriteConsoleW(emu),
        _ => return false,
    }
    true
}
