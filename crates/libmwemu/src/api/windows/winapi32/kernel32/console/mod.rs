use crate::emu::Emu;
use crate::winapi::winapi32::kernel32;

mod write_console;
pub use write_console::{WriteConsoleA, WriteConsoleW};

mod get_console_keyboard_layout_name;
pub use get_console_keyboard_layout_name::{
    GetConsoleKeyboardLayoutNameA, GetConsoleKeyboardLayoutNameW,
};

mod close_console_handle;
pub use close_console_handle::CloseConsoleHandle;

mod console_menu_control;
pub use console_menu_control::ConsoleMenuControl;

mod get_console_char_type;
pub use get_console_char_type::GetConsoleCharType;

mod get_console_font_info;
pub use get_console_font_info::GetConsoleFontInfo;

mod get_console_hardware_state;
pub use get_console_hardware_state::GetConsoleHardwareState;

mod get_console_input_wait_handle;
pub use get_console_input_wait_handle::GetConsoleInputWaitHandle;

mod get_number_of_console_fonts;
pub use get_number_of_console_fonts::GetNumberOfConsoleFonts;

mod invalidate_console_di_bits;
pub use invalidate_console_di_bits::InvalidateConsoleDIBits;

mod register_console_ime;
pub use register_console_ime::RegisterConsoleIME;

mod register_console_os2;
pub use register_console_os2::RegisterConsoleOS2;

mod set_console_cursor;
pub use set_console_cursor::SetConsoleCursor;

mod set_console_font;
pub use set_console_font::SetConsoleFont;

mod set_console_hardware_state;
pub use set_console_hardware_state::SetConsoleHardwareState;

mod set_console_icon;
pub use set_console_icon::SetConsoleIcon;

mod set_console_key_shortcuts;
pub use set_console_key_shortcuts::SetConsoleKeyShortcuts;

mod set_console_local_eudc;
pub use set_console_local_eudc::SetConsoleLocalEUDC;

mod set_console_maximum_window_size;
pub use set_console_maximum_window_size::SetConsoleMaximumWindowSize;

mod set_console_menu_close;
pub use set_console_menu_close::SetConsoleMenuClose;

mod set_console_os2_oem_format;
pub use set_console_os2_oem_format::SetConsoleOS2OemFormat;

mod set_console_palette;
pub use set_console_palette::SetConsolePalette;

mod show_console_cursor;
pub use show_console_cursor::ShowConsoleCursor;

mod verify_console_io_handle;
pub use verify_console_io_handle::VerifyConsoleIoHandle;

mod get_console_mode;
pub use get_console_mode::GetConsoleMode;

mod get_console_cp;
pub use get_console_cp::GetConsoleCP;

mod get_console_output_cp;
pub use get_console_output_cp::GetConsoleOutputCP;

pub(super) fn dispatch(api: &str, addr: u32, emu: &mut Emu) -> bool {
    match api {
        "GetConsoleMode" => kernel32::GetConsoleMode(emu),
        "GetConsoleCP" => kernel32::GetConsoleCP(emu),
        "GetConsoleOutputCP" => kernel32::GetConsoleOutputCP(emu),
        "WriteConsoleW" => kernel32::WriteConsoleW(emu),
        "ConsoleMenuControl" => kernel32::ConsoleMenuControl(emu),
        "GetConsoleCharType" => kernel32::GetConsoleCharType(emu),
        "GetConsoleFontInfo" => kernel32::GetConsoleFontInfo(emu),
        "GetConsoleHardwareState" => kernel32::GetConsoleHardwareState(emu),
        "GetConsoleKeyboardLayoutNameA" => kernel32::GetConsoleKeyboardLayoutNameA(emu),
        "GetConsoleKeyboardLayoutNameW" => kernel32::GetConsoleKeyboardLayoutNameW(emu),
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
        "WriteConsoleInputVDMA" => kernel32::WriteConsoleInputVDMA(emu),
        "WriteConsoleInputVDMW" => kernel32::WriteConsoleInputVDMW(emu),
        "AddConsoleAliasA" => kernel32::AddConsoleAliasA(emu),
        "AddConsoleAliasW" => kernel32::AddConsoleAliasW(emu),
        "AllocConsole" => kernel32::AllocConsole(emu),
        "AllocConsoleWithOptions" => kernel32::AllocConsoleWithOptions(emu),
        "AttachConsole" => kernel32::AttachConsole(emu),
        "ClosePseudoConsole" => kernel32::ClosePseudoConsole(emu),
        "CreateConsoleScreenBuffer" => kernel32::CreateConsoleScreenBuffer(emu),
        "CreatePseudoConsole" => kernel32::CreatePseudoConsole(emu),
        "FillConsoleOutputAttribute" => kernel32::FillConsoleOutputAttribute(emu),
        "FillConsoleOutputCharacterA" => kernel32::FillConsoleOutputCharacterA(emu),
        "FillConsoleOutputCharacterW" => kernel32::FillConsoleOutputCharacterW(emu),
        "FlushConsoleInputBuffer" => kernel32::FlushConsoleInputBuffer(emu),
        "FreeConsole" => kernel32::FreeConsole(emu),
        "GenerateConsoleCtrlEvent" => kernel32::GenerateConsoleCtrlEvent(emu),
        "GetConsoleAliasA" => kernel32::GetConsoleAliasA(emu),
        "GetConsoleAliasExesA" => kernel32::GetConsoleAliasExesA(emu),
        "GetConsoleAliasExesLengthA" => kernel32::GetConsoleAliasExesLengthA(emu),
        "GetConsoleAliasExesLengthW" => kernel32::GetConsoleAliasExesLengthW(emu),
        "GetConsoleAliasExesW" => kernel32::GetConsoleAliasExesW(emu),
        "GetConsoleAliasW" => kernel32::GetConsoleAliasW(emu),
        "GetConsoleAliasesA" => kernel32::GetConsoleAliasesA(emu),
        "GetConsoleAliasesLengthA" => kernel32::GetConsoleAliasesLengthA(emu),
        "GetConsoleAliasesLengthW" => kernel32::GetConsoleAliasesLengthW(emu),
        "GetConsoleAliasesW" => kernel32::GetConsoleAliasesW(emu),
        "GetConsoleCursorInfo" => kernel32::GetConsoleCursorInfo(emu),
        "GetConsoleCursorMode" => kernel32::GetConsoleCursorMode(emu),
        "GetConsoleDisplayMode" => kernel32::GetConsoleDisplayMode(emu),
        "GetConsoleFontSize" => kernel32::GetConsoleFontSize(emu),
        "GetConsoleHistoryInfo" => kernel32::GetConsoleHistoryInfo(emu),
        "GetConsoleInputExeNameA" => kernel32::GetConsoleInputExeNameA(emu),
        "GetConsoleInputExeNameW" => kernel32::GetConsoleInputExeNameW(emu),
        "GetConsoleNlsMode" => kernel32::GetConsoleNlsMode(emu),
        "GetConsoleOriginalTitleA" => kernel32::GetConsoleOriginalTitleA(emu),
        "GetConsoleOriginalTitleW" => kernel32::GetConsoleOriginalTitleW(emu),
        "GetConsoleProcessList" => kernel32::GetConsoleProcessList(emu),
        "GetConsoleScreenBufferInfo" => kernel32::GetConsoleScreenBufferInfo(emu),
        "GetConsoleScreenBufferInfoEx" => kernel32::GetConsoleScreenBufferInfoEx(emu),
        "GetConsoleSelectionInfo" => kernel32::GetConsoleSelectionInfo(emu),
        "GetConsoleTitleA" => kernel32::GetConsoleTitleA(emu),
        "GetConsoleTitleW" => kernel32::GetConsoleTitleW(emu),
        "GetConsoleWindow" => kernel32::GetConsoleWindow(emu),
        "GetCurrentConsoleFont" => kernel32::GetCurrentConsoleFont(emu),
        "GetCurrentConsoleFontEx" => kernel32::GetCurrentConsoleFontEx(emu),
        "GetLargestConsoleWindowSize" => kernel32::GetLargestConsoleWindowSize(emu),
        "GetNumberOfConsoleInputEvents" => kernel32::GetNumberOfConsoleInputEvents(emu),
        "GetNumberOfConsoleMouseButtons" => kernel32::GetNumberOfConsoleMouseButtons(emu),
        "OpenConsoleW" => kernel32::OpenConsoleW(emu),
        "OpenConsoleWStub" => kernel32::OpenConsoleWStub(emu),
        "PeekConsoleInputA" => kernel32::PeekConsoleInputA(emu),
        "PeekConsoleInputW" => kernel32::PeekConsoleInputW(emu),
        "ReadConsoleA" => kernel32::ReadConsoleA(emu),
        "ReadConsoleInputA" => kernel32::ReadConsoleInputA(emu),
        "ReadConsoleInputExA" => kernel32::ReadConsoleInputExA(emu),
        "ReadConsoleInputExW" => kernel32::ReadConsoleInputExW(emu),
        "ReadConsoleInputW" => kernel32::ReadConsoleInputW(emu),
        "ReadConsoleOutputA" => kernel32::ReadConsoleOutputA(emu),
        "ReadConsoleOutputAttribute" => kernel32::ReadConsoleOutputAttribute(emu),
        "ReadConsoleOutputCharacterA" => kernel32::ReadConsoleOutputCharacterA(emu),
        "ReadConsoleOutputCharacterW" => kernel32::ReadConsoleOutputCharacterW(emu),
        "ReadConsoleOutputW" => kernel32::ReadConsoleOutputW(emu),
        "ReadConsoleW" => kernel32::ReadConsoleW(emu),
        "ReleasePseudoConsole" => kernel32::ReleasePseudoConsole(emu),
        "ResizePseudoConsole" => kernel32::ResizePseudoConsole(emu),
        "ScrollConsoleScreenBufferA" => kernel32::ScrollConsoleScreenBufferA(emu),
        "ScrollConsoleScreenBufferW" => kernel32::ScrollConsoleScreenBufferW(emu),
        "SetConsoleActiveScreenBuffer" => kernel32::SetConsoleActiveScreenBuffer(emu),
        "SetConsoleCP" => kernel32::SetConsoleCP(emu),
        "SetConsoleCursorInfo" => kernel32::SetConsoleCursorInfo(emu),
        "SetConsoleCursorMode" => kernel32::SetConsoleCursorMode(emu),
        "SetConsoleCursorPosition" => kernel32::SetConsoleCursorPosition(emu),
        "SetConsoleDisplayMode" => kernel32::SetConsoleDisplayMode(emu),
        "SetConsoleHistoryInfo" => kernel32::SetConsoleHistoryInfo(emu),
        "SetConsoleInputExeNameA" => kernel32::SetConsoleInputExeNameA(emu),
        "SetConsoleInputExeNameW" => kernel32::SetConsoleInputExeNameW(emu),
        "SetConsoleMode" => kernel32::SetConsoleMode(emu),
        "SetConsoleNlsMode" => kernel32::SetConsoleNlsMode(emu),
        "SetConsoleOutputCP" => kernel32::SetConsoleOutputCP(emu),
        "SetConsoleScreenBufferInfoEx" => kernel32::SetConsoleScreenBufferInfoEx(emu),
        "SetConsoleScreenBufferSize" => kernel32::SetConsoleScreenBufferSize(emu),
        "SetConsoleTextAttribute" => kernel32::SetConsoleTextAttribute(emu),
        "SetConsoleTitleA" => kernel32::SetConsoleTitleA(emu),
        "SetConsoleTitleW" => kernel32::SetConsoleTitleW(emu),
        "SetConsoleWindowInfo" => kernel32::SetConsoleWindowInfo(emu),
        "SetCurrentConsoleFontEx" => kernel32::SetCurrentConsoleFontEx(emu),
        "SetLastConsoleEventActive" => kernel32::SetLastConsoleEventActive(emu),
        "UnregisterConsoleIME" => kernel32::UnregisterConsoleIME(emu),
        "WTSGetActiveConsoleSessionId" => kernel32::WTSGetActiveConsoleSessionId(emu),
        "WriteConsoleA" => kernel32::WriteConsoleA(emu),
        "WriteConsoleInputA" => kernel32::WriteConsoleInputA(emu),
        "WriteConsoleInputW" => kernel32::WriteConsoleInputW(emu),
        "WriteConsoleOutputA" => kernel32::WriteConsoleOutputA(emu),
        "WriteConsoleOutputAttribute" => kernel32::WriteConsoleOutputAttribute(emu),
        "WriteConsoleOutputCharacterA" => kernel32::WriteConsoleOutputCharacterA(emu),
        "WriteConsoleOutputCharacterW" => kernel32::WriteConsoleOutputCharacterW(emu),
        "WriteConsoleOutputW" => kernel32::WriteConsoleOutputW(emu),
        _ => return false,
    }
    true
}
