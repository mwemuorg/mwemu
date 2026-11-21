use crate::emu::Emu;
use crate::winapi::winapi64::kernel32;

mod get_console_cp;
pub use get_console_cp::GetConsoleCP;

mod get_console_mode;
pub use get_console_mode::GetConsoleMode;

mod get_console_output_cp;
pub use get_console_output_cp::GetConsoleOutputCP;

mod write_console;
pub use write_console::{WriteConsoleA, WriteConsoleW};

pub(super) fn dispatch(api: &str, addr: u64, emu: &mut Emu) -> bool {
    match api {
        "GetConsoleCP" => kernel32::GetConsoleCP(emu),
        "GetConsoleMode" => kernel32::GetConsoleMode(emu),
        "GetConsoleOutputCP" => kernel32::GetConsoleOutputCP(emu),
        "WriteConsoleA" => kernel32::WriteConsoleA(emu),
        "WriteConsoleW" => kernel32::WriteConsoleW(emu),
        _ => return false,
    }
    true
}
