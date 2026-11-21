pub use crate::winapi::libraries::kernel32::entry_points::kernel32::{
    GetConsoleCP, GetConsoleOutputCP, WriteConsoleA, WriteConsoleW,
};

use crate::api::dispatch_api;
use crate::emu::Emu;
use crate::winapi::common::fallback::unsupported_exports;
use crate::winapi::winapi32::kernel32;

pub use crate::winapi::common::handles::GetConsoleMode;

unsupported_exports!(
    "kernel32",
    CloseConsoleHandle,
    ConsoleMenuControl,
    GetConsoleCharType,
    GetConsoleFontInfo,
    GetConsoleHardwareState,
    GetConsoleInputWaitHandle,
    GetConsoleKeyboardLayoutNameA,
    GetConsoleKeyboardLayoutNameW,
    GetNumberOfConsoleFonts,
    InvalidateConsoleDIBits,
    RegisterConsoleIME,
    RegisterConsoleOS2,
    SetConsoleCursor,
    SetConsoleFont,
    SetConsoleHardwareState,
    SetConsoleIcon,
    SetConsoleKeyShortcuts,
    SetConsoleLocalEUDC,
    SetConsoleMaximumWindowSize,
    SetConsoleMenuClose,
    SetConsoleOS2OemFormat,
    SetConsolePalette,
    ShowConsoleCursor,
    VerifyConsoleIoHandle,
);

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "GetConsoleMode" => kernel32::GetConsoleMode(emu),
        "GetConsoleCP" => kernel32::GetConsoleCP(emu),
        "GetConsoleOutputCP" => kernel32::GetConsoleOutputCP(emu),
        "WriteConsoleW" => kernel32::WriteConsoleW(emu),
        "ConsoleMenuControl" => kernel32::ConsoleMenuControl(emu),
        "GetConsoleCharType" => kernel32::GetConsoleCharType(emu),
        "GetConsoleFontInfo" => kernel32::GetConsoleFontInfo(emu),
        "GetConsoleHardwareState" => kernel32::GetConsoleHardwareState(emu),
        "GetNumberOfConsoleFonts" => kernel32::GetNumberOfConsoleFonts(emu),
        "InvalidateConsoleDIBits" => kernel32::InvalidateConsoleDIBits(emu),
        "NtWow64ConsoleLaunchServerProcess" => kernel32::NtWow64ConsoleLaunchServerProcess(emu),
        "RegisterConsoleIME" => kernel32::RegisterConsoleIME(emu),
        "RegisterConsoleOS2" => kernel32::RegisterConsoleOS2(emu),
        "RegisterConsoleVDM" => kernel32::RegisterConsoleVDM(emu),
        "SetConsoleCursor" => kernel32::SetConsoleCursor(emu),
        "SetConsoleFont" => kernel32::SetConsoleFont(emu),
        "SetConsoleHardwareState" => kernel32::SetConsoleHardwareState(emu),
        "SetConsoleIcon" => kernel32::SetConsoleIcon(emu),
        "SetConsoleKeyShortcuts" => kernel32::SetConsoleKeyShortcuts(emu),
        "SetConsoleLocalEUDC" => kernel32::SetConsoleLocalEUDC(emu),
        "SetConsoleMaximumWindowSize" => kernel32::SetConsoleMaximumWindowSize(emu),
        "SetConsoleMenuClose" => kernel32::SetConsoleMenuClose(emu),
        "SetConsoleOS2OemFormat" => kernel32::SetConsoleOS2OemFormat(emu),
        "SetConsolePalette" => kernel32::SetConsolePalette(emu),
        "ShowConsoleCursor" => kernel32::ShowConsoleCursor(emu),
        "VDMConsoleOperation" => kernel32::VDMConsoleOperation(emu),
        "GetConsoleCursorInfo" => kernel32::GetConsoleCursorInfo(emu),
        "GetConsoleInputExeNameA" => kernel32::GetConsoleInputExeNameA(emu),
        "GetConsoleInputExeNameW" => kernel32::GetConsoleInputExeNameW(emu),
        "GetNumberOfConsoleInputEvents" => kernel32::GetNumberOfConsoleInputEvents(emu),
        "ReadConsoleInputExA" => kernel32::ReadConsoleInputExA(emu),
        "ReadConsoleInputExW" => kernel32::ReadConsoleInputExW(emu),
        "SetConsoleCursorInfo" => kernel32::SetConsoleCursorInfo(emu),
        "SetConsoleInputExeNameA" => kernel32::SetConsoleInputExeNameA(emu),
        "SetConsoleInputExeNameW" => kernel32::SetConsoleInputExeNameW(emu),
        "SetConsoleMode" => kernel32::SetConsoleMode(emu),
        "SetLastConsoleEventActive" => kernel32::SetLastConsoleEventActive(emu),
        "WriteConsoleA" => kernel32::WriteConsoleA(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
