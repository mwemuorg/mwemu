use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::kernel32;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "MessageBoxA" => MessageBoxA(emu),
        "MessageBoxW" => MessageBoxW(emu),
        "GetDesktopWindow" => GetDesktopWindow(emu),
        "wsprintfW" => wsprintfW(emu),
        "GetProcessWindowStation" => GetProcessWindowStation(emu),
        "GetUserObjectInformationW" => GetUserObjectInformationW(emu),
        "CharLowerW" => CharLowerW(emu),
        "wsprintfA" => wsprintfA(emu),
        "GetSystemMetrics" => GetSystemMetrics(emu),
        "SystemParametersInfoA" => SystemParametersInfoA(emu),
        "LoadIconA" => LoadIconA(emu),
        "LoadCursorA" => LoadCursorA(emu),
        "RegisterClassA" => RegisterClassA(emu),
        "RegisterClassW" => RegisterClassW(emu),
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "CharLowerBuffA" => CharLowerBuffA(emu),
        "CharLowerBuffW" => CharLowerBuffW(emu),
        "CharUpperBuffA" => CharUpperBuffA(emu),
        "CharUpperBuffW" => CharUpperBuffW(emu),
        "FindWindowA" => FindWindowA(emu),
        "FindWindowW" => FindWindowW(emu),
        "GetKeyState" => GetKeyState(emu),
        "GetAsyncKeyState" => GetAsyncKeyState(emu),
        "SetWindowsHookExA" => SetWindowsHookExA(emu),
        "AdjustWindowRect" => AdjustWindowRect(emu),
        "AdjustWindowRectEx" => AdjustWindowRectEx(emu),
        "CreateWindowExA" => CreateWindowExA(emu),
        "CreateWindowExW" => CreateWindowExW(emu),
        "ShowWindow" => ShowWindow(emu),
        "UpdateWindow" => UpdateWindow(emu),
        "DestroyWindow" => DestroyWindow(emu),
        "PostMessageA" => PostMessageA(emu),
        "PostMessageW" => PostMessageW(emu),
        "SendMessageA" => SendMessageA(emu),
        "SendMessageW" => SendMessageW(emu),
        "DefWindowProcA" => DefWindowProcA(emu),
        "DefWindowProcW" => DefWindowProcW(emu),
        "GetWindowRect" => GetWindowRect(emu),
        "GetClientRect" => GetClientRect(emu),
        "SetWindowPos" => SetWindowPos(emu),
        "SetWindowTextA" => SetWindowTextA(emu),
        "SetWindowTextW" => SetWindowTextW(emu),
        "GetWindowTextA" => GetWindowTextA(emu),
        "GetWindowTextW" => GetWindowTextW(emu),
        "GetWindowLongA" => GetWindowLongA(emu),
        "GetWindowLongW" => GetWindowLongW(emu),
        "SetWindowLongA" => SetWindowLongA(emu),
        "SetWindowLongW" => SetWindowLongW(emu),
        "PeekMessageA" => PeekMessageA(emu),
        "PeekMessageW" => PeekMessageW(emu),
        "GetMessageA" => GetMessageA(emu),
        "GetMessageW" => GetMessageW(emu),
        "TranslateMessage" => TranslateMessage(emu),
        "DispatchMessageA" => DispatchMessageA(emu),
        "DispatchMessageW" => DispatchMessageW(emu),
        "PostQuitMessage" => PostQuitMessage(emu),
        "IsWindow" => IsWindow(emu),
        "IsWindowVisible" => IsWindowVisible(emu),
        "EnableWindow" => EnableWindow(emu),
        "SetFocus" => SetFocus(emu),
        "GetFocus" => GetFocus(emu),
        "GetForegroundWindow" => GetForegroundWindow(emu),
        "SetForegroundWindow" => SetForegroundWindow(emu),
        "GetCursorPos" => GetCursorPos(emu),
        "SetCursorPos" => SetCursorPos(emu),
        "SetCursor" => SetCursor(emu),
        "ShowCursor" => ShowCursor(emu),
        "LoadStringA" => LoadStringA(emu),
        "LoadStringW" => LoadStringW(emu),
        "GetClassInfoExA" => GetClassInfoExA(emu),
        "GetClassInfoExW" => GetClassInfoExW(emu),
        "MapVirtualKeyA" => MapVirtualKeyA(emu),
        "MapVirtualKeyW" => MapVirtualKeyW(emu),
        "GetKeyboardLayout" => GetKeyboardLayout(emu),
        "GetKeyboardType" => GetKeyboardType(emu),
        "BeginPaint" => BeginPaint(emu),
        "EndPaint" => EndPaint(emu),
        "InvalidateRect" => InvalidateRect(emu),
        "FillRect" => FillRect(emu),
        "DrawTextA" => DrawTextA(emu),
        "DrawTextW" => DrawTextW(emu),
        "ActivateKeyboardLayout" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ActivateKeyboardLayout arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("activatekeyboardlayout");
            stdcall_ret(emu, 2);
        }
        "AddClipboardFormatListener" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AddClipboardFormatListener arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("addclipboardformatlistener");
            stdcall_ret(emu, 2);
        }
        "AddVisualIdentifier" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AddVisualIdentifier arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("addvisualidentifier");
            stdcall_ret(emu, 2);
        }
        "AdjustWindowRectExForDpi" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AdjustWindowRectExForDpi arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AlignRects" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!AlignRects arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AllowForegroundActivation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AllowForegroundActivation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AllowSetForegroundWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AllowSetForegroundWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AnimateWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!AnimateWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "AnyPopup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!AnyPopup arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AppendMenuA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!AppendMenuA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "AppendMenuW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!AppendMenuW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "ApplyWindowAction" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ApplyWindowAction arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AreDpiAwarenessContextsEqual" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AreDpiAwarenessContextsEqual arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ArrangeIconicWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ArrangeIconicWindows arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "AttachThreadInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!AttachThreadInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("attachthreadinput");
            stdcall_ret(emu, 2);
        }
        "BeginDeferWindowPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BeginDeferWindowPos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("begindeferwindowpos");
            stdcall_ret(emu, 2);
        }
        "BlockInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!BlockInput arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "BringWindowToTop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BringWindowToTop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "BroadcastSystemMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BroadcastSystemMessage arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "BroadcastSystemMessageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BroadcastSystemMessageA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "BroadcastSystemMessageExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BroadcastSystemMessageExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "BroadcastSystemMessageExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BroadcastSystemMessageExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "BroadcastSystemMessageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BroadcastSystemMessageW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "BuildReasonArray" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!BuildReasonArray arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CalcMenuBar" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CalcMenuBar arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CalculatePopupWindowPosition" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CalculatePopupWindowPosition arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CallMsgFilter" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CallMsgFilter arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CallMsgFilterA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CallMsgFilterA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CallMsgFilterW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CallMsgFilterW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CallNextHookEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CallNextHookEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "CallWindowProcA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CallWindowProcA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "CallWindowProcW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CallWindowProcW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "CancelShutdown" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CancelShutdown handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CascadeChildWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CascadeChildWindows arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CascadeWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CascadeWindows arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeClipboardChain" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeClipboardChain arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeDisplaySettingsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeDisplaySettingsA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeDisplaySettingsExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeDisplaySettingsExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "ChangeDisplaySettingsExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeDisplaySettingsExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "ChangeDisplaySettingsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeDisplaySettingsW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeMenuA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ChangeMenuA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeMenuW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ChangeMenuW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeWindowMessageFilter" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeWindowMessageFilter arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChangeWindowMessageFilterEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChangeWindowMessageFilterEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CharLowerA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharLowerA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CharNextA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharNextA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CharNextExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharNextExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CharNextW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharNextW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CharPrevA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharPrevA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CharPrevExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharPrevExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CharPrevW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharPrevW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CharToOemA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharToOemA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CharToOemBuffA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharToOemBuffA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CharToOemBuffW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharToOemBuffW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CharToOemW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharToOemW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CharUpperA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharUpperA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CharUpperW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CharUpperW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CheckBannedOneCoreTransformApi" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckBannedOneCoreTransformApi arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CheckDBCSEnabledExt" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckDBCSEnabledExt arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("checkdbcsenabledext");
            stdcall_ret(emu, 2);
        }
        "CheckDlgButton" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CheckDlgButton arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CheckMenuItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CheckMenuItem arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CheckMenuRadioItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckMenuRadioItem arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CheckProcessForClipboardAccess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckProcessForClipboardAccess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CheckProcessSession" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckProcessSession arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CheckRadioButton" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckRadioButton arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "CheckWindowThreadDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CheckWindowThreadDesktop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ChildWindowFromPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChildWindowFromPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "ChildWindowFromPointEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ChildWindowFromPointEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "CliImmSetHotKey" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CliImmSetHotKey arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ClientThreadSetup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ClientThreadSetup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ClientToScreen" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ClientToScreen arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ClipCursor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ClipCursor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CloseClipboard" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CloseClipboard handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CloseDesktop" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CloseDesktop handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CloseGestureInfoHandle" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CloseGestureInfoHandle handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CloseTouchInputHandle" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CloseTouchInputHandle handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CloseWindow" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CloseWindow handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "CloseWindowStation" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!CloseWindowStation handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "ConsoleControl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ConsoleControl arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ControlMagnification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ControlMagnification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ConvertPrimaryPointerToMouseDrag" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ConvertPrimaryPointerToMouseDrag arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "ConvertToInterceptWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ConvertToInterceptWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CopyAcceleratorTableA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CopyAcceleratorTableA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CopyAcceleratorTableW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CopyAcceleratorTableW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CopyIcon" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CopyIcon arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("copyicon");
            stdcall_ret(emu, 1);
        }
        "CopyImage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CopyImage arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("copyimage");
            stdcall_ret(emu, 2);
        }
        "CopyRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CopyRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CountClipboardFormats" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CountClipboardFormats arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "CreateAcceleratorTableA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateAcceleratorTableA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createacceleratortablea");
            stdcall_ret(emu, 2);
        }
        "CreateAcceleratorTableW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateAcceleratorTableW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createacceleratortablew");
            stdcall_ret(emu, 2);
        }
        "CreateCaret" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CreateCaret arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("createcaret");
            stdcall_ret(emu, 1);
        }
        "CreateCursor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CreateCursor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("createcursor");
            stdcall_ret(emu, 8);
        }
        "CreateDCompositionHwndTarget" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDCompositionHwndTarget arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createdcompositionhwndtarget");
            stdcall_ret(emu, 1);
        }
        "CreateDesktopA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CreateDesktopA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateDesktopExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDesktopExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateDesktopExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDesktopExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateDesktopW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CreateDesktopW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateDialogIndirectParamA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createdialogindirectparama");
            stdcall_ret(emu, 6);
        }
        "CreateDialogIndirectParamAorW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamAorW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "CreateDialogIndirectParamW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createdialogindirectparamw");
            stdcall_ret(emu, 6);
        }
        "CreateDialogParamA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDialogParamA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createdialogparama");
            stdcall_ret(emu, 6);
        }
        "CreateDialogParamW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateDialogParamW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createdialogparamw");
            stdcall_ret(emu, 6);
        }
        "CreateIcon" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CreateIcon arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("createicon");
            stdcall_ret(emu, 8);
        }
        "CreateIconFromResource" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateIconFromResource arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createiconfromresource");
            stdcall_ret(emu, 4);
        }
        "CreateIconFromResourceEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateIconFromResourceEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createiconfromresourceex");
            stdcall_ret(emu, 8);
        }
        "CreateIconIndirect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateIconIndirect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createiconindirect");
            stdcall_ret(emu, 1);
        }
        "CreateMDIWindowA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateMDIWindowA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createmdiwindowa");
            stdcall_ret(emu, 4);
        }
        "CreateMDIWindowW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateMDIWindowW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createmdiwindoww");
            stdcall_ret(emu, 4);
        }
        "CreateMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CreateMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("createmenu");
            stdcall_ret(emu, 2);
        }
        "CreatePopupMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreatePopupMenu arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createpopupmenu");
            stdcall_ret(emu, 2);
        }
        "CreateSyntheticPointerDevice" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateSyntheticPointerDevice arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createsyntheticpointerdevice");
            stdcall_ret(emu, 1);
        }
        "CreateSyntheticPointerDevice2" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateSyntheticPointerDevice2 arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createsyntheticpointerdevice2");
            stdcall_ret(emu, 1);
        }
        "CreateSystemThreads" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateSystemThreads arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createsystemthreads");
            stdcall_ret(emu, 1);
        }
        "CreateWindowInBand" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateWindowInBand arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createwindowinband");
            stdcall_ret(emu, 1);
        }
        "CreateWindowInBandEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateWindowInBandEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createwindowinbandex");
            stdcall_ret(emu, 1);
        }
        "CreateWindowIndirect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateWindowIndirect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createwindowindirect");
            stdcall_ret(emu, 1);
        }
        "CreateWindowStationA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateWindowStationA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createwindowstationa");
            stdcall_ret(emu, 4);
        }
        "CreateWindowStationW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CreateWindowStationW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("createwindowstationw");
            stdcall_ret(emu, 4);
        }
        "CsrBroadcastSystemMessageExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!CsrBroadcastSystemMessageExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "CtxInitUser32" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!CtxInitUser32 arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("ctxinituser32");
            stdcall_ret(emu, 2);
        }
        "DdeAbandonTransaction" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DdeAbandonTransaction handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeAccessData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeAccessData arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeAddData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeAddData arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("ddeadddata");
            stdcall_ret(emu, 2);
        }
        "DdeClientTransaction" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeClientTransaction arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeCmpStringHandles" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeCmpStringHandles arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeConnect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeConnect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("ddeconnect");
            stdcall_ret(emu, 2);
        }
        "DdeConnectList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeConnectList arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("ddeconnectlist");
            stdcall_ret(emu, 2);
        }
        "DdeCreateDataHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeCreateDataHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("ddecreatedatahandle");
            stdcall_ret(emu, 2);
        }
        "DdeCreateStringHandleA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeCreateStringHandleA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeCreateStringHandleW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeCreateStringHandleW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeDisconnect" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DdeDisconnect handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeDisconnectList" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DdeDisconnectList handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeEnableCallback" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeEnableCallback arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeFreeDataHandle" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DdeFreeDataHandle handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeFreeStringHandle" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DdeFreeStringHandle handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeGetData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeGetData arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeGetLastError" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeGetLastError arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeGetQualityOfService" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeGetQualityOfService arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeImpersonateClient" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeImpersonateClient arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeInitializeA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeInitializeA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeInitializeW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeInitializeW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeKeepStringHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeKeepStringHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeNameService" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeNameService arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdePostAdvise" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdePostAdvise arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeQueryConvInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeQueryConvInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeQueryNextServer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeQueryNextServer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeQueryStringA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeQueryStringA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeQueryStringW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeQueryStringW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeReconnect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DdeReconnect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeSetQualityOfService" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeSetQualityOfService arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeSetUserHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeSetUserHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeUnaccessData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DdeUnaccessData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DdeUninitialize" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DdeUninitialize handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefDlgProcA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DefDlgProcA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefDlgProcW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DefDlgProcW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefFrameProcA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DefFrameProcA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefFrameProcW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DefFrameProcW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefMDIChildProcA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DefMDIChildProcA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefMDIChildProcW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DefMDIChildProcW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DefRawInputProc" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DefRawInputProc arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeferWindowPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DeferWindowPos arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeferWindowPosAndBand" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DeferWindowPosAndBand arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DelegateInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DelegateInput arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DeleteMenu" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DeleteMenu handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "DeregisterShellHookWindow" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!DeregisterShellHookWindow handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DestroyAcceleratorTable" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DestroyAcceleratorTable handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroyCaret" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DestroyCaret handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroyCursor" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DestroyCursor handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroyDCompositionHwndTarget" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!DestroyDCompositionHwndTarget handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroyIcon" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DestroyIcon handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroyMenu" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DestroyMenu handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroyReasons" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DestroyReasons handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DestroySyntheticPointerDevice" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!DestroySyntheticPointerDevice handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DialogBoxIndirectParamA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DialogBoxIndirectParamA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "DialogBoxIndirectParamAorW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DialogBoxIndirectParamAorW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DialogBoxIndirectParamW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DialogBoxIndirectParamW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "DialogBoxParamA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DialogBoxParamA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "DialogBoxParamW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DialogBoxParamW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "DisableProcessWindowsGhosting" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!DisableProcessWindowsGhosting handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DisplayConfigGetDeviceInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DisplayConfigGetDeviceInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DisplayConfigSetDeviceInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DisplayConfigSetDeviceInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DisplayExitWindowsWarnings" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DisplayExitWindowsWarnings arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirListA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DlgDirListA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirListComboBoxA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DlgDirListComboBoxA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirListComboBoxW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DlgDirListComboBoxW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirListW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DlgDirListW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirSelectComboBoxExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DlgDirSelectComboBoxExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirSelectComboBoxExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DlgDirSelectComboBoxExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirSelectExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DlgDirSelectExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DlgDirSelectExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DlgDirSelectExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DoSoundConnect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DoSoundConnect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("dosoundconnect");
            stdcall_ret(emu, 2);
        }
        "DoSoundDisconnect" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!DoSoundDisconnect handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DragDetect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DragDetect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DragObject" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DragObject arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawAnimatedRects" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DrawAnimatedRects arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawCaption" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawCaption arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawCaptionTempA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DrawCaptionTempA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawCaptionTempW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DrawCaptionTempW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawEdge" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawEdge arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "DrawFocusRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawFocusRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawFrame" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawFrame arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawFrameControl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DrawFrameControl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "DrawIcon" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawIcon arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "DrawIconEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawIconEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "DrawMenuBar" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawMenuBar arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DrawMenuBarTemp" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DrawMenuBarTemp arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DrawStateA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawStateA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "DrawStateW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawStateW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "DrawTextExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawTextExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 7);
        }
        "DrawTextExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DrawTextExW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 7);
        }
        "DwmGetDxRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!DwmGetDxRgn arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DwmGetDxSharedSurface" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmGetDxSharedSurface arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DwmGetRemoteSessionOcclusionEvent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmGetRemoteSessionOcclusionEvent arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "DwmGetRemoteSessionOcclusionState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmGetRemoteSessionOcclusionState arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DwmKernelShutdown" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmKernelShutdown arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DwmKernelStartup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmKernelStartup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("dwmkernelstartup");
            stdcall_ret(emu, 2);
        }
        "DwmLockScreenUpdates" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmLockScreenUpdates arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("dwmlockscreenupdates");
            stdcall_ret(emu, 1);
        }
        "DwmValidateWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmValidateWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "DwmWindowNotificationsEnabled" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!DwmWindowNotificationsEnabled arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("dwmwindownotificationsenabled");
            stdcall_ret(emu, 2);
        }
        "EditWndProc" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EditWndProc arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EmptyClipboard" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EmptyClipboard arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EnableMenuItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnableMenuItem arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("enablemenuitem");
            stdcall_ret(emu, 3);
        }
        "EnableMouseInPointer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnableMouseInPointer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("enablemouseinpointer");
            stdcall_ret(emu, 2);
        }
        "EnableNonClientDpiScaling" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnableNonClientDpiScaling arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("enablenonclientdpiscaling");
            stdcall_ret(emu, 2);
        }
        "EnableOneCoreTransformMode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnableOneCoreTransformMode arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("enableonecoretransformmode");
            stdcall_ret(emu, 2);
        }
        "EnableScrollBar" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnableScrollBar arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("enablescrollbar");
            stdcall_ret(emu, 3);
        }
        "EnableSessionForMMCSS" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnableSessionForMMCSS arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("enablesessionformmcss");
            stdcall_ret(emu, 2);
        }
        "EnableWindowShellWindowManagementBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnableWindowShellWindowManagementBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax =
                helper::handler_create("enablewindowshellwindowmanagementbehavior");
            stdcall_ret(emu, 2);
        }
        "EndDeferWindowPos" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!EndDeferWindowPos handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EndDeferWindowPosEx" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!EndDeferWindowPosEx handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EndDialog" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!EndDialog handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EndMenu" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!EndMenu handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EndTask" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!EndTask handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EnterMoveSizeLoop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnterMoveSizeLoop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("entermovesizeloop");
            stdcall_ret(emu, 2);
        }
        "EnterReaderModeHelper" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnterReaderModeHelper arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("enterreadermodehelper");
            stdcall_ret(emu, 2);
        }
        "EnumChildWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumChildWindows arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "EnumClipboardFormats" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumClipboardFormats arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 1);
        }
        "EnumDesktopWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDesktopWindows arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EnumDesktopsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumDesktopsA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDesktopsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumDesktopsW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDisplayDevicesA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplayDevicesA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDisplayDevicesW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplayDevicesW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDisplayMonitors" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplayMonitors arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDisplaySettingsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplaySettingsA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "EnumDisplaySettingsExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplaySettingsExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDisplaySettingsExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplaySettingsExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumDisplaySettingsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumDisplaySettingsW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "EnumPropsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumPropsA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumPropsExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumPropsExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumPropsExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumPropsExW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumPropsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumPropsW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumThreadWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumThreadWindows arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "EnumWindowStationsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumWindowStationsA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EnumWindowStationsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EnumWindowStationsW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "EqualRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EqualRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EvaluateProximityToPolygon" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EvaluateProximityToPolygon arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EvaluateProximityToRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!EvaluateProximityToRect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ExcludeUpdateRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ExcludeUpdateRgn arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ExitWindowsEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ExitWindowsEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FlashWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!FlashWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "FlashWindowEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!FlashWindowEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "FrameRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!FrameRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "FreeDDElParam" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!FreeDDElParam handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "FrostCrashedWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!FrostCrashedWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetActiveWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetActiveWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("getactivewindow");
            stdcall_ret(emu, 2);
        }
        "GetAltTabInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetAltTabInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetAltTabInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetAltTabInfoA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetAltTabInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetAltTabInfoW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetAncestor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetAncestor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetAppCompatFlags" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetAppCompatFlags arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetAppCompatFlags2" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetAppCompatFlags2 arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetAutoRotationState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetAutoRotationState arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetAwarenessFromDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetAwarenessFromDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetCIMSSM" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetCIMSSM arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetCapture" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetCapture arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetCaretBlinkTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetCaretBlinkTime arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetCaretPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetCaretPos arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClassInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassInfoA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetClassInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassInfoW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetClassLongA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassLongA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClassLongPtrA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClassLongPtrA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClassLongPtrW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClassLongPtrW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClassLongW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassLongW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClassNameA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassNameA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetClassNameW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassNameW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetClassWord" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClassWord arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClipCursor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetClipCursor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClipboardAccessToken" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardAccessToken arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClipboardData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("getclipboarddata");
            stdcall_ret(emu, 1);
        }
        "GetClipboardFormatNameA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardFormatNameA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetClipboardFormatNameW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardFormatNameW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetClipboardMetadata" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardMetadata arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClipboardOwner" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardOwner arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetClipboardSequenceNumber" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardSequenceNumber arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetClipboardViewer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetClipboardViewer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetComboBoxInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetComboBoxInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetCurrentInputMessageSource" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetCurrentInputMessageSource arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetCurrentMonitorTopologyId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetCurrentMonitorTopologyId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetCursor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetCursor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getcursor");
            stdcall_ret(emu, 2);
        }
        "GetCursorFrameInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetCursorFrameInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetCursorInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetCursorInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDCEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetDCEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getdcex");
            stdcall_ret(emu, 3);
        }
        "GetDCompositionHwndBitmap" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDCompositionHwndBitmap arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDesktopID" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetDesktopID arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDialogBaseUnits" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDialogBaseUnits arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDialogControlDpiChangeBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDialogControlDpiChangeBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDialogDpiChangeBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDialogDpiChangeBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDisplayAutoRotationPreferences" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDisplayAutoRotationPreferences arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDisplayConfigBufferSizes" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDisplayConfigBufferSizes arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDlgCtrlID" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetDlgCtrlID arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetDlgItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetDlgItem arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getdlgitem");
            stdcall_ret(emu, 2);
        }
        "GetDlgItemInt" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetDlgItemInt arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getdlgitemint");
            stdcall_ret(emu, 2);
        }
        "GetDlgItemTextA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDlgItemTextA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "GetDlgItemTextW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDlgItemTextW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 4);
        }
        "GetDoubleClickTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDoubleClickTime arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetDpiAwarenessContextForProcess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDpiAwarenessContextForProcess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDpiForMonitorInternal" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDpiForMonitorInternal arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDpiForSystem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDpiForSystem arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDpiForWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDpiForWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetDpiFromDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetDpiFromDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetExtendedPointerDeviceProperty" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetExtendedPointerDeviceProperty arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetGUIThreadInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetGUIThreadInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetGestureConfig" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetGestureConfig arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetGestureExtraArgs" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetGestureExtraArgs arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetGestureInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetGestureInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetGuiResources" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetGuiResources arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetIconInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetIconInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetIconInfoExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetIconInfoExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetIconInfoExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetIconInfoExW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetInputDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetInputDesktop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetInputLocaleInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetInputLocaleInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetInputState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetInputState arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetInternalWindowPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetInternalWindowPos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetKBCodePage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetKBCodePage arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetKeyNameTextA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetKeyNameTextA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetKeyNameTextW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetKeyNameTextW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetKeyboardLayoutList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetKeyboardLayoutList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetKeyboardLayoutNameA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetKeyboardLayoutNameA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetKeyboardLayoutNameW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetKeyboardLayoutNameW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetKeyboardState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetKeyboardState arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetLastActivePopup" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetLastActivePopup arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetLastInputInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetLastInputInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetLayeredWindowAttributes" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetLayeredWindowAttributes arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetListBoxInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetListBoxInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMagnificationDesktopColorEffect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopColorEffect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMagnificationDesktopMagnification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopMagnification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMagnificationDesktopSamplingMode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopSamplingMode arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMagnificationLensCtxInformation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationLensCtxInformation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getmenu");
            stdcall_ret(emu, 1);
        }
        "GetMenuBarInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenuBarInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuCheckMarkDimensions" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuCheckMarkDimensions arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuContextHelpId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuContextHelpId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuDefaultItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuDefaultItem arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenuInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuItemCount" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuItemCount arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetMenuItemID" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenuItemID arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuItemInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuItemInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "GetMenuItemInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuItemInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "GetMenuItemRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMenuItemRect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenuState arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMenuStringA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenuStringA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetMenuStringW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMenuStringW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetMessageExtraInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMessageExtraInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetMessagePos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMessagePos arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetMessageTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetMessageTime arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMonitorInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMonitorInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMonitorInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMonitorInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetMouseMovePointsEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetMouseMovePointsEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetNextDlgGroupItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetNextDlgGroupItem arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetNextDlgTabItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetNextDlgTabItem arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetOpenClipboardWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetOpenClipboardWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("getopenclipboardwindow");
            stdcall_ret(emu, 2);
        }
        "GetParent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetParent arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getparent");
            stdcall_ret(emu, 1);
        }
        "GetPointerCursorId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerCursorId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDevice" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDevice arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDeviceCursors" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceCursors arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDeviceInputSpace" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceInputSpace arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDeviceOrientation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceOrientation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDeviceProperties" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceProperties arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDeviceRects" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceRects arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerDevices" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerDevices arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameArrivalTimes" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameArrivalTimes arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFramePenInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFramePenInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFramePenInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFramePenInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameTimes" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTimes arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameTouchInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameTouchInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameTouchpadInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchpadInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerFrameTouchpadInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchpadInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetPointerInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerInputTransform" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerInputTransform arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerPenInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerPenInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerPenInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerPenInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerTouchInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerTouchInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerTouchpadInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchpadInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerTouchpadInfoHistory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchpadInfoHistory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPointerType" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetPointerType arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPriorityClipboardFormat" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPriorityClipboardFormat arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetProcessDefaultLayout" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetProcessDefaultLayout arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetProcessDpiAwarenessInternal" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetProcessDpiAwarenessInternal arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetProcessUIContextInformation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetProcessUIContextInformation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetProgmanWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetProgmanWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetPropA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetPropA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetPropW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetPropW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetQueueStatus" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetQueueStatus arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetRawInputBuffer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRawInputBuffer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetRawInputData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRawInputData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetRawInputDeviceInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRawInputDeviceInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetRawInputDeviceInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRawInputDeviceInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetRawInputDeviceList" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRawInputDeviceList arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetRawPointerDeviceData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRawPointerDeviceData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetReasonTitleFromReasonCode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetReasonTitleFromReasonCode arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetRegisteredRawInputDevices" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetRegisteredRawInputDevices arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("getregisteredrawinputdevices");
            stdcall_ret(emu, 2);
        }
        "GetScrollBarInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetScrollBarInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetScrollInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetScrollInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetScrollPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetScrollPos arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetScrollRange" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetScrollRange arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "GetSendMessageReceiver" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetSendMessageReceiver arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetShellChangeNotifyWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetShellChangeNotifyWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetShellWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetShellWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetSubMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetSubMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getsubmenu");
            stdcall_ret(emu, 2);
        }
        "GetSysColor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetSysColor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "GetSysColorBrush" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetSysColorBrush arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("getsyscolorbrush");
            stdcall_ret(emu, 1);
        }
        "GetSystemDpiForProcess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetSystemDpiForProcess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetSystemMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetSystemMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetSystemMetricsForDpi" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetSystemMetricsForDpi arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetTabbedTextExtentA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetTabbedTextExtentA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetTabbedTextExtentW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetTabbedTextExtentW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetTaskmanWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetTaskmanWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetThreadDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetThreadDesktop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetThreadDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetThreadDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetThreadDpiHostingBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetThreadDpiHostingBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetTitleBarInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetTitleBarInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetTopLevelWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetTopLevelWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetTopWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetTopWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("gettopwindow");
            stdcall_ret(emu, 1);
        }
        "GetTouchInputInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetTouchInputInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetUnpredictedMessagePos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetUnpredictedMessagePos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetUpdateRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetUpdateRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetUpdateRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetUpdateRgn arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetUpdatedClipboardFormats" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetUpdatedClipboardFormats arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetUserObjectInformationA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetUserObjectInformationA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetUserObjectSecurity" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetUserObjectSecurity arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWinStationInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWinStationInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "GetWindowBand" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindowBand arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowCompositionAttribute" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowCompositionAttribute arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowCompositionInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowCompositionInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowContextHelpId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowContextHelpId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowDC" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindowDC arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("getwindowdc");
            stdcall_ret(emu, 1);
        }
        "GetWindowDisplayAffinity" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowDisplayAffinity arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowDpiHostingBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowDpiHostingBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowFeedbackSetting" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowFeedbackSetting arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindowInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowMinimizeRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowMinimizeRect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowModuleFileName" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowModuleFileName arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowModuleFileNameA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowModuleFileNameA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetWindowModuleFileNameW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowModuleFileNameW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetWindowPlacement" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowPlacement arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowProcessHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowProcessHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindowRgn arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowRgnBox" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowRgnBox arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowRgnEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindowRgnEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowTextLengthA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowTextLengthA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetWindowTextLengthW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowTextLengthW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetWindowThreadProcessId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowThreadProcessId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0x123;
            stdcall_ret(emu, 2);
        }
        "GetWindowWord" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GetWindowWord arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GhostWindowFromHungWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GhostWindowFromHungWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GrayStringA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GrayStringA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "GrayStringW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!GrayStringW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "HandleDelegatedInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!HandleDelegatedInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HideCaret" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!HideCaret arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HiliteMenuItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!HiliteMenuItem arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "HungWindowFromGhostWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!HungWindowFromGhostWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IMPGetIMEA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IMPGetIMEA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IMPGetIMEW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IMPGetIMEW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IMPQueryIMEA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IMPQueryIMEA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IMPQueryIMEW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IMPQueryIMEW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IMPSetIMEA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IMPSetIMEA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IMPSetIMEW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IMPSetIMEW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ImpersonateDdeClientWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ImpersonateDdeClientWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InSendMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InSendMessage arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InSendMessageEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InSendMessageEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "InflateRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InflateRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InheritWindowMonitor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InheritWindowMonitor arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InitDManipHook" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InitDManipHook arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("initdmaniphook");
            stdcall_ret(emu, 2);
        }
        "InitializeGenericHidInjection" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InitializeGenericHidInjection arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("initializegenerichidinjection");
            stdcall_ret(emu, 2);
        }
        "InitializeInputDeviceInjection" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InitializeInputDeviceInjection arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("initializeinputdeviceinjection");
            stdcall_ret(emu, 2);
        }
        "InitializeLpkHooks" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InitializeLpkHooks arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("initializelpkhooks");
            stdcall_ret(emu, 2);
        }
        "InitializePointerDeviceInjection" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InitializePointerDeviceInjection arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("initializepointerdeviceinjection");
            stdcall_ret(emu, 2);
        }
        "InitializePointerDeviceInjectionEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InitializePointerDeviceInjectionEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("initializepointerdeviceinjectionex");
            stdcall_ret(emu, 2);
        }
        "InitializeTouchInjection" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InitializeTouchInjection arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("initializetouchinjection");
            stdcall_ret(emu, 2);
        }
        "InjectDeviceInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectDeviceInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InjectGenericHidInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectGenericHidInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InjectKeyboardInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectKeyboardInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InjectMouseInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectMouseInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InjectPointerInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectPointerInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InjectSyntheticPointerInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectSyntheticPointerInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InjectTouchInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InjectTouchInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InputSpaceRegionFromPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InputSpaceRegionFromPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InsertMenuA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InsertMenuA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "InsertMenuItemA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InsertMenuItemA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InsertMenuItemW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InsertMenuItemW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "InsertMenuW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InsertMenuW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "InternalGetWindowIcon" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InternalGetWindowIcon arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InternalGetWindowText" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!InternalGetWindowText arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IntersectRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IntersectRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "InvalidateRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InvalidateRgn arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "InvertRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!InvertRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsCharAlphaA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsCharAlphaA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharAlphaNumericA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsCharAlphaNumericA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharAlphaNumericW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsCharAlphaNumericW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharAlphaW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsCharAlphaW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharLowerA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsCharLowerA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharLowerW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsCharLowerW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharUpperA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsCharUpperA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsCharUpperW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsCharUpperW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsChild" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsChild arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsClipboardFormatAvailable" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsClipboardFormatAvailable arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsDialogMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsDialogMessage arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsDialogMessageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsDialogMessageA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsDialogMessageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsDialogMessageW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsDlgButtonChecked" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsDlgButtonChecked arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsGUIThread" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsGUIThread arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "IsHungAppWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsHungAppWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsIconic" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsIconic arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsImmersiveProcess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsImmersiveProcess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsInDesktopWindowBand" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsInDesktopWindowBand arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsInterceptWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsInterceptWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsMouseInPointerEnabled" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsMouseInPointerEnabled arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("ismouseinpointerenabled");
            stdcall_ret(emu, 2);
        }
        "IsOneCoreTransformMode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsOneCoreTransformMode arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsProcessDPIAware" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsProcessDPIAware arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsRectEmpty" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsRectEmpty arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsSETEnabled" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsSETEnabled arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("issetenabled");
            stdcall_ret(emu, 2);
        }
        "IsServerSideWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsServerSideWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsThreadDesktopComposited" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsThreadDesktopComposited arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsThreadMessageQueueAttached" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsThreadMessageQueueAttached arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("isthreadmessagequeueattached");
            stdcall_ret(emu, 2);
        }
        "IsThreadTSFEventAware" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsThreadTSFEventAware arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "IsTopLevelWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsTopLevelWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsTouchWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsTouchWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsValidDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsValidDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsWinEventHookInstalled" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsWinEventHookInstalled arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("iswineventhookinstalled");
            stdcall_ret(emu, 1);
        }
        "IsWindowArranged" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsWindowArranged arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsWindowDisplayChangeSuppressed" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsWindowDisplayChangeSuppressed arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsWindowEnabled" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsWindowEnabled arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("iswindowenabled");
            stdcall_ret(emu, 1);
        }
        "IsWindowInDestroy" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!IsWindowInDestroy handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsWindowRedirectedForPrint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsWindowRedirectedForPrint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsWindowUnicode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!IsWindowUnicode arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsWow64Message" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsWow64Message arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "IsZoomed" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!IsZoomed arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "KillTimer" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!KillTimer handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LoadAcceleratorsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadAcceleratorsA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadacceleratorsa");
            stdcall_ret(emu, 2);
        }
        "LoadAcceleratorsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadAcceleratorsW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadacceleratorsw");
            stdcall_ret(emu, 2);
        }
        "LoadBitmapA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadBitmapA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadbitmapa");
            stdcall_ret(emu, 2);
        }
        "LoadBitmapW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadBitmapW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadbitmapw");
            stdcall_ret(emu, 2);
        }
        "LoadCursorFromFileA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadCursorFromFileA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadcursorfromfilea");
            stdcall_ret(emu, 1);
        }
        "LoadCursorFromFileW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadCursorFromFileW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadcursorfromfilew");
            stdcall_ret(emu, 1);
        }
        "LoadCursorW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadCursorW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadcursorw");
            stdcall_ret(emu, 2);
        }
        "LoadIconW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadIconW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadiconw");
            stdcall_ret(emu, 2);
        }
        "LoadImageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadImageA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadimagea");
            stdcall_ret(emu, 7);
        }
        "LoadImageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadImageW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadimagew");
            stdcall_ret(emu, 7);
        }
        "LoadKeyboardLayoutA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadKeyboardLayoutA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LoadKeyboardLayoutEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadKeyboardLayoutEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadkeyboardlayoutex");
            stdcall_ret(emu, 2);
        }
        "LoadKeyboardLayoutW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadKeyboardLayoutW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LoadLocalFonts" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadLocalFonts arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadlocalfonts");
            stdcall_ret(emu, 2);
        }
        "LoadMenuA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadMenuA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadmenua");
            stdcall_ret(emu, 1);
        }
        "LoadMenuIndirectA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadMenuIndirectA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadmenuindirecta");
            stdcall_ret(emu, 1);
        }
        "LoadMenuIndirectW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadMenuIndirectW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadmenuindirectw");
            stdcall_ret(emu, 1);
        }
        "LoadMenuW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!LoadMenuW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("loadmenuw");
            stdcall_ret(emu, 1);
        }
        "LoadRemoteFonts" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LoadRemoteFonts arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("loadremotefonts");
            stdcall_ret(emu, 2);
        }
        "LockSetForegroundWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LockSetForegroundWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("locksetforegroundwindow");
            stdcall_ret(emu, 1);
        }
        "LockWindowStation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LockWindowStation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("lockwindowstation");
            stdcall_ret(emu, 1);
        }
        "LockWindowUpdate" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LockWindowUpdate arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("lockwindowupdate");
            stdcall_ret(emu, 1);
        }
        "LockWorkStation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LockWorkStation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("lockworkstation");
            stdcall_ret(emu, 1);
        }
        "LogicalToPhysicalPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LogicalToPhysicalPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LogicalToPhysicalPointForPerMonitorDPI" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LogicalToPhysicalPointForPerMonitorDPI arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LookupIconIdFromDirectory" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LookupIconIdFromDirectory arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "LookupIconIdFromDirectoryEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!LookupIconIdFromDirectoryEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MBToWCSEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MBToWCSEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MBToWCSExt" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MBToWCSExt arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MB_GetString" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MB_GetString arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MITGetCursorUpdateHandle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MITGetCursorUpdateHandle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MITSetLastInputRecipient" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MITSetLastInputRecipient arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MITSynthesizeTouchInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MITSynthesizeTouchInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MakeThreadTSFEventAware" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MakeThreadTSFEventAware arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MapDialogRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MapDialogRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("mapdialogrect");
            stdcall_ret(emu, 2);
        }
        "MapPointsByVisualIdentifier" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MapPointsByVisualIdentifier arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("mappointsbyvisualidentifier");
            stdcall_ret(emu, 2);
        }
        "MapVirtualKeyExA" | "MapVirtualKeyExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MapVirtualKeyEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "MapVisualRelativePoints" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MapVisualRelativePoints arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("mapvisualrelativepoints");
            stdcall_ret(emu, 2);
        }
        "MapWindowPoints" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MapWindowPoints arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("mapwindowpoints");
            stdcall_ret(emu, 4);
        }
        "MenuItemFromPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MenuItemFromPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MenuWindowProcA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MenuWindowProcA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MenuWindowProcW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MenuWindowProcW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MessageBeep" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MessageBeep arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MessageBoxExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MessageBoxExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "MessageBoxExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MessageBoxExW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "MessageBoxIndirectA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MessageBoxIndirectA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "MessageBoxIndirectW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MessageBoxIndirectW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "MessageBoxTimeoutA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MessageBoxTimeoutA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "MessageBoxTimeoutW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MessageBoxTimeoutW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "ModifyMenuA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ModifyMenuA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ModifyMenuW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ModifyMenuW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MonitorFromPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MonitorFromPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("monitorfrompoint");
            stdcall_ret(emu, 3);
        }
        "MonitorFromRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MonitorFromRect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "MonitorFromWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MonitorFromWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("monitorfromwindow");
            stdcall_ret(emu, 2);
        }
        "MoveWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!MoveWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "MsgWaitForMultipleObjects" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MsgWaitForMultipleObjects arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "MsgWaitForMultipleObjectsEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!MsgWaitForMultipleObjectsEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 6);
        }
        "NotifyOverlayWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!NotifyOverlayWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "NotifyWinEvent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!NotifyWinEvent arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "OemKeyScan" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OemKeyScan arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "OemToCharA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OemToCharA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "OemToCharBuffA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OemToCharBuffA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "OemToCharBuffW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OemToCharBuffW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "OemToCharW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OemToCharW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "OffsetRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OffsetRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "OpenClipboard" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OpenClipboard arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("openclipboard");
            stdcall_ret(emu, 1);
        }
        "OpenDesktopA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OpenDesktopA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "OpenDesktopW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OpenDesktopW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "OpenIcon" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!OpenIcon arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("openicon");
            stdcall_ret(emu, 1);
        }
        "OpenInputDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!OpenInputDesktop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("openinputdesktop");
            stdcall_ret(emu, 1);
        }
        "OpenThreadDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!OpenThreadDesktop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("openthreaddesktop");
            stdcall_ret(emu, 1);
        }
        "OpenWindowStationA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!OpenWindowStationA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "OpenWindowStationW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!OpenWindowStationW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "PackDDElParam" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!PackDDElParam arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PackTouchHitTestingProximityEvaluation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PackTouchHitTestingProximityEvaluation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PaintDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!PaintDesktop arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PaintMenuBar" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!PaintMenuBar arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PaintMonitor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!PaintMonitor arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PhysicalToLogicalPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PhysicalToLogicalPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PhysicalToLogicalPointForPerMonitorDPI" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PhysicalToLogicalPointForPerMonitorDPI arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PostThreadMessageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PostThreadMessageA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "PostThreadMessageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PostThreadMessageW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "PrintWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!PrintWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivateExtractIconExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PrivateExtractIconExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivateExtractIconExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PrivateExtractIconExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivateExtractIconsA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PrivateExtractIconsA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivateExtractIconsW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PrivateExtractIconsW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "PrivateRegisterICSProc" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!PrivateRegisterICSProc arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("privateregistericsproc");
            stdcall_ret(emu, 2);
        }
        "PtInRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!PtInRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "QueryBSDRWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!QueryBSDRWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "QueryDisplayConfig" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!QueryDisplayConfig arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "QuerySendMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!QuerySendMessage arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMAddInputObserver" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMAddInputObserver arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("rimaddinputobserver");
            stdcall_ret(emu, 2);
        }
        "RIMAreSiblingDevices" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMAreSiblingDevices arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMDeviceIoControl" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMDeviceIoControl arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMEnableMonitorMappingForDevice" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMEnableMonitorMappingForDevice arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("rimenablemonitormappingfordevice");
            stdcall_ret(emu, 2);
        }
        "RIMFreeInputBuffer" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!RIMFreeInputBuffer handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMGetDevicePreparsedData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMGetDevicePreparsedData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMGetDevicePreparsedDataLockfree" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMGetDevicePreparsedDataLockfree arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("rimgetdevicepreparseddatalockfree");
            stdcall_ret(emu, 2);
        }
        "RIMGetDeviceProperties" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMGetDeviceProperties arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMGetDevicePropertiesLockfree" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMGetDevicePropertiesLockfree arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("rimgetdevicepropertieslockfree");
            stdcall_ret(emu, 2);
        }
        "RIMGetPhysicalDeviceRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMGetPhysicalDeviceRect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMGetSourceProcessId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMGetSourceProcessId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMObserveNextInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMObserveNextInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMOnAsyncPnpWorkNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMOnAsyncPnpWorkNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMOnPnpNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMOnPnpNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMOnTimerNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMOnTimerNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMQueryDevicePath" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMQueryDevicePath arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMReadInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!RIMReadInput arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMRegisterForInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMRegisterForInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("rimregisterforinput");
            stdcall_ret(emu, 2);
        }
        "RIMRegisterForInputEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMRegisterForInputEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("rimregisterforinputex");
            stdcall_ret(emu, 2);
        }
        "RIMRemoveInputObserver" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!RIMRemoveInputObserver handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMSetExtendedDeviceProperty" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMSetExtendedDeviceProperty arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMSetTestModeStatus" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMSetTestModeStatus arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMUnregisterForInput" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!RIMUnregisterForInput handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RIMUpdateInputObserverRegistration" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RIMUpdateInputObserverRegistration arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RaiseLowerShellWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RaiseLowerShellWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RealChildWindowFromPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RealChildWindowFromPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "RealGetWindowClass" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RealGetWindowClass arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RealGetWindowClassA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RealGetWindowClassA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RealGetWindowClassW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RealGetWindowClassW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReasonCodeNeedsBugID" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ReasonCodeNeedsBugID arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReasonCodeNeedsComment" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ReasonCodeNeedsComment arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RecordShutdownReason" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RecordShutdownReason arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RedrawWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!RedrawWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "RegisterBSDRWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterBSDRWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerbsdrwindow");
            stdcall_ret(emu, 2);
        }
        "RegisterClipboardFormatA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterClipboardFormatA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xc000;
            stdcall_ret(emu, 1);
        }
        "RegisterClipboardFormatW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterClipboardFormatW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xc000;
            stdcall_ret(emu, 1);
        }
        "RegisterCloakedNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterCloakedNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registercloakednotification");
            stdcall_ret(emu, 2);
        }
        "RegisterDManipHook" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterDManipHook arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerdmaniphook");
            stdcall_ret(emu, 2);
        }
        "RegisterDeviceNotificationA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterDeviceNotificationA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "RegisterDeviceNotificationW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterDeviceNotificationW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "RegisterErrorReportingDialog" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterErrorReportingDialog arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registererrorreportingdialog");
            stdcall_ret(emu, 2);
        }
        "RegisterForCustomDockTargets" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterForCustomDockTargets arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerforcustomdocktargets");
            stdcall_ret(emu, 2);
        }
        "RegisterForTooltipDismissNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterForTooltipDismissNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerfortooltipdismissnotification");
            stdcall_ret(emu, 2);
        }
        "RegisterFrostWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterFrostWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerfrostwindow");
            stdcall_ret(emu, 2);
        }
        "RegisterGhostWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterGhostWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerghostwindow");
            stdcall_ret(emu, 2);
        }
        "RegisterHotKey" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!RegisterHotKey arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("registerhotkey");
            stdcall_ret(emu, 2);
        }
        "RegisterLogonProcess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterLogonProcess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerlogonprocess");
            stdcall_ret(emu, 2);
        }
        "RegisterMessagePumpHook" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterMessagePumpHook arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registermessagepumphook");
            stdcall_ret(emu, 2);
        }
        "RegisterPointerDeviceNotifications" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterPointerDeviceNotifications arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerpointerdevicenotifications");
            stdcall_ret(emu, 2);
        }
        "RegisterPointerInputTarget" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterPointerInputTarget arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerpointerinputtarget");
            stdcall_ret(emu, 2);
        }
        "RegisterPointerInputTargetEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterPointerInputTargetEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerpointerinputtargetex");
            stdcall_ret(emu, 2);
        }
        "RegisterPowerSettingNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterPowerSettingNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerpowersettingnotification");
            stdcall_ret(emu, 2);
        }
        "RegisterRawInputDevices" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterRawInputDevices arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerrawinputdevices");
            stdcall_ret(emu, 2);
        }
        "RegisterServicesProcess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterServicesProcess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registerservicesprocess");
            stdcall_ret(emu, 2);
        }
        "RegisterSessionPort" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterSessionPort arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registersessionport");
            stdcall_ret(emu, 2);
        }
        "RegisterShellHookWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterShellHookWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registershellhookwindow");
            stdcall_ret(emu, 2);
        }
        "RegisterSuspendResumeNotification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterSuspendResumeNotification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registersuspendresumenotification");
            stdcall_ret(emu, 2);
        }
        "RegisterSystemThread" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterSystemThread arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registersystemthread");
            stdcall_ret(emu, 2);
        }
        "RegisterTasklist" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterTasklist arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registertasklist");
            stdcall_ret(emu, 2);
        }
        "RegisterTouchHitTestingWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchHitTestingWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registertouchhittestingwindow");
            stdcall_ret(emu, 2);
        }
        "RegisterTouchWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registertouchwindow");
            stdcall_ret(emu, 2);
        }
        "RegisterTouchpadCapableThread" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchpadCapableThread arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registertouchpadcapablethread");
            stdcall_ret(emu, 2);
        }
        "RegisterTouchpadCapableWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchpadCapableWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registertouchpadcapablewindow");
            stdcall_ret(emu, 2);
        }
        "RegisterUserApiHook" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterUserApiHook arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("registeruserapihook");
            stdcall_ret(emu, 2);
        }
        "RegisterWindowMessageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterWindowMessageA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "RegisterWindowMessageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterWindowMessageW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "ReleaseCapture" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!ReleaseCapture handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "ReleaseDwmHitTestWaiters" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!ReleaseDwmHitTestWaiters handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "RemoveClipboardFormatListener" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!RemoveClipboardFormatListener handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RemoveInjectionDevice" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!RemoveInjectionDevice handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RemoveMenu" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!RemoveMenu handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "RemovePropA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!RemovePropA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RemovePropW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!RemovePropW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RemoveThreadTSFEventAwareness" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!RemoveThreadTSFEventAwareness handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RemoveVisualIdentifier" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!RemoveVisualIdentifier handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReplyMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ReplyMessage arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReportInertia" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ReportInertia arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ResolveDesktopForWOW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ResolveDesktopForWOW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ReuseDDElParam" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ReuseDDElParam arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ScreenToClient" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ScreenToClient arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ScrollChildren" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ScrollChildren arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ScrollDC" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ScrollDC arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ScrollWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ScrollWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 5);
        }
        "ScrollWindowEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ScrollWindowEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 8);
        }
        "SendDlgItemMessageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendDlgItemMessageA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendDlgItemMessageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendDlgItemMessageW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendIMEMessageExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendIMEMessageExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendIMEMessageExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendIMEMessageExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SendInput arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendMessageCallbackA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendMessageCallbackA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendMessageCallbackW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendMessageCallbackW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendMessageTimeoutA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendMessageTimeoutA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendMessageTimeoutW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendMessageTimeoutW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SendNotifyMessageA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendNotifyMessageA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SendNotifyMessageW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SendNotifyMessageW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SetActiveWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetActiveWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("setactivewindow");
            stdcall_ret(emu, 1);
        }
        "SetAdditionalForegroundBoostProcesses" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetAdditionalForegroundBoostProcesses arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("setadditionalforegroundboostprocesses");
            stdcall_ret(emu, 2);
        }
        "SetCapture" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetCapture arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("setcapture");
            stdcall_ret(emu, 1);
        }
        "SetCaretBlinkTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetCaretBlinkTime arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetCaretPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetCaretPos arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetClassLongA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetClassLongA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetClassLongPtrA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetClassLongPtrA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetClassLongPtrW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetClassLongPtrW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetClassLongW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetClassLongW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetClassWord" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetClassWord arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetClipboardData" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetClipboardData arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetClipboardViewer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetClipboardViewer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetCoalescableTimer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetCoalescableTimer arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "SetCoreWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetCoreWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetCoveredWindowStates" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetCoveredWindowStates arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetCursorContents" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetCursorContents arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDebugErrorLevel" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDebugErrorLevel arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDeskWallpaper" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDeskWallpaper arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDesktopColorTransform" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDesktopColorTransform arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDialogControlDpiChangeBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDialogControlDpiChangeBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDialogDpiChangeBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDialogDpiChangeBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDisplayAutoRotationPreferences" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDisplayAutoRotationPreferences arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDisplayConfig" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDisplayConfig arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetDlgItemInt" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetDlgItemInt arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SetDlgItemTextA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDlgItemTextA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetDlgItemTextW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDlgItemTextW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetDoubleClickTime" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetDoubleClickTime arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetFeatureReportResponse" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetFeatureReportResponse arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetForegroundRedirectionForActivationObject" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetForegroundRedirectionForActivationObject arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetFullscreenMagnifierOffsetsDWMUpdated" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetFullscreenMagnifierOffsetsDWMUpdated arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetGestureConfig" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetGestureConfig arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetInternalWindowPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetInternalWindowPos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetKeyboardState" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetKeyboardState arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SetLastErrorEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetLastErrorEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetLayeredWindowAttributes" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetLayeredWindowAttributes arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMagnificationDesktopColorEffect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopColorEffect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMagnificationDesktopMagnification" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopMagnification arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMagnificationDesktopMagnifierOffsetsDWMUpdated" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopMagnifierOffsetsDWMUpdated arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMagnificationDesktopSamplingMode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopSamplingMode arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMagnificationLensCtxInformation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationLensCtxInformation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMaxTouchpadSensitivity" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMaxTouchpadSensitivity arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMenuContextHelpId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMenuContextHelpId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMenuDefaultItem" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMenuDefaultItem arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMenuInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetMenuInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMenuItemBitmaps" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMenuItemBitmaps arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMenuItemInfoA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMenuItemInfoA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SetMenuItemInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMenuItemInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SetMessageExtraInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMessageExtraInfo arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetMessageQueue" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMessageQueue arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("setmessagequeue");
            stdcall_ret(emu, 2);
        }
        "SetMirrorRendering" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetMirrorRendering arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetParent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetParent arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetPhysicalCursorPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetPhysicalCursorPos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetPointerDeviceInputSpace" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetPointerDeviceInputSpace arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProcessDPIAware" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessDPIAware arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProcessDefaultLayout" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessDefaultLayout arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProcessDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProcessDpiAwarenessInternal" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessDpiAwarenessInternal arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProcessLaunchForegroundPolicy" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessLaunchForegroundPolicy arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("setprocesslaunchforegroundpolicy");
            stdcall_ret(emu, 2);
        }
        "SetProcessRestrictionExemption" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessRestrictionExemption arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProcessWindowStation" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProcessWindowStation arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetProgmanWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetProgmanWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetPropA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetPropA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetPropW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetPropW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "SetRectEmpty" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetRectEmpty arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "SetScrollInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetScrollInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SetScrollPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetScrollPos arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "SetScrollRange" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetScrollRange arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 5);
        }
        "SetShellChangeNotifyWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetShellChangeNotifyWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetShellWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetShellWindow arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetShellWindowEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetShellWindowEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetSysColors" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetSysColors arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetSysColorsTemp" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetSysColorsTemp arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetSystemCursor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetSystemCursor arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetSystemMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetSystemMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetTaskmanWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetTaskmanWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetThreadCursorCreationScaling" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetThreadCursorCreationScaling arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetThreadDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetThreadDesktop arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetThreadDpiAwarenessContext" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetThreadDpiAwarenessContext arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetThreadDpiHostingBehavior" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetThreadDpiHostingBehavior arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetThreadInputBlocked" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetThreadInputBlocked arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetTimer" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetTimer arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("settimer");
            stdcall_ret(emu, 4);
        }
        "SetUserObjectCapability" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetUserObjectCapability arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetUserObjectInformationA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetUserObjectInformationA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUserObjectInformationW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetUserObjectInformationW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetUserObjectSecurity" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetUserObjectSecurity arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWinEventHook" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWinEventHook arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 8);
        }
        "SetWindowBand" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetWindowBand arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowCompositionAttribute" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowCompositionAttribute arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowCompositionTransition" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowCompositionTransition arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowContextHelpId" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowContextHelpId arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowDisplayAffinity" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowDisplayAffinity arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowFeedbackSetting" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowFeedbackSetting arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowMessageCapability" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowMessageCapability arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowPlacement" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowPlacement arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetWindowRgn arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowRgnEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetWindowRgnEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowStationUser" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowStationUser arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowWord" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SetWindowWord arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowsHookA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowsHookA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetWindowsHookExAW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowsHookExAW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetWindowsHookExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowsHookExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("setwindowshookexw");
            stdcall_ret(emu, 4);
        }
        "SetWindowsHookW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowsHookW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "ShellForegroundBoostProcess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellForegroundBoostProcess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShellHandwritingDelegateInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellHandwritingDelegateInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShellHandwritingHandleDelegatedInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellHandwritingHandleDelegatedInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShellHandwritingUndelegateInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellHandwritingUndelegateInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShellMigrateWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellMigrateWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShellRegisterHotKey" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellRegisterHotKey arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("shellregisterhotkey");
            stdcall_ret(emu, 2);
        }
        "ShellSetWindowPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShellSetWindowPos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowCaret" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ShowCaret arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowOwnedPopups" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShowOwnedPopups arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowScrollBar" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ShowScrollBar arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "ShowStartGlass" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ShowStartGlass arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.regs_mut().rax = helper::handler_create("showstartglass");
            stdcall_ret(emu, 2);
        }
        "ShowSystemCursor" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShowSystemCursor arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShowWindowAsync" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShowWindowAsync arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShutdownBlockReasonCreate" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShutdownBlockReasonCreate arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("shutdownblockreasoncreate");
            stdcall_ret(emu, 2);
        }
        "ShutdownBlockReasonDestroy" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!ShutdownBlockReasonDestroy handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ShutdownBlockReasonQuery" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!ShutdownBlockReasonQuery arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SignalRedirectionStartComplete" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SignalRedirectionStartComplete arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("signalredirectionstartcomplete");
            stdcall_ret(emu, 2);
        }
        "SkipPointerFrameMessages" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SkipPointerFrameMessages arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SoftModalMessageBox" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SoftModalMessageBox arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SoundSentry" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SoundSentry arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SubtractRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SubtractRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SuppressWindowDisplayChange" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SuppressWindowDisplayChange arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SwapMouseButton" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SwapMouseButton arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SwitchDesktop" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!SwitchDesktop arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SwitchDesktopWithFade" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SwitchDesktopWithFade arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SwitchToThisWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SwitchToThisWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SystemParametersInfoForDpi" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SystemParametersInfoForDpi arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SystemParametersInfoW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SystemParametersInfoW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "TabbedTextOutA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!TabbedTextOutA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "TabbedTextOutW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!TabbedTextOutW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 9);
        }
        "TileChildWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TileChildWindows arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "TileWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!TileWindows arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ToAscii" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ToAscii arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 5);
        }
        "ToAsciiEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ToAsciiEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "ToUnicode" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ToUnicode arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "ToUnicodeEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ToUnicodeEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 7);
        }
        "TrackMouseEvent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TrackMouseEvent arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "TrackPopupMenu" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!TrackPopupMenu arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 7);
        }
        "TrackPopupMenuEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TrackPopupMenuEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 6);
        }
        "TranslateAccelerator" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TranslateAccelerator arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "TranslateAcceleratorA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TranslateAcceleratorA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "TranslateAcceleratorW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TranslateAcceleratorW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "TranslateMDISysAccel" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TranslateMDISysAccel arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "TranslateMessageEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!TranslateMessageEx arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UndelegateInput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UndelegateInput arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnhookWinEvent" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!UnhookWinEvent arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UnhookWindowsHook" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UnhookWindowsHook arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnhookWindowsHookEx" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnhookWindowsHookEx handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UnionRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!UnionRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "UnloadKeyboardLayout" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnloadKeyboardLayout handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnlockWindowStation" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnlockWindowStation handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UnpackDDElParam" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UnpackDDElParam arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterDeviceNotification" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!UnregisterDeviceNotification handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterHotKey" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnregisterHotKey handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterMessagePumpHook" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!UnregisterMessagePumpHook handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterPointerInputTarget" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!UnregisterPointerInputTarget handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterPointerInputTargetEx" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!UnregisterPointerInputTargetEx handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterPowerSettingNotification" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!UnregisterPowerSettingNotification handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterSessionPort" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnregisterSessionPort handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterSuspendResumeNotification" => {
            let handle = arg(emu, 0) as u64;
            log_red!(
                emu,
                "user32!UnregisterSuspendResumeNotification handle:0x{:x}",
                handle
            );
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterTouchWindow" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnregisterTouchWindow handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterUserApiHook" => {
            let handle = arg(emu, 0) as u64;
            log_red!(emu, "user32!UnregisterUserApiHook handle:0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdateDefaultDesktopThumbnail" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UpdateDefaultDesktopThumbnail arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdateLayeredWindow" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UpdateLayeredWindow arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdateLayeredWindowIndirect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UpdateLayeredWindowIndirect arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdatePerUserSystemParameters" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UpdatePerUserSystemParameters arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UpdateWindowInputSinkHints" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UpdateWindowInputSinkHints arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "User32InitializeImmEntryTable" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!User32InitializeImmEntryTable arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("user32initializeimmentrytable");
            stdcall_ret(emu, 2);
        }
        "UserClientDllInitialize" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UserClientDllInitialize arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("userclientdllinitialize");
            stdcall_ret(emu, 2);
        }
        "UserHandleGrantAccess" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UserHandleGrantAccess arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UserLpkPSMTextOut" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UserLpkPSMTextOut arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UserLpkTabbedTextOut" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UserLpkTabbedTextOut arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UserRealizePalette" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UserRealizePalette arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UserRegisterWowHandlers" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UserRegisterWowHandlers arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("userregisterwowhandlers");
            stdcall_ret(emu, 2);
        }
        "VRipOutput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!VRipOutput arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "VTagOutput" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!VTagOutput arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ValidateRect" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ValidateRect arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "ValidateRgn" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!ValidateRgn arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "VkKeyScanA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!VkKeyScanA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "VkKeyScanExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!VkKeyScanExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "VkKeyScanExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!VkKeyScanExW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "VkKeyScanW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!VkKeyScanW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "WCSToMBEx" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!WCSToMBEx arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WINNLSEnableIME" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WINNLSEnableIME arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("winnlsenableime");
            stdcall_ret(emu, 2);
        }
        "WINNLSGetEnableStatus" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WINNLSGetEnableStatus arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("winnlsgetenablestatus");
            stdcall_ret(emu, 2);
        }
        "WINNLSGetIMEHotkey" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WINNLSGetIMEHotkey arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WaitForInputIdle" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WaitForInputIdle arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WaitForRedirectionStartComplete" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WaitForRedirectionStartComplete arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.regs_mut().rax = helper::handler_create("waitforredirectionstartcomplete");
            stdcall_ret(emu, 2);
        }
        "WaitMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!WaitMessage arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "WinHelpA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!WinHelpA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "WinHelpW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!WinHelpW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "WindowFromDC" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!WindowFromDC arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WindowFromPhysicalPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WindowFromPhysicalPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "WindowFromPoint" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!WindowFromPoint arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "_UserTestTokenForInteractive" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!_UserTestTokenForInteractive arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "gSharedInfo" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!gSharedInfo arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "gapfnScSendMessage" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!gapfnScSendMessage arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "keybd_event" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!keybd_event arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "mouse_event" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!mouse_event arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "wvsprintfA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!wvsprintfA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "wvsprintfW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!wvsprintfW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "Wow64Transition" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!Wow64Transition arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "RegisterClassExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterClassExA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "RegisterClassExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!RegisterClassExW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 1);
        }
        "UnregisterClassA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UnregisterClassA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "UnregisterClassW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!UnregisterClassW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "EnumWindows" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!EnumWindows arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            stdcall_ret(emu, 2);
        }
        "FindWindowExA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!FindWindowExA arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "FindWindowExW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(emu, "user32!FindWindowExW arg0:0x{:x} arg1:0x{:x}", a0, a1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 4);
        }
        "GetWindowLongPtrA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowLongPtrA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "GetWindowLongPtrW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetWindowLongPtrW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }
        "SetWindowLongPtrA" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowLongPtrA arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "SetWindowLongPtrW" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!SetWindowLongPtrW arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 3);
        }
        "GetPhysicalCursorPos" => {
            let a0 = arg(emu, 0) as u64;
            let a1 = arg(emu, 1) as u64;
            log_red!(
                emu,
                "user32!GetPhysicalCursorPos arg0:0x{:x} arg1:0x{:x}",
                a0,
                a1
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            stdcall_ret(emu, 2);
        }

        _ => {
            return fallback::unimplemented(addr, api, emu);
        }
    }

    String::new()
}

fn MessageBoxA(emu: &mut emu::Emu) {
    let titleptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let msgptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let msg = emu.maps.read_string(msgptr);
    let title = emu.maps.read_string(titleptr);

    log_red!(emu, "user32!MessageBoxA {} {}", title, msg);

    emu.regs_mut().rax = 0;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

fn MessageBoxW(emu: &mut emu::Emu) {
    let titleptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let msgptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let msg = emu.maps.read_wide_string(msgptr);
    let title = emu.maps.read_wide_string(titleptr);

    log_red!(emu, "user32!MessageBoxW {} {}", title, msg);

    emu.regs_mut().rax = 0;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}

fn GetDesktopWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetDesktopWindow");
    //emu.regs_mut().rax = 0x11223344; // current window handle
    emu.regs_mut().rax = 0; // no windows handler is more stealthy
}

fn wsprintfW(emu: &mut emu::Emu) {}

fn GetProcessWindowStation(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetProcessWindowStation");

    emu.regs_mut().rax = 0x1337; // get handler
}

fn GetUserObjectInformationW(emu: &mut emu::Emu) {
    let hndl = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let nidx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let out_vinfo = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let nlen = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let out_len = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    log_red!(emu, "user32!GetUserObjectInformationW");

    for _ in 0..5 {
        emu.stack_pop32(false);
    }

    emu.regs_mut().rax = 1; // get handler
}

fn CharLowerW(emu: &mut emu::Emu) {
    let ptr_str = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;

    let s = emu.maps.read_wide_string(ptr_str);

    log_red!(emu, "user32!CharLowerW(`{}`)", s);

    emu.maps.write_wide_string(ptr_str, &s.to_lowercase());

    emu.stack_pop32(false);
    emu.regs_mut().rax = ptr_str;
}

fn wsprintfA(emu: &mut emu::Emu) {
    let out = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let in_fmt = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let mut multiple = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;

    let fmt = emu.maps.read_string(in_fmt);
    let mut args = Vec::new();
    let mut chars = fmt.chars().peekable();
    let mut arg_index = 0;
    let mut result = String::new();

    while let Some(arg) = emu.maps.read_dword(multiple) {
        args.push(arg as u64);
        multiple += 4;
    }

    while let Some(c) = chars.next() {
        if c == '%' {
            if chars.peek() == Some(&'%') {
                result.push('%');
                chars.next();
            } else if arg_index < args.len() {
                let specifier = chars.next();
                match specifier {
                    Some('d') => result.push_str(&format!("{}", args[arg_index] as i32)),
                    Some('u') => result.push_str(&format!("{}", args[arg_index])),
                    Some('x') => result.push_str(&format!("{:x}", args[arg_index])),
                    Some('X') => result.push_str(&format!("{:X}", args[arg_index])),
                    Some('s') => {
                        let addr = args[arg_index];
                        let s = emu.maps.read_string(addr);
                        if !s.is_empty() {
                            result.push_str(&s);
                        } else {
                            result.push_str("<invalid string>");
                        }
                    }
                    Some('c') => result.push(args[arg_index] as u8 as char),
                    _ => result.push_str("<unsupported format>"),
                }
                arg_index += 1;
            } else {
                result.push_str("<missing>");
            }
        } else {
            result.push(c);
        }
    }

    emu.maps.write_string(out, &result);

    log_red!(emu, "user32!wsprintfA fmt:`{}` out:`{}`", fmt, &result);

    emu.stack_pop32(false);
    emu.stack_pop32(false);
    emu.stack_pop32(false);

    emu.regs_mut().rax = result.len() as u64;
}

fn GetSystemMetrics(emu: &mut emu::Emu) {
    let index = arg(emu, 0) as i32;
    let result: u32 = match index {
        0 => 1920,
        1 => 1080,
        2 => 20,
        3 => 20,
        4 => 23,
        5 => 1,
        6 => 1,
        7 => 4,
        8 => 4,
        9 => 20,
        10 => 20,
        11 => 32,
        12 => 32,
        13 => 32,
        14 => 32,
        15 => 20,
        16 => 1920,
        17 => 1017,
        20 => 17,
        21 => 17,
        28 => 20,
        29 => 20,
        30 => 22,
        31 => 22,
        32 => 8,
        33 => 8,
        34 => 112,
        35 => 27,
        36 => 4,
        37 => 4,
        38 => 75,
        39 => 75,
        43 => 1,
        48 => 16,
        49 => 16,
        50 => 16,
        51 => 16,
        52 => 16,
        53 => 18,
        54 => 18,
        61 => 1920,
        62 => 1080,
        63 => 0,
        64 => 0,
        76 => 1920,
        77 => 1040,
        78 => 1936,
        79 => 1096,
        80 => 1,
        _ => 0,
    };
    log_red!(
        emu,
        "user32!GetSystemMetrics nIndex: {} = {}",
        index,
        result
    );
    emu.regs_mut().rax = result as u64;
    stdcall_ret(emu, 1);
}
fn SystemParametersInfoA(emu: &mut emu::Emu) {
    let action = arg(emu, 0);
    log_red!(emu, "user32!SystemParametersInfoA uiAction: 0x{:x}", action);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
fn LoadIconA(emu: &mut emu::Emu) {
    let hinstance = arg(emu, 0);
    let name = arg(emu, 1);
    log_red!(
        emu,
        "user32!LoadIconA hInstance: 0x{:x} name: 0x{:x}",
        hinstance,
        name
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
fn LoadCursorA(emu: &mut emu::Emu) {
    let hinstance = arg(emu, 0);
    let name = arg(emu, 1);
    log_red!(
        emu,
        "user32!LoadCursorA hInstance: 0x{:x} name: 0x{:x}",
        hinstance,
        name
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}
fn RegisterClassA(emu: &mut emu::Emu) {
    let wndclass = arg(emu, 0);
    log_red!(emu, "user32!RegisterClassA 0x{:x}", wndclass);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
fn RegisterClassW(emu: &mut emu::Emu) {
    let wndclass = arg(emu, 0);
    log_red!(emu, "user32!RegisterClassW 0x{:x}", wndclass);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
fn GetDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "user32!GetDC hwnd: 0x{:x}", hwnd);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}
fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let hdc = arg(emu, 1);
    log_red!(emu, "user32!ReleaseDC hwnd: 0x{:x} hdc: 0x{:x}", hwnd, hdc);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
fn CharLowerBuffA(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!CharLowerBuffA buf=0x{:x} len={}", buf, len);
    for i in 0..len {
        if let Some(ch) = emu.maps.read_byte(buf + i) {
            emu.maps.write_byte(buf + i, ch.to_ascii_lowercase());
        }
    }
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
fn CharUpperBuffA(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!CharUpperBuffA buf=0x{:x} len={}", buf, len);
    for i in 0..len {
        if let Some(ch) = emu.maps.read_byte(buf + i) {
            emu.maps.write_byte(buf + i, ch.to_ascii_uppercase());
        }
    }
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
fn CharLowerBuffW(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!CharLowerBuffW buf=0x{:x} len={}", buf, len);
    for i in 0..len {
        if let Some(ch) = emu.maps.read_word(buf + i * 2)
            && let Some(c) = char::from_u32(ch as u32)
        {
            let out = c.to_lowercase().next().unwrap_or(c) as u32;
            emu.maps.write_word(buf + i * 2, out as u16);
        }
    }
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
fn CharUpperBuffW(emu: &mut emu::Emu) {
    let buf = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64;
    log_red!(emu, "user32!CharUpperBuffW buf=0x{:x} len={}", buf, len);
    for i in 0..len {
        if let Some(ch) = emu.maps.read_word(buf + i * 2)
            && let Some(c) = char::from_u32(ch as u32)
        {
            let out = c.to_uppercase().next().unwrap_or(c) as u32;
            emu.maps.write_word(buf + i * 2, out as u16);
        }
    }
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 2);
}
fn FindWindowA(emu: &mut emu::Emu) {
    let class_ptr = arg(emu, 0) as u64;
    let window_ptr = arg(emu, 1) as u64;
    let class_name = if class_ptr != 0 {
        emu.maps.read_string(class_ptr)
    } else {
        String::new()
    };
    let window_name = if window_ptr != 0 {
        emu.maps.read_string(window_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!FindWindowA class='{}' window='{}'",
        class_name,
        window_name
    );
    emu.regs_mut().rax = 0; // not found
    stdcall_ret(emu, 2);
}
fn FindWindowW(emu: &mut emu::Emu) {
    let class_ptr = arg(emu, 0) as u64;
    let window_ptr = arg(emu, 1) as u64;
    let class_name = if class_ptr != 0 {
        emu.maps.read_wide_string(class_ptr)
    } else {
        String::new()
    };
    let window_name = if window_ptr != 0 {
        emu.maps.read_wide_string(window_ptr)
    } else {
        String::new()
    };
    log_red!(
        emu,
        "user32!FindWindowW class='{}' window='{}'",
        class_name,
        window_name
    );
    emu.regs_mut().rax = 0; // not found
    stdcall_ret(emu, 2);
}
fn GetKeyState(emu: &mut emu::Emu) {
    let vkey = arg(emu, 0);
    log_red!(emu, "user32!GetKeyState vKey=0x{:x}", vkey);
    emu.regs_mut().rax = 0; // not pressed
    stdcall_ret(emu, 1);
}
fn GetAsyncKeyState(emu: &mut emu::Emu) {
    let vkey = arg(emu, 0);
    log_red!(emu, "user32!GetAsyncKeyState vKey=0x{:x}", vkey);
    emu.regs_mut().rax = 0; // not pressed
    stdcall_ret(emu, 1);
}
fn SetWindowsHookExA(emu: &mut emu::Emu) {
    let id_hook = arg(emu, 0) as i32;
    let lpfn = arg(emu, 1);
    let hmod = arg(emu, 2);
    let thread_id = arg(emu, 3);
    log_red!(
        emu,
        "user32!SetWindowsHookExA idHook={} lpfn=0x{:x} hMod=0x{:x} threadId={}",
        id_hook,
        lpfn,
        hmod,
        thread_id
    );
    emu.regs_mut().rax = helper::handler_create("hook://");
    stdcall_ret(emu, 4);
}

fn AdjustWindowRect(emu: &mut emu::Emu) {
    log_red!(emu, "user32!AdjustWindowRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn AdjustWindowRectEx(emu: &mut emu::Emu) {
    log_red!(emu, "user32!AdjustWindowRectEx");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn CreateWindowExA(emu: &mut emu::Emu) {
    let class = emu.maps.read_string(arg(emu, 1) as u64);
    let title = emu.maps.read_string(arg(emu, 2) as u64);
    log_red!(
        emu,
        "user32!CreateWindowExA class:'{}' title:'{}'",
        class,
        title
    );
    emu.regs_mut().rax = 0x1337_0001;
    stdcall_ret(emu, 12);
}

fn CreateWindowExW(emu: &mut emu::Emu) {
    let class = emu.maps.read_wide_string(arg(emu, 1) as u64);
    let title = emu.maps.read_wide_string(arg(emu, 2) as u64);
    log_red!(
        emu,
        "user32!CreateWindowExW class:'{}' title:'{}'",
        class,
        title
    );
    emu.regs_mut().rax = 0x1337_0001;
    stdcall_ret(emu, 12);
}

fn ShowWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!ShowWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn UpdateWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!UpdateWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn DestroyWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!DestroyWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn PostMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageA hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn PostMessageW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageW hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn SendMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SendMessageA hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn SendMessageW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SendMessageW hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn DefWindowProcA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!DefWindowProcA hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn DefWindowProcW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!DefWindowProcW hwnd:0x{:x} msg:0x{:x}",
        arg(emu, 0),
        arg(emu, 1)
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn GetWindowRect(emu: &mut emu::Emu) {
    let rect = arg(emu, 1) as u64;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "user32!GetWindowRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn GetClientRect(emu: &mut emu::Emu) {
    let rect = arg(emu, 1) as u64;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1904);
        emu.maps.write_dword(rect + 12, 1041);
    }
    log_red!(emu, "user32!GetClientRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn SetWindowPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowPos");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}

fn SetWindowTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(arg(emu, 1) as u64);
    log_red!(emu, "user32!SetWindowTextA '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn SetWindowTextW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(arg(emu, 1) as u64);
    log_red!(emu, "user32!SetWindowTextW '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn GetWindowTextA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowTextA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn GetWindowTextW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowTextW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn GetWindowLongA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowLongA idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

fn GetWindowLongW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowLongW idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

fn SetWindowLongA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowLongA idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn SetWindowLongW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowLongW idx:{}", arg(emu, 1) as i32);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn PeekMessageA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!PeekMessageA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

fn PeekMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!PeekMessageW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 5);
}

fn GetMessageA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetMessageA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn GetMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetMessageW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn TranslateMessage(emu: &mut emu::Emu) {
    log_red!(emu, "user32!TranslateMessage");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn DispatchMessageA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!DispatchMessageA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn DispatchMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!DispatchMessageW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn PostQuitMessage(emu: &mut emu::Emu) {
    let exit_code = arg(emu, 0) as i32;
    log_red!(emu, "user32!PostQuitMessage code:{}", exit_code);
    stdcall_ret(emu, 1);
}

fn IsWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn IsWindowVisible(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsWindowVisible hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn EnableWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EnableWindow hwnd:0x{:x}", arg(emu, 0));
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

fn SetFocus(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "user32!SetFocus hwnd:0x{:x}", hwnd);
    emu.regs_mut().rax = hwnd as u64;
    stdcall_ret(emu, 1);
}

fn GetFocus(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetFocus");
    emu.regs_mut().rax = 0x1337_0001;
}

fn GetForegroundWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetForegroundWindow");
    emu.regs_mut().rax = 0x1337_0001;
}

fn SetForegroundWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetForegroundWindow");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn GetCursorPos(emu: &mut emu::Emu) {
    let pt = arg(emu, 0) as u64;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    log_red!(emu, "user32!GetCursorPos");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn SetCursorPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetCursorPos");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn SetCursor(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetCursor");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn ShowCursor(emu: &mut emu::Emu) {
    log_red!(emu, "user32!ShowCursor");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn LoadStringA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!LoadStringA id:{}", arg(emu, 1));
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn LoadStringW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!LoadStringW id:{}", arg(emu, 1));
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}

fn GetClassInfoExA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetClassInfoExA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn GetClassInfoExW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetClassInfoExW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn MapVirtualKeyA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!MapVirtualKeyA");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

fn MapVirtualKeyW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!MapVirtualKeyW");
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 2);
}

fn GetKeyboardLayout(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetKeyboardLayout");
    emu.regs_mut().rax = 0x0409_0409;
    stdcall_ret(emu, 1);
}

fn GetKeyboardType(emu: &mut emu::Emu) {
    let ntype = arg(emu, 0) as i32;
    let result: u32 = match ntype {
        0 => 4,
        1 => 0,
        2 => 12,
        _ => 0,
    };
    log_red!(emu, "user32!GetKeyboardType type:{} = {}", ntype, result);
    emu.regs_mut().rax = result as u64;
    stdcall_ret(emu, 1);
}

fn BeginPaint(emu: &mut emu::Emu) {
    log_red!(emu, "user32!BeginPaint");
    emu.regs_mut().rax = 0x1337_0002;
    stdcall_ret(emu, 2);
}

fn EndPaint(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EndPaint");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn InvalidateRect(emu: &mut emu::Emu) {
    log_red!(emu, "user32!InvalidateRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn FillRect(emu: &mut emu::Emu) {
    log_red!(emu, "user32!FillRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn DrawTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(arg(emu, 1) as u64);
    log_red!(emu, "user32!DrawTextA '{}'", text);
    emu.regs_mut().rax = 20;
    stdcall_ret(emu, 5);
}

fn DrawTextW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(arg(emu, 1) as u64);
    log_red!(emu, "user32!DrawTextW '{}'", text);
    emu.regs_mut().rax = 20;
    stdcall_ret(emu, 5);
}
