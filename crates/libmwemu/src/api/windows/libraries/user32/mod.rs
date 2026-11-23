mod dispatch;
pub(crate) use dispatch::dispatch;
pub mod exports;

mod map_virtual_key_ex;
pub use map_virtual_key_ex::{MapVirtualKeyExA, MapVirtualKeyExW};

mod get_async_key_state;
pub use get_async_key_state::GetAsyncKeyState;

mod map_virtual_key;
pub use map_virtual_key::{MapVirtualKeyA, MapVirtualKeyW};

mod wsprintf;
pub use wsprintf::{wsprintfA, wsprintfW};

mod rectangles;
pub use rectangles::{
    CopyRect, EqualRect, InflateRect, IntersectRect, IsRectEmpty, OffsetRect, PtInRect, SetRect,
    SetRectEmpty, SubtractRect, UnionRect,
};

mod window_text;
pub use window_text::{
    GetClassNameA, GetClassNameW, GetDlgItemTextA, GetDlgItemTextW, GetWindowTextA,
    GetWindowTextLengthA, GetWindowTextLengthW, GetWindowTextW, SetWindowTextA, SetWindowTextW,
};

mod enumerate_windows;
pub use enumerate_windows::{EnumChildWindows, EnumWindows};

mod window_classes;
pub use window_classes::{
    CreateWindowExA, CreateWindowExW, DestroyWindow, FindWindowA, FindWindowExA, FindWindowExW,
    FindWindowW, RegisterClassA, RegisterClassExA, RegisterClassExW, RegisterClassW,
    UnregisterClassA, UnregisterClassW,
};

mod window_relations;
pub use window_relations::{
    EnableWindow, GetDlgCtrlID, GetDlgItem, GetParent, GetWindowLongA, GetWindowLongPtrA,
    GetWindowLongPtrW, GetWindowLongW, GetWindowThreadProcessId, IsChild, IsWindow,
    IsWindowEnabled, IsWindowUnicode, SetParent, SetWindowLongA, SetWindowLongPtrA,
    SetWindowLongPtrW, SetWindowLongW,
};

mod char_conversion;
pub use char_conversion::{
    CharToOemA, CharToOemBuffA, CharToOemBuffW, CharToOemW, OemToCharA, OemToCharBuffA,
    OemToCharBuffW, OemToCharW,
};

mod input_state;
pub use input_state::{
    GetCursorPos, GetKeyState, GetKeyboardState, GetPhysicalCursorPos, IsGUIThread,
    IsProcessDPIAware, SetKeyboardState, SetProcessDPIAware,
};

mod load_string;
pub use load_string::{LoadStringA, LoadStringW};

mod timers;
pub use timers::{DispatchMessageA, DispatchMessageW, KillTimer, SetTimer};

mod messages;
pub use messages::{
    DefWindowProcA, DefWindowProcW, GetMessageA, GetMessageTime, GetMessageW, PeekMessageA,
    PeekMessageW, PostMessageA, PostMessageW, PostQuitMessage, SendMessageA, SendMessageW,
    TranslateMessage, WaitMessage,
};

mod window_state;
pub use window_state::{
    CloseWindow, GetClientRect, GetDesktopWindow, GetFocus, GetForegroundWindow, GetTopWindow,
    GetWindowRect, IsWindowVisible, SetFocus, SetForegroundWindow, SetWindowPos, ShowWindow,
};

mod dialogs;
pub use dialogs::{MessageBoxA, MessageBoxExA, MessageBoxExW, MessageBoxW};

mod display_profile;
pub use display_profile::{
    AdjustWindowRect, AdjustWindowRectEx, EnumDisplayMonitors, GetCaretBlinkTime,
    GetDoubleClickTime, GetKeyboardLayout, GetKeyboardType, GetMonitorInfoA, GetMonitorInfoW,
    GetSysColor, GetSystemMetrics, MonitorFromPoint, MonitorFromWindow, SystemParametersInfoA,
    SystemParametersInfoW,
};

mod user_objects;
pub use user_objects::{
    CloseDesktop, CopyIcon, CreateMenu, CreatePopupMenu, DeleteMenu, DestroyCursor, DestroyIcon,
    DestroyMenu, GetCursor, GetMenu, GetMenuItemCount, GetProcessWindowStation, GetSubMenu,
    GetUserObjectInformationA, GetUserObjectInformationW, LoadCursorA, LoadCursorW, LoadIconA,
    LoadIconW, LoadImageA, LoadImageW, OpenInputDesktop, SetCursor, SetCursorPos, ShowCursor,
};

mod input_text;
pub use input_text::{
    CharLowerA, CharLowerBuffA, CharLowerBuffW, CharLowerW, CharUpperA, CharUpperBuffA,
    CharUpperBuffW, CharUpperW, ToAscii, ToAsciiEx, ToUnicode, ToUnicodeEx,
};
