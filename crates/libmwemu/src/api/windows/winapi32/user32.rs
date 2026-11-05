use crate::emu;
use crate::serialization;
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
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ActivateKeyboardLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddClipboardFormatListener" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AddClipboardFormatListener arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddVisualIdentifier" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AddVisualIdentifier arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AdjustWindowRectExForDpi" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AdjustWindowRectExForDpi arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AlignRects" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!AlignRects arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AllowForegroundActivation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AllowForegroundActivation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AllowSetForegroundWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AllowSetForegroundWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AnimateWindow" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!AnimateWindow p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AnyPopup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!AnyPopup arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppendMenuA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!AppendMenuA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "AppendMenuW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!AppendMenuW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ApplyWindowAction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ApplyWindowAction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AreDpiAwarenessContextsEqual" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AreDpiAwarenessContextsEqual arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ArrangeIconicWindows" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ArrangeIconicWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AttachThreadInput" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!AttachThreadInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BeginDeferWindowPos" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!BeginDeferWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BlockInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!BlockInput 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "BringWindowToTop" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!BringWindowToTop");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!BroadcastSystemMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!BroadcastSystemMessageA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!BroadcastSystemMessageExA '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!BroadcastSystemMessageExW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!BroadcastSystemMessageW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "BuildReasonArray" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!BuildReasonArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CalcMenuBar" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CalcMenuBar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CalculatePopupWindowPosition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CalculatePopupWindowPosition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallMsgFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CallMsgFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallMsgFilterA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CallMsgFilterA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CallMsgFilterW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CallMsgFilterW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CallNextHookEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CallNextHookEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallWindowProcA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CallWindowProcA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CallWindowProcW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CallWindowProcW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "CancelShutdown" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CancelShutdown h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CascadeChildWindows" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CascadeChildWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CascadeWindows" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CascadeWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeClipboardChain" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ChangeClipboardChain arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeDisplaySettingsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeDisplaySettingsA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "ChangeDisplaySettingsExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!ChangeDisplaySettingsExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "ChangeDisplaySettingsExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!ChangeDisplaySettingsExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "ChangeDisplaySettingsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeDisplaySettingsW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "ChangeMenuA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeMenuA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ChangeMenuW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeMenuW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ChangeWindowMessageFilter" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ChangeWindowMessageFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeWindowMessageFilterEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ChangeWindowMessageFilterEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CharLowerA" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if ptr < 0x10000 {
                let ch = (ptr & 0xFF) as u8;
                let lower = (ch as char).to_ascii_lowercase() as u64;
                log_red!(emu, "user32!CharLowerA char: '{}'", ch as char);
                for _ in 0..1u32 {
                    emu.stack_pop32(false);
                }
                emu.regs_mut().rax = lower;
            } else {
                let s = emu.maps.read_string(ptr);
                log_red!(emu, "user32!CharLowerA str: '{}'", s);
                let lower = s.to_ascii_lowercase();
                emu.maps.write_string(ptr, &lower);
                for _ in 0..1u32 {
                    emu.stack_pop32(false);
                }
                emu.regs_mut().rax = ptr;
            }
        }
        "CharNextA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharNextA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CharNextExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharNextExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CharNextW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharNextW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CharPrevA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharPrevA '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CharPrevExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharPrevExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CharPrevW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharPrevW '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CharToOemA" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!CharToOemA src: 0x{:x} dst: 0x{:x}", src, dst);
            if src != 0 && dst != 0 {
                let s = emu.maps.read_string(src);
                emu.maps.write_string(dst, &s);
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CharToOemBuffA" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "user32!CharToOemBuffA src: 0x{:x} dst: 0x{:x} len: {}",
                src,
                dst,
                len
            );
            if src != 0 && dst != 0 {
                for i in 0..len as u64 {
                    let b = emu.maps.read_byte(src + i).unwrap_or(0);
                    emu.maps.write_byte(dst + i, b);
                }
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CharToOemBuffW" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let len = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "user32!CharToOemBuffW src: 0x{:x} dst: 0x{:x} len: {}",
                src,
                dst,
                len
            );
            if src != 0 && dst != 0 {
                for i in 0..len as u64 {
                    let ch = emu.maps.read_word(src + i * 2).unwrap_or(0);
                    emu.maps.write_byte(dst + i, (ch & 0xFF) as u8);
                }
            }
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CharToOemW" => {
            let src = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dst = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!CharToOemW src: 0x{:x} dst: 0x{:x}", src, dst);
            if src != 0 && dst != 0 {
                let s = emu.maps.read_wide_string(src);
                emu.maps.write_string(dst, &s);
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CharUpperA" => {
            let ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if ptr < 0x10000 {
                let ch = (ptr & 0xFF) as u8;
                let upper = (ch as char).to_ascii_uppercase() as u64;
                log_red!(emu, "user32!CharUpperA char: '{}'", ch as char);
                for _ in 0..1u32 {
                    emu.stack_pop32(false);
                }
                emu.regs_mut().rax = upper;
            } else {
                let s = emu.maps.read_string(ptr);
                log_red!(emu, "user32!CharUpperA str: '{}'", s);
                let upper = s.to_ascii_uppercase();
                emu.maps.write_string(ptr, &upper);
                for _ in 0..1u32 {
                    emu.stack_pop32(false);
                }
                emu.regs_mut().rax = ptr;
            }
        }
        "CharUpperW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharUpperW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "CheckBannedOneCoreTransformApi" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckBannedOneCoreTransformApi arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckDBCSEnabledExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckDBCSEnabledExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckDlgButton" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!CheckDlgButton p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckMenuItem" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!CheckMenuItem p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckMenuRadioItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckMenuRadioItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckProcessForClipboardAccess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckProcessForClipboardAccess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckProcessSession" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckProcessSession h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckRadioButton" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckRadioButton p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckWindowThreadDesktop" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CheckWindowThreadDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChildWindowFromPoint" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ChildWindowFromPoint p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChildWindowFromPointEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ChildWindowFromPointEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CliImmSetHotKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CliImmSetHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClientThreadSetup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ClientThreadSetup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClientToScreen" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pt = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "user32!ClientToScreen hwnd:0x{:x} pt:0x{:x}", hwnd, pt);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClipCursor" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!ClipCursor");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseClipboard" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseClipboard handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseDesktop" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseDesktop handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseGestureInfoHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseGestureInfoHandle handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseTouchInputHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseTouchInputHandle handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseWindow handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CloseWindowStation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseWindowStation handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ConsoleControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ConsoleControl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ControlMagnification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ControlMagnification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertPrimaryPointerToMouseDrag" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ConvertPrimaryPointerToMouseDrag arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertToInterceptWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ConvertToInterceptWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyAcceleratorTableA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CopyAcceleratorTableA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CopyAcceleratorTableW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CopyAcceleratorTableW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "CopyIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!CopyIcon arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyImage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!CopyImage arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyRect" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if dst != 0 && src != 0 {
                let l = emu.maps.read_dword(src).unwrap_or(0);
                let t = emu.maps.read_dword(src + 4).unwrap_or(0);
                let r = emu.maps.read_dword(src + 8).unwrap_or(0);
                let b = emu.maps.read_dword(src + 12).unwrap_or(0);
                emu.maps.write_dword(dst, l);
                emu.maps.write_dword(dst + 4, t);
                emu.maps.write_dword(dst + 8, r);
                emu.maps.write_dword(dst + 12, b);
            }
            log_red!(emu, "user32!CopyRect dst:0x{:x} src:0x{:x}", dst, src);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CountClipboardFormats" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!CountClipboardFormats arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateAcceleratorTableA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateAcceleratorTableA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://CreateAcceleratorTableA");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateAcceleratorTableW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateAcceleratorTableW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://CreateAcceleratorTableW");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateCaret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateCaret");
            log_red!(emu, "user32!CreateCaret => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateCursor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let arg6 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateCursor arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateDCompositionHwndTarget" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateDCompositionHwndTarget");
            log_red!(emu, "user32!CreateDCompositionHwndTarget => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateDesktopA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CreateDesktopA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateDesktopExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CreateDesktopExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateDesktopExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CreateDesktopExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateDesktopW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CreateDesktopW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateDialogIndirectParamA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            );
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateDialogIndirectParamAorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamAorW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CreateDialogIndirectParamW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            );
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateDialogParamA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateDialogParamA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            );
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateDialogParamW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateDialogParamW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5
            );
            for _ in 0..5u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let arg6 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateIcon arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateIconFromResource" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateIconFromResource arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4
            );
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateIconFromResourceEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let arg6 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateIconFromResourceEx arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            for _ in 0..7u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateIconIndirect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!CreateIconIndirect arg1:0x{:x}", arg1);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://CreateIconIndirect");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "CreateMDIWindowA" => {
            let ex_style = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let title_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let style = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let class_name = if class_ptr > 0xffff {
                emu.maps.read_string(class_ptr)
            } else {
                format!("atom_{}", class_ptr)
            };
            let title = if title_ptr != 0 {
                emu.maps.read_string(title_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("window_{}", class_name));
            log_red!(
                emu,
                "user32!CreateMDIWindowA class:'{}' title:'{}' => 0x{:x}",
                class_name,
                title,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..12 {
                emu.stack_pop32(false);
            }
        }
        "CreateMDIWindowW" => {
            let ex_style = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let title_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let style = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let class_name = if class_ptr > 0xffff {
                emu.maps.read_wide_string(class_ptr)
            } else {
                format!("atom_{}", class_ptr)
            };
            let title = if title_ptr != 0 {
                emu.maps.read_wide_string(title_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("window_{}", class_name));
            log_red!(
                emu,
                "user32!CreateMDIWindowW class:'{}' title:'{}' => 0x{:x}",
                class_name,
                title,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..12 {
                emu.stack_pop32(false);
            }
        }
        "CreateMenu" => {
            log_red!(emu, "user32!CreateMenu");
            emu.regs_mut().rax = helper::handler_create("CreateMenu");
        }
        "CreatePopupMenu" => {
            log_red!(emu, "user32!CreatePopupMenu");
            emu.regs_mut().rax = helper::handler_create("CreatePopupMenu");
        }
        "CreateSyntheticPointerDevice" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateSyntheticPointerDevice");
            log_red!(emu, "user32!CreateSyntheticPointerDevice => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateSyntheticPointerDevice2" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateSyntheticPointerDevice2");
            log_red!(emu, "user32!CreateSyntheticPointerDevice2 => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateSystemThreads" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateSystemThreads");
            log_red!(emu, "user32!CreateSystemThreads => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateWindowInBand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateWindowInBand");
            log_red!(emu, "user32!CreateWindowInBand => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateWindowInBandEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateWindowInBandEx");
            log_red!(emu, "user32!CreateWindowInBandEx => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateWindowIndirect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("CreateWindowIndirect");
            log_red!(emu, "user32!CreateWindowIndirect => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "CreateWindowStationA" => {
            let ex_style = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let title_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let style = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let class_name = if class_ptr > 0xffff {
                emu.maps.read_string(class_ptr)
            } else {
                format!("atom_{}", class_ptr)
            };
            let title = if title_ptr != 0 {
                emu.maps.read_string(title_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("window_{}", class_name));
            log_red!(
                emu,
                "user32!CreateWindowStationA class:'{}' title:'{}' => 0x{:x}",
                class_name,
                title,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..12 {
                emu.stack_pop32(false);
            }
        }
        "CreateWindowStationW" => {
            let ex_style = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let title_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let style = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let class_name = if class_ptr > 0xffff {
                emu.maps.read_wide_string(class_ptr)
            } else {
                format!("atom_{}", class_ptr)
            };
            let title = if title_ptr != 0 {
                emu.maps.read_wide_string(title_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("window_{}", class_name));
            log_red!(
                emu,
                "user32!CreateWindowStationW class:'{}' title:'{}' => 0x{:x}",
                class_name,
                title,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..12 {
                emu.stack_pop32(false);
            }
        }
        "CsrBroadcastSystemMessageExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!CsrBroadcastSystemMessageExW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "CtxInitUser32" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!CtxInitUser32 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeAbandonTransaction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeAbandonTransaction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeAccessData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeAccessData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeAddData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DdeAddData arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeClientTransaction" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeClientTransaction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCmpStringHandles" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeCmpStringHandles h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeConnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DdeConnect arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeConnectList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeConnectList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCreateDataHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeCreateDataHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCreateStringHandleA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeCreateStringHandleA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DdeCreateStringHandleW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeCreateStringHandleW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DdeDisconnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeDisconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeDisconnectList" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeDisconnectList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeEnableCallback" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeEnableCallback arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeFreeDataHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeFreeDataHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeFreeStringHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeFreeStringHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeGetData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DdeGetData arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeGetLastError" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeGetLastError arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeGetQualityOfService" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeGetQualityOfService arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeImpersonateClient" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeImpersonateClient arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeInitializeA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeInitializeA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DdeInitializeW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeInitializeW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DdeKeepStringHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeKeepStringHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeNameService" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeNameService arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdePostAdvise" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdePostAdvise arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryConvInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeQueryConvInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryNextServer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeQueryNextServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryStringA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeQueryStringA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DdeQueryStringW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeQueryStringW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DdeReconnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeReconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeSetQualityOfService" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeSetQualityOfService arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeSetUserHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeSetUserHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeUnaccessData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeUnaccessData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeUninitialize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DdeUninitialize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefDlgProcA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefDlgProcA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DefDlgProcW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefDlgProcW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DefFrameProcA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefFrameProcA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DefFrameProcW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefFrameProcW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DefMDIChildProcA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefMDIChildProcA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DefMDIChildProcW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefMDIChildProcW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DefRawInputProc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DefRawInputProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeferWindowPos" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DeferWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeferWindowPosAndBand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DeferWindowPosAndBand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DelegateInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteMenu" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DeleteMenu handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DeregisterShellHookWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DeregisterShellHookWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DestroyAcceleratorTable" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyAcceleratorTable handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyCaret" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyCaret handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyCursor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyCursor handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyDCompositionHwndTarget" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "user32!DestroyDCompositionHwndTarget handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyIcon" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyIcon handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyMenu" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyMenu handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroyReasons" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyReasons handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DestroySyntheticPointerDevice" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(
                emu,
                "user32!DestroySyntheticPointerDevice handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DialogBoxIndirectParamA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DialogBoxIndirectParamA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "DialogBoxIndirectParamAorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DialogBoxIndirectParamAorW '{}'  p1:0x{:x}",
                s,
                p1
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DialogBoxIndirectParamW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DialogBoxIndirectParamW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
        }
        "DialogBoxParamA" => {
            let hinst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let template = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hwnd_parent = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let dlg_proc = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let init_param = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if template != 0 && template > 0xffff {
                emu.maps.read_string(template)
            } else {
                format!("0x{:x}", template)
            };
            log_red!(
                emu,
                "user32!DialogBoxParamA inst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
                hinst,
                s,
                hwnd_parent,
                dlg_proc
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // IDOK
        }
        "DialogBoxParamW" => {
            let hinst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let template = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hwnd_parent = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let dlg_proc = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let init_param = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if template != 0 && template > 0xffff {
                emu.maps.read_wide_string(template)
            } else {
                format!("0x{:x}", template)
            };
            log_red!(
                emu,
                "user32!DialogBoxParamW inst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
                hinst,
                s,
                hwnd_parent,
                dlg_proc
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1; // IDOK
        }
        "DisableProcessWindowsGhosting" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DisableProcessWindowsGhosting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisplayConfigGetDeviceInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DisplayConfigGetDeviceInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisplayConfigSetDeviceInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DisplayConfigSetDeviceInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisplayExitWindowsWarnings" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DisplayExitWindowsWarnings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirListA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirListComboBoxA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListComboBoxA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirListComboBoxW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListComboBoxW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirListW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectComboBoxExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectComboBoxExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectComboBoxExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectComboBoxExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DoSoundConnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DoSoundConnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DoSoundDisconnect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DoSoundDisconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DragDetect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DragDetect arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DragObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DragObject h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawAnimatedRects" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawAnimatedRects arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawCaption" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawCaption arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawCaptionTempA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DrawCaptionTempA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DrawCaptionTempW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DrawCaptionTempW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DrawEdge" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawEdge p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawFocusRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DrawFocusRect p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawFrame" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!DrawFrame arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawFrameControl" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawFrameControl p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawIcon" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawIcon p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawIconEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawIconEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawMenuBar" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!DrawMenuBar");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawMenuBarTemp" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DrawMenuBarTemp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawStateA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DrawStateA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..10 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DrawStateW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DrawStateW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..10 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "DrawTextExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DrawTextExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "DrawTextExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!DrawTextExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
        }
        "DwmGetDxRgn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmGetDxRgn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmGetDxSharedSurface" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmGetDxSharedSurface arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmGetRemoteSessionOcclusionEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!DwmGetRemoteSessionOcclusionEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DwmGetRemoteSessionOcclusionState" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmGetRemoteSessionOcclusionState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmKernelShutdown" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmKernelShutdown arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmKernelStartup" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmKernelStartup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmLockScreenUpdates" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!DwmLockScreenUpdates 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "DwmValidateWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmValidateWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmWindowNotificationsEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!DwmWindowNotificationsEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EditWndProc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EditWndProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EmptyClipboard" => {
            log_red!(emu, "user32!EmptyClipboard");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableMenuItem" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!EnableMenuItem p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableMouseInPointer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnableMouseInPointer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableNonClientDpiScaling" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnableNonClientDpiScaling arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableOneCoreTransformMode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnableOneCoreTransformMode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableScrollBar" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!EnableScrollBar p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableSessionForMMCSS" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnableSessionForMMCSS arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableWindowShellWindowManagementBehavior" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnableWindowShellWindowManagementBehavior arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndDeferWindowPos" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EndDeferWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndDeferWindowPosEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EndDeferWindowPosEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndDialog" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!EndDialog p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!EndMenu arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndTask" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!EndTask arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnterMoveSizeLoop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnterMoveSizeLoop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnterReaderModeHelper" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnterReaderModeHelper arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumChildWindows" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!EnumChildWindows p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumClipboardFormats" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!EnumClipboardFormats");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDesktopWindows" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnumDesktopWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDesktopsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDesktopsA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDesktopsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDesktopsW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDisplayDevicesA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDisplayDevicesA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDisplayDevicesW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDisplayDevicesW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDisplayMonitors" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnumDisplayMonitors p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDisplaySettingsA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDisplaySettingsExA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDisplaySettingsExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumDisplaySettingsW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumPropsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumPropsA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumPropsExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumPropsExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumPropsExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumPropsExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumPropsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumPropsW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumThreadWindows" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!EnumThreadWindows p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumWindowStationsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumWindowStationsA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumWindowStationsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!EnumWindowStationsW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "EnumWindows" => {
            let callback = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let lparam = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EnumWindows callback:0x{:x} lparam:0x{:x}",
                callback,
                lparam
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EqualRect" => {
            let rect1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let rect2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let eq = if rect1 != 0 && rect2 != 0 {
                let l1 = emu.maps.read_dword(rect1).unwrap_or(0);
                let l2 = emu.maps.read_dword(rect2).unwrap_or(0);
                let t1 = emu.maps.read_dword(rect1 + 4).unwrap_or(0);
                let t2 = emu.maps.read_dword(rect2 + 4).unwrap_or(0);
                let r1 = emu.maps.read_dword(rect1 + 8).unwrap_or(0);
                let r2 = emu.maps.read_dword(rect2 + 8).unwrap_or(0);
                let b1 = emu.maps.read_dword(rect1 + 12).unwrap_or(0);
                let b2 = emu.maps.read_dword(rect2 + 12).unwrap_or(0);
                l1 == l2 && t1 == t2 && r1 == r2 && b1 == b2
            } else {
                false
            };
            log_red!(
                emu,
                "user32!EqualRect 0x{:x} 0x{:x} => {}",
                rect1,
                rect2,
                eq
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if eq { 1 } else { 0 };
        }
        "EvaluateProximityToPolygon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EvaluateProximityToPolygon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EvaluateProximityToRect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!EvaluateProximityToRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExcludeUpdateRgn" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!ExcludeUpdateRgn p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExitWindowsEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ExitWindowsEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FindWindowExA" => {
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let title_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let class_name = if class_ptr != 0 && class_ptr > 0xffff {
                emu.maps.read_string(class_ptr)
            } else {
                String::new()
            };
            let title = if title_ptr != 0 {
                emu.maps.read_string(title_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("window_{}", class_name));
            log_red!(
                emu,
                "user32!FindWindowExA class:'{}' title:'{}' => 0x{:x}",
                class_name,
                title,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "FindWindowExW" => {
            let class_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let title_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let class_name = if class_ptr != 0 && class_ptr > 0xffff {
                emu.maps.read_wide_string(class_ptr)
            } else {
                String::new()
            };
            let title = if title_ptr != 0 {
                emu.maps.read_wide_string(title_ptr)
            } else {
                String::new()
            };
            let hndl = helper::handler_create(&format!("window_{}", class_name));
            log_red!(
                emu,
                "user32!FindWindowExW class:'{}' title:'{}' => 0x{:x}",
                class_name,
                title,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "FlashWindow" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!FlashWindow p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlashWindowEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!FlashWindowEx");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FrameRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!FrameRect p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeDDElParam" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!FreeDDElParam handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "FrostCrashedWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!FrostCrashedWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetActiveWindow" => {
            let hwnd = helper::handler_create("hwnd://active");
            log_red!(emu, "user32!GetActiveWindow => 0x{:x}", hwnd);
            emu.set_last_error(0);
            emu.regs_mut().rax = hwnd;
        }
        "GetAltTabInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetAltTabInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAltTabInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetAltTabInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetAltTabInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetAltTabInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetAncestor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetAncestor h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAppCompatFlags" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetAppCompatFlags h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAppCompatFlags2" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetAppCompatFlags2 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAutoRotationState" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetAutoRotationState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAwarenessFromDpiAwarenessContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetAwarenessFromDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCIMSSM" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetCIMSSM h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCapture" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetCapture arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetCaretBlinkTime" => {
            log_red!(emu, "user32!GetCaretBlinkTime => 530");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 530; // default 530ms
        }
        "GetCaretPos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetCaretPos h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClassInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetClassInfoA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetClassInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetClassInfoW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetClassLongA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetClassLongW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetClassNameA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let max_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            let class_name = "mwemu_class";
            if buf != 0 && max_count > 0 {
                emu.maps.write_string(buf, class_name);
            }
            log_red!(
                emu,
                "user32!GetClassNameA hwnd:0x{:x} => '{}'",
                hwnd,
                class_name
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = class_name.len() as u64;
        }
        "GetClassNameW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let max_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as usize;
            let class_name = "mwemu_class";
            if buf != 0 && max_count > 0 {
                emu.maps.write_wide_string(buf, class_name);
            }
            log_red!(
                emu,
                "user32!GetClassNameW hwnd:0x{:x} => '{}'",
                hwnd,
                class_name
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = class_name.len() as u64;
        }
        "GetClassWord" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetClassWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipCursor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetClipCursor h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardAccessToken" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetClipboardAccessToken h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardData" => {
            let format = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetClipboardData format:{}", format);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetClipboardFormatNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetClipboardFormatNameA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetClipboardFormatNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetClipboardFormatNameW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetClipboardMetadata" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetClipboardMetadata h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardOwner" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetClipboardOwner arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetClipboardSequenceNumber" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetClipboardSequenceNumber arg1:0x{:x}", arg1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "GetClipboardViewer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetClipboardViewer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetComboBoxInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetComboBoxInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentInputMessageSource" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetCurrentInputMessageSource h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentMonitorTopologyId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetCurrentMonitorTopologyId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCursor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetCursor h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCursorFrameInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetCursorFrameInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCursorInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetCursorInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDCEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            log_red!(
                emu,
                "user32!GetDCEx arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            for _ in 0..3u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://GetDCEx");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "GetDCompositionHwndBitmap" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDCompositionHwndBitmap h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDesktopID" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDesktopID h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDialogBaseUnits" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDialogBaseUnits h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDialogControlDpiChangeBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDialogControlDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDialogDpiChangeBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDialogDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDisplayAutoRotationPreferences" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDisplayAutoRotationPreferences h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDisplayConfigBufferSizes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDisplayConfigBufferSizes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDlgCtrlID" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetDlgCtrlID hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDlgItem" => {
            let hdlg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let hwnd = helper::handler_create("dlg_item");
            log_red!(
                emu,
                "user32!GetDlgItem dlg:0x{:x} id:{} => 0x{:x}",
                hdlg,
                id,
                hwnd
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hwnd;
        }
        "GetDlgItemInt" => {
            let hdlg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let item_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let hndl = helper::handler_create(&format!("dlgitem_{}", item_id));
            log_red!(
                emu,
                "user32!GetDlgItemInt dlg:0x{:x} id:{} => 0x{:x}",
                hdlg,
                item_id,
                hndl
            );
            emu.regs_mut().rax = hndl;
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetDlgItemTextA" => {
            let _hdlg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let max_count = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(emu, "user32!GetDlgItemTextA id:{} buf:0x{:x}", id, buf);
            if buf != 0 && max_count > 0 {
                emu.maps.write_byte(buf, 0);
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "GetDlgItemTextW" => {
            let _hdlg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let buf = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let max_count = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            log_red!(emu, "user32!GetDlgItemTextW id:{} buf:0x{:x}", id, buf);
            if buf != 0 && max_count > 0 {
                emu.maps.write_word(buf, 0);
            }
            for _ in 0..4u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "GetDoubleClickTime" => {
            log_red!(emu, "user32!GetDoubleClickTime => 500");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 500; // default 500ms
        }
        "GetDpiAwarenessContextForProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDpiAwarenessContextForProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiForMonitorInternal" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDpiForMonitorInternal h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiForSystem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDpiForSystem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiForWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDpiForWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiFromDpiAwarenessContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetDpiFromDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetExtendedPointerDeviceProperty" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetExtendedPointerDeviceProperty h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGUIThreadInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetGUIThreadInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGestureConfig" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetGestureConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGestureExtraArgs" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetGestureExtraArgs h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGestureInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetGestureInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGuiResources" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetGuiResources h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetIconInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetIconInfo h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetIconInfoExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetIconInfoExA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetIconInfoExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetIconInfoExW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetInputDesktop" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetInputDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetInputLocaleInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetInputLocaleInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetInputState" => {
            log_red!(emu, "user32!GetInputState");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetInternalWindowPos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetInternalWindowPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKBCodePage" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetKBCodePage h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKeyNameTextA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetKeyNameTextA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetKeyNameTextW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetKeyNameTextW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetKeyboardLayoutList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetKeyboardLayoutList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKeyboardLayoutNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetKeyboardLayoutNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetKeyboardLayoutNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetKeyboardLayoutNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetKeyboardState" => {
            let buf = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetKeyboardState buf:0x{:x}", buf);
            if buf != 0 {
                for i in 0u64..256 {
                    emu.maps.write_byte(buf + i, 0);
                }
            }
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetLastActivePopup" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetLastActivePopup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLastInputInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetLastInputInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLayeredWindowAttributes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetLayeredWindowAttributes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetListBoxInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetListBoxInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationDesktopColorEffect" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopColorEffect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationDesktopMagnification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopMagnification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationDesktopSamplingMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopSamplingMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationLensCtxInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMagnificationLensCtxInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenu" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetMenu");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuBarInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuBarInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuCheckMarkDimensions" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuCheckMarkDimensions h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuContextHelpId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuDefaultItem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuDefaultItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetMenuInfo h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemCount" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetMenuItemCount");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemID" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuItemID h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetMenuItemInfoA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetMenuItemInfoW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemRect" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuItemRect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuState" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMenuState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuStringA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetMenuStringA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMenuStringW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetMenuStringW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMessageExtraInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMessageExtraInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMessagePos" => {
            log_red!(emu, "user32!GetMessagePos");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetMessageTime" => {
            let tick = emu.tick;
            log_red!(emu, "user32!GetMessageTime => {}", tick);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = tick as u64;
        }
        "GetMonitorInfoA" => {
            let hmonitor = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if info_ptr != 0 {
                emu.maps.write_dword(info_ptr + 4, 0);
                emu.maps.write_dword(info_ptr + 8, 0);
                emu.maps.write_dword(info_ptr + 12, 1920);
                emu.maps.write_dword(info_ptr + 16, 1080);
                emu.maps.write_dword(info_ptr + 20, 0);
                emu.maps.write_dword(info_ptr + 24, 0);
                emu.maps.write_dword(info_ptr + 28, 1920);
                emu.maps.write_dword(info_ptr + 32, 1080);
                emu.maps.write_dword(info_ptr + 36, 1);
            }
            log_red!(
                emu,
                "user32!GetMonitorInfoA hmon:0x{:x} info:0x{:x}",
                hmonitor,
                info_ptr
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMonitorInfoW" => {
            let hmonitor = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let info_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if info_ptr != 0 {
                emu.maps.write_dword(info_ptr + 4, 0);
                emu.maps.write_dword(info_ptr + 8, 0);
                emu.maps.write_dword(info_ptr + 12, 1920);
                emu.maps.write_dword(info_ptr + 16, 1080);
                emu.maps.write_dword(info_ptr + 20, 0);
                emu.maps.write_dword(info_ptr + 24, 0);
                emu.maps.write_dword(info_ptr + 28, 1920);
                emu.maps.write_dword(info_ptr + 32, 1080);
                emu.maps.write_dword(info_ptr + 36, 1);
            }
            log_red!(
                emu,
                "user32!GetMonitorInfoW hmon:0x{:x} info:0x{:x}",
                hmonitor,
                info_ptr
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetMouseMovePointsEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetMouseMovePointsEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNextDlgGroupItem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetNextDlgGroupItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNextDlgTabItem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetNextDlgTabItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetOpenClipboardWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetOpenClipboardWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetParent" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetParent hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetPhysicalCursorPos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPhysicalCursorPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerCursorId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerCursorId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDevice" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDevice h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceCursors" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceCursors h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceInputSpace" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceInputSpace h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceOrientation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceOrientation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceProperties" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceProperties h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceRects" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDeviceRects h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDevices" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerDevices h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameArrivalTimes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameArrivalTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFramePenInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFramePenInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFramePenInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFramePenInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTimes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchpadInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchpadInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchpadInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchpadInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerInputTransform" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerInputTransform h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerPenInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerPenInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerPenInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerPenInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchpadInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchpadInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchpadInfoHistory" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerTouchpadInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerType" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPointerType h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPriorityClipboardFormat" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetPriorityClipboardFormat h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessDefaultLayout" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetProcessDefaultLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessDpiAwarenessInternal" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetProcessDpiAwarenessInternal h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessUIContextInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetProcessUIContextInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProgmanWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetProgmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPropA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetPropA '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetPropW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetPropW '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetQueueStatus" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetQueueStatus h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputBuffer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetRawInputBuffer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputData" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetRawInputData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputDeviceInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetRawInputDeviceInfoA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetRawInputDeviceInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetRawInputDeviceInfoW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetRawInputDeviceList" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetRawInputDeviceList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawPointerDeviceData" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetRawPointerDeviceData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetReasonTitleFromReasonCode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetReasonTitleFromReasonCode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRegisteredRawInputDevices" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetRegisteredRawInputDevices h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollBarInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetScrollBarInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollInfo" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetScrollInfo p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollPos" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetScrollPos p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollRange" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetScrollRange p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSendMessageReceiver" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetSendMessageReceiver h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetShellChangeNotifyWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetShellChangeNotifyWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetShellWindow" => {
            log_red!(emu, "user32!GetShellWindow");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetSubMenu" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetSubMenu p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSysColor" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetSysColor");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xffffff;
        }
        "GetSysColorBrush" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hndl = helper::handler_create("brush");
            log_red!(emu, "user32!GetSysColorBrush => 0x{:x}", hndl);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hndl;
        }
        "GetSystemDpiForProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetSystemDpiForProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemMenu" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetSystemMenu h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemMetricsForDpi" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetSystemMetricsForDpi h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTabbedTextExtentA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetTabbedTextExtentA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTabbedTextExtentW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetTabbedTextExtentW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetTaskmanWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetTaskmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadDesktop" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetThreadDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadDpiAwarenessContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetThreadDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadDpiHostingBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetThreadDpiHostingBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTitleBarInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetTitleBarInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTopLevelWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetTopLevelWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTopWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!GetTopWindow arg1:0x{:x}", arg1);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://GetTopWindow");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "GetTouchInputInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetTouchInputInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUnpredictedMessagePos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetUnpredictedMessagePos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUpdateRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetUpdateRect p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUpdateRgn" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetUpdateRgn p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUpdatedClipboardFormats" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetUpdatedClipboardFormats h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUserObjectInformationA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetUserObjectInformationA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetUserObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetUserObjectSecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWinStationInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWinStationInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindow" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let cmd = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "user32!GetWindow hwnd:0x{:x} cmd:{}", hwnd, cmd);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetWindowBand" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowBand h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowCompositionAttribute" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowCompositionAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowCompositionInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowCompositionInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowContextHelpId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowDC" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hdc = helper::handler_create("dc");
            log_red!(emu, "user32!GetWindowDC hwnd:0x{:x} => 0x{:x}", hwnd, hdc);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hdc;
        }
        "GetWindowDisplayAffinity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowDisplayAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowDpiAwarenessContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowDpiHostingBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowDpiHostingBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowFeedbackSetting" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowFeedbackSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowMinimizeRect" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowMinimizeRect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowModuleFileName" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowModuleFileName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowModuleFileNameA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetWindowModuleFileNameA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetWindowModuleFileNameW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GetWindowModuleFileNameW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GetWindowPlacement" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let lpwndpl = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            if lpwndpl != 0 {
                emu.maps.write_dword(lpwndpl, 44); // length
                emu.maps.write_dword(lpwndpl + 4, 0); // flags
                emu.maps.write_dword(lpwndpl + 8, 1); // showCmd = SW_SHOWNORMAL
                emu.maps.write_dword(lpwndpl + 12, 0); // ptMinPosition.x
                emu.maps.write_dword(lpwndpl + 16, 0); // ptMinPosition.y
                emu.maps.write_dword(lpwndpl + 20, 0); // ptMaxPosition.x
                emu.maps.write_dword(lpwndpl + 24, 0); // ptMaxPosition.y
                emu.maps.write_dword(lpwndpl + 28, 0); // rcNormalPosition.left
                emu.maps.write_dword(lpwndpl + 32, 0); // rcNormalPosition.top
                emu.maps.write_dword(lpwndpl + 36, 800); // rcNormalPosition.right
                emu.maps.write_dword(lpwndpl + 40, 600); // rcNormalPosition.bottom
            }
            log_red!(
                emu,
                "user32!GetWindowPlacement hwnd:0x{:x} lpwndpl:0x{:x}",
                hwnd,
                lpwndpl
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowProcessHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowProcessHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowRgn" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowRgn h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowRgnBox" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowRgnBox h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowRgnEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowRgnEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowTextLengthA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let max_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let title = "mwemu_window";
            if buf_ptr != 0 && max_count > 0 {
                emu.maps.write_string(buf_ptr, title);
            }
            log_red!(
                emu,
                "user32!GetWindowTextLengthA hwnd:0x{:x} => '{}'",
                hwnd,
                title
            );
            emu.regs_mut().rax = title.len() as u64;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetWindowTextLengthW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let buf_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let max_count = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let title = "mwemu_window";
            if buf_ptr != 0 && max_count > 0 {
                emu.maps.write_wide_string(buf_ptr, title);
            }
            log_red!(
                emu,
                "user32!GetWindowTextLengthW hwnd:0x{:x} => '{}'",
                hwnd,
                title
            );
            emu.regs_mut().rax = title.len() as u64;
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetWindowThreadProcessId" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pid_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!GetWindowThreadProcessId hwnd:0x{:x}", hwnd);
            if pid_ptr != 0 {
                emu.maps.write_dword(pid_ptr, 0x123);
            }
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0x123;
        }
        "GetWindowWord" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GetWindowWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GhostWindowFromHungWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!GhostWindowFromHungWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GrayStringA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GrayStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "GrayStringW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!GrayStringW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..9 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "HandleDelegatedInput" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!HandleDelegatedInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HideCaret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!HideCaret arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HiliteMenuItem" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!HiliteMenuItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HungWindowFromGhostWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!HungWindowFromGhostWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPGetIMEA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPGetIMEA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IMPGetIMEW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPGetIMEW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IMPQueryIMEA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPQueryIMEA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IMPQueryIMEW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPQueryIMEW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IMPSetIMEA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPSetIMEA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IMPSetIMEW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPSetIMEW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ImpersonateDdeClientWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ImpersonateDdeClientWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InSendMessage" => {
            log_red!(emu, "user32!InSendMessage");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InSendMessageEx" => {
            log_red!(emu, "user32!InSendMessageEx");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "InflateRect" => {
            let rect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let dy = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as i32;
            if rect != 0 {
                let left = emu.maps.read_dword(rect).unwrap_or(0) as i32 - dx;
                let top = emu.maps.read_dword(rect + 4).unwrap_or(0) as i32 - dy;
                let right = emu.maps.read_dword(rect + 8).unwrap_or(0) as i32 + dx;
                let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0) as i32 + dy;
                emu.maps.write_dword(rect, left as u32);
                emu.maps.write_dword(rect + 4, top as u32);
                emu.maps.write_dword(rect + 8, right as u32);
                emu.maps.write_dword(rect + 12, bottom as u32);
            }
            log_red!(
                emu,
                "user32!InflateRect rect:0x{:x} dx:{} dy:{}",
                rect,
                dx,
                dy
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InheritWindowMonitor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InheritWindowMonitor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitDManipHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitDManipHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeGenericHidInjection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitializeGenericHidInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeInputDeviceInjection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitializeInputDeviceInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeLpkHooks" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitializeLpkHooks arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializePointerDeviceInjection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitializePointerDeviceInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializePointerDeviceInjectionEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitializePointerDeviceInjectionEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeTouchInjection" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InitializeTouchInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectDeviceInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectDeviceInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectGenericHidInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectGenericHidInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectKeyboardInput" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectKeyboardInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectMouseInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectMouseInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectPointerInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectPointerInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectSyntheticPointerInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectSyntheticPointerInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectTouchInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InjectTouchInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InputSpaceRegionFromPoint" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InputSpaceRegionFromPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InsertMenuA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!InsertMenuA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InsertMenuItemA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!InsertMenuItemA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InsertMenuItemW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!InsertMenuItemW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InsertMenuW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!InsertMenuW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "InternalGetWindowIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InternalGetWindowIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternalGetWindowText" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InternalGetWindowText arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IntersectRect" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let src2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if dst != 0 && src1 != 0 && src2 != 0 {
                let l1 = emu.maps.read_dword(src1).unwrap_or(0) as i32;
                let t1 = emu.maps.read_dword(src1 + 4).unwrap_or(0) as i32;
                let r1 = emu.maps.read_dword(src1 + 8).unwrap_or(0) as i32;
                let b1 = emu.maps.read_dword(src1 + 12).unwrap_or(0) as i32;
                let l2 = emu.maps.read_dword(src2).unwrap_or(0) as i32;
                let t2 = emu.maps.read_dword(src2 + 4).unwrap_or(0) as i32;
                let r2 = emu.maps.read_dword(src2 + 8).unwrap_or(0) as i32;
                let b2 = emu.maps.read_dword(src2 + 12).unwrap_or(0) as i32;
                let left = std::cmp::max(l1, l2);
                let top = std::cmp::max(t1, t2);
                let right = std::cmp::min(r1, r2);
                let bottom = std::cmp::min(b1, b2);
                if left < right && top < bottom {
                    emu.maps.write_dword(dst, left as u32);
                    emu.maps.write_dword(dst + 4, top as u32);
                    emu.maps.write_dword(dst + 8, right as u32);
                    emu.maps.write_dword(dst + 12, bottom as u32);
                    emu.regs_mut().rax = 1;
                } else {
                    emu.maps.write_dword(dst, 0);
                    emu.maps.write_dword(dst + 4, 0);
                    emu.maps.write_dword(dst + 8, 0);
                    emu.maps.write_dword(dst + 12, 0);
                    emu.regs_mut().rax = 0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
            log_red!(
                emu,
                "user32!IntersectRect dst:0x{:x} s1:0x{:x} s2:0x{:x}",
                dst,
                src1,
                src2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "InvalidateRgn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!InvalidateRgn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InvertRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!InvertRect p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaNumericA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaNumericA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaNumericW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaNumericW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsCharLowerA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharLowerA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "IsCharLowerW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharLowerW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "IsCharUpperA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharUpperA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "IsCharUpperW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharUpperW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "IsChild" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!IsChild arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsClipboardFormatAvailable" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!IsClipboardFormatAvailable");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsDialogMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsDialogMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsDialogMessageA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsDialogMessageA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsDialogMessageW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsDialogMessageW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "IsDlgButtonChecked" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!IsDlgButtonChecked p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsGUIThread" => {
            log_red!(emu, "user32!IsGUIThread");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "IsHungAppWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsHungAppWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsIconic" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!IsIconic hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "IsImmersiveProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsImmersiveProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsInDesktopWindowBand" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsInDesktopWindowBand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsInterceptWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsInterceptWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsMenu" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!IsMenu arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsMouseInPointerEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsMouseInPointerEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsOneCoreTransformMode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsOneCoreTransformMode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsProcessDPIAware" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsProcessDPIAware h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsRectEmpty" => {
            let rect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let empty = if rect != 0 {
                let left = emu.maps.read_dword(rect).unwrap_or(0);
                let top = emu.maps.read_dword(rect + 4).unwrap_or(0);
                let right = emu.maps.read_dword(rect + 8).unwrap_or(0);
                let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0);
                right <= left || bottom <= top
            } else {
                true
            };
            log_red!(emu, "user32!IsRectEmpty 0x{:x} => {}", rect, empty);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if empty { 1 } else { 0 };
        }
        "IsSETEnabled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsSETEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsServerSideWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsServerSideWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsThreadDesktopComposited" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsThreadDesktopComposited h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsThreadMessageQueueAttached" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsThreadMessageQueueAttached h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsThreadTSFEventAware" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!IsThreadTSFEventAware 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "IsTopLevelWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsTopLevelWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsTouchWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsTouchWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsValidDpiAwarenessContext" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsValidDpiAwarenessContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWinEventHookInstalled" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!IsWinEventHookInstalled 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "IsWindowArranged" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsWindowArranged arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowDisplayChangeSuppressed" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsWindowDisplayChangeSuppressed arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowEnabled" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!IsWindowEnabled hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowInDestroy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsWindowInDestroy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowRedirectedForPrint" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsWindowRedirectedForPrint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowUnicode" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsWindowUnicode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWow64Message" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!IsWow64Message arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsZoomed" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!IsZoomed hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "KillTimer" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let timer_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            helper::handler_close(timer_id);
            log_red!(emu, "user32!KillTimer hwnd:0x{:x} timer:{}", hwnd, timer_id);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadAcceleratorsA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "user32!LoadAcceleratorsA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://LoadAcceleratorsA");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadAcceleratorsW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "user32!LoadAcceleratorsW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://LoadAcceleratorsW");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadBitmapA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadBitmapA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadBitmapW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadBitmapW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadCursorFromFileA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!LoadCursorFromFileA arg1:0x{:x}", arg1);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://LoadCursorFromFileA");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadCursorFromFileW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!LoadCursorFromFileW arg1:0x{:x}", arg1);
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://LoadCursorFromFileW");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadCursorW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(
                emu,
                "user32!LoadCursorW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://LoadCursorW");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadIconW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "user32!LoadIconW arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://LoadIconW");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadImageA" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let arg6 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            log_red!(
                emu,
                "user32!LoadImageA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadImageW" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let arg3 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let arg4 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let arg5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            let arg6 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            log_red!(
                emu,
                "user32!LoadImageW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x}",
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6
            );
            for _ in 0..6u32 {
                emu.stack_pop32(false);
            }
            let hndl = helper::handler_create("user32://unknown");
            assert!(hndl < 0x00000001_00000000);
            emu.regs_mut().rax = hndl;
        }
        "LoadKeyboardLayoutA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadKeyboardLayoutA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadKeyboardLayoutEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LoadKeyboardLayoutEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadKeyboardLayoutW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadKeyboardLayoutW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadLocalFonts" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LoadLocalFonts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadMenuA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadMenuA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadMenuIndirectA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadMenuIndirectA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadMenuIndirectW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadMenuIndirectW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadMenuW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadMenuW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "LoadRemoteFonts" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LoadRemoteFonts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LockSetForegroundWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!LockSetForegroundWindow 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LockWindowStation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!LockWindowStation 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LockWindowUpdate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!LockWindowUpdate 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LockWorkStation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!LockWorkStation 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "LogicalToPhysicalPoint" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LogicalToPhysicalPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LogicalToPhysicalPointForPerMonitorDPI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LogicalToPhysicalPointForPerMonitorDPI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LookupIconIdFromDirectory" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LookupIconIdFromDirectory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LookupIconIdFromDirectoryEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!LookupIconIdFromDirectoryEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MBToWCSEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!MBToWCSEx arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MBToWCSExt" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!MBToWCSExt arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MB_GetString" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MB_GetString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MITGetCursorUpdateHandle" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MITGetCursorUpdateHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MITSetLastInputRecipient" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MITSetLastInputRecipient arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MITSynthesizeTouchInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MITSynthesizeTouchInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MakeThreadTSFEventAware" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!MakeThreadTSFEventAware 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "MapDialogRect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MapDialogRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapPointsByVisualIdentifier" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MapPointsByVisualIdentifier arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapVirtualKeyExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!MapVirtualKeyExA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "MapVirtualKeyExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!MapVirtualKeyExW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "MapVisualRelativePoints" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MapVisualRelativePoints arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapWindowPoints" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MapWindowPoints p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MenuItemFromPoint" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MenuItemFromPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MenuWindowProcA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!MenuWindowProcA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "MenuWindowProcW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!MenuWindowProcW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "MessageBeep" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!MessageBeep arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MessageBoxExA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let text_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let caption_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mb_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let text = if text_ptr != 0 {
                emu.maps.read_string(text_ptr)
            } else {
                String::new()
            };
            let caption = if caption_ptr != 0 {
                emu.maps.read_string(caption_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!MessageBoxExA '{}': '{}'", caption, text);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "MessageBoxExW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let text_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let caption_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mb_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let text = if text_ptr != 0 {
                emu.maps.read_wide_string(text_ptr)
            } else {
                String::new()
            };
            let caption = if caption_ptr != 0 {
                emu.maps.read_wide_string(caption_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!MessageBoxExW '{}': '{}'", caption, text);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "MessageBoxIndirectA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let text_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let caption_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mb_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let text = if text_ptr != 0 {
                emu.maps.read_string(text_ptr)
            } else {
                String::new()
            };
            let caption = if caption_ptr != 0 {
                emu.maps.read_string(caption_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!MessageBoxIndirectA '{}': '{}'", caption, text);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "MessageBoxIndirectW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let text_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let caption_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mb_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let text = if text_ptr != 0 {
                emu.maps.read_wide_string(text_ptr)
            } else {
                String::new()
            };
            let caption = if caption_ptr != 0 {
                emu.maps.read_wide_string(caption_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!MessageBoxIndirectW '{}': '{}'", caption, text);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "MessageBoxTimeoutA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let text_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let caption_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mb_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let text = if text_ptr != 0 {
                emu.maps.read_string(text_ptr)
            } else {
                String::new()
            };
            let caption = if caption_ptr != 0 {
                emu.maps.read_string(caption_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!MessageBoxTimeoutA '{}': '{}'", caption, text);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "MessageBoxTimeoutW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let text_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let caption_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let mb_type = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let text = if text_ptr != 0 {
                emu.maps.read_wide_string(text_ptr)
            } else {
                String::new()
            };
            let caption = if caption_ptr != 0 {
                emu.maps.read_wide_string(caption_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!MessageBoxTimeoutW '{}': '{}'", caption, text);
            emu.regs_mut().rax = 1;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "ModifyMenuA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ModifyMenuA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ModifyMenuW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ModifyMenuW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "MonitorFromPoint" => {
            let pt_x = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as i32;
            let pt_y = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let hmon = helper::handler_create("monitor");
            log_red!(
                emu,
                "user32!MonitorFromPoint ({},{}) flags:0x{:x} => 0x{:x}",
                pt_x,
                pt_y,
                flags,
                hmon
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hmon;
        }
        "MonitorFromRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!MonitorFromRect p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MonitorFromWindow" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let hmon = helper::handler_create("monitor");
            log_red!(
                emu,
                "user32!MonitorFromWindow hwnd:0x{:x} flags:0x{:x} => 0x{:x}",
                hwnd,
                flags,
                hmon
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hmon;
        }
        "MoveWindow" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let x = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let y = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as i32;
            let width = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as i32;
            let height = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as i32;
            let repaint = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
            log_red!(
                emu,
                "user32!MoveWindow hwnd:0x{:x} x:{} y:{} w:{} h:{} repaint:{}",
                hwnd,
                x,
                y,
                width,
                height,
                repaint
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MsgWaitForMultipleObjects" => {
            let count = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let handles = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let wait_all = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) != 0;
            let ms = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let wake_mask = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            helper::advance_tick(emu, ms as u64);
            log_red!(
                emu,
                "user32!MsgWaitForMultipleObjects count:{} all:{} ms:{} mask:0x{:x}",
                count,
                wait_all,
                ms,
                wake_mask
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "MsgWaitForMultipleObjectsEx" => {
            let count = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let handles = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let ms = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let wake_mask = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            helper::advance_tick(emu, ms as u64);
            log_red!(
                emu,
                "user32!MsgWaitForMultipleObjectsEx count:{} ms:{} mask:0x{:x} flags:0x{:x}",
                count,
                ms,
                wake_mask,
                flags
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0;
        }
        "NotifyOverlayWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!NotifyOverlayWindow 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "NotifyWinEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!NotifyWinEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OemKeyScan" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!OemKeyScan h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OemToCharA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!OemToCharA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OemToCharBuffA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!OemToCharBuffA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OemToCharBuffW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!OemToCharBuffW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OemToCharW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!OemToCharW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OffsetRect" => {
            let rect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let dx = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let dy = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as i32;
            if rect != 0 {
                let left = emu.maps.read_dword(rect).unwrap_or(0) as i32 + dx;
                let top = emu.maps.read_dword(rect + 4).unwrap_or(0) as i32 + dy;
                let right = emu.maps.read_dword(rect + 8).unwrap_or(0) as i32 + dx;
                let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0) as i32 + dy;
                emu.maps.write_dword(rect, left as u32);
                emu.maps.write_dword(rect + 4, top as u32);
                emu.maps.write_dword(rect + 8, right as u32);
                emu.maps.write_dword(rect + 12, bottom as u32);
            }
            log_red!(
                emu,
                "user32!OffsetRect rect:0x{:x} dx:{} dy:{}",
                rect,
                dx,
                dy
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenClipboard" => {
            let hwnd_owner = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("clipboard");
            log_red!(
                emu,
                "user32!OpenClipboard owner:0x{:x} => 0x{:x}",
                hwnd_owner,
                hndl
            );
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenDesktopA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!OpenDesktopA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenDesktopW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!OpenDesktopW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenIcon" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("OpenIcon");
            log_red!(emu, "user32!OpenIcon => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OpenInputDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("OpenInputDesktop");
            log_red!(emu, "user32!OpenInputDesktop => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OpenThreadDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hndl = helper::handler_create("OpenThreadDesktop");
            log_red!(emu, "user32!OpenThreadDesktop => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "OpenWindowStationA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!OpenWindowStationA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "OpenWindowStationW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!OpenWindowStationW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PackDDElParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PackDDElParam arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackTouchHitTestingProximityEvaluation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PackTouchHitTestingProximityEvaluation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PaintDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PaintDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PaintMenuBar" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PaintMenuBar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PaintMonitor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PaintMonitor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PhysicalToLogicalPoint" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PhysicalToLogicalPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PhysicalToLogicalPointForPerMonitorDPI" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PhysicalToLogicalPointForPerMonitorDPI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PostThreadMessageA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!PostThreadMessageA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PostThreadMessageW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!PostThreadMessageW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrintWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PrintWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconsA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconsA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconsW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconsW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "PrivateRegisterICSProc" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!PrivateRegisterICSProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PtInRect" => {
            let rect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let pt_x = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as i32;
            let pt_y = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as i32;
            let inside = if rect != 0 {
                let left = emu.maps.read_dword(rect).unwrap_or(0) as i32;
                let top = emu.maps.read_dword(rect + 4).unwrap_or(0) as i32;
                let right = emu.maps.read_dword(rect + 8).unwrap_or(0) as i32;
                let bottom = emu.maps.read_dword(rect + 12).unwrap_or(0) as i32;
                pt_x >= left && pt_x < right && pt_y >= top && pt_y < bottom
            } else {
                false
            };
            log_red!(
                emu,
                "user32!PtInRect rect:0x{:x} pt:({},{}) => {}",
                rect,
                pt_x,
                pt_y,
                inside
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if inside { 1 } else { 0 };
        }
        "QueryBSDRWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!QueryBSDRWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryDisplayConfig" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!QueryDisplayConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QuerySendMessage" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!QuerySendMessage h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMAddInputObserver" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMAddInputObserver arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMAreSiblingDevices" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMAreSiblingDevices arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMDeviceIoControl" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMDeviceIoControl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMEnableMonitorMappingForDevice" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMEnableMonitorMappingForDevice arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMFreeInputBuffer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMFreeInputBuffer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDevicePreparsedData" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMGetDevicePreparsedData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDevicePreparsedDataLockfree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!RIMGetDevicePreparsedDataLockfree 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RIMGetDeviceProperties" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMGetDeviceProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDevicePropertiesLockfree" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!RIMGetDevicePropertiesLockfree 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RIMGetPhysicalDeviceRect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMGetPhysicalDeviceRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetSourceProcessId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMGetSourceProcessId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMObserveNextInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMObserveNextInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMOnAsyncPnpWorkNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMOnAsyncPnpWorkNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMOnPnpNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMOnPnpNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMOnTimerNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!RIMOnTimerNotification 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RIMQueryDevicePath" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMQueryDevicePath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMReadInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMReadInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMRegisterForInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMRegisterForInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMRegisterForInputEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMRegisterForInputEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMRemoveInputObserver" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMRemoveInputObserver arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMSetExtendedDeviceProperty" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMSetExtendedDeviceProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMSetTestModeStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMSetTestModeStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMUnregisterForInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMUnregisterForInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMUpdateInputObserverRegistration" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RIMUpdateInputObserverRegistration arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RaiseLowerShellWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RaiseLowerShellWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealChildWindowFromPoint" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RealChildWindowFromPoint p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealGetWindowClass" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RealGetWindowClass arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealGetWindowClassA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RealGetWindowClassA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RealGetWindowClassW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RealGetWindowClassW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReasonCodeNeedsBugID" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ReasonCodeNeedsBugID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReasonCodeNeedsComment" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ReasonCodeNeedsComment arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RecordShutdownReason" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RecordShutdownReason arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RedrawWindow" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RedrawWindow p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterBSDRWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterBSDRWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterClassExA" => {
            let wndclass_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let atom = 0xc000u64 + (wndclass_ptr & 0xfff);
            log_red!(emu, "user32!RegisterClassExA => atom:0x{:x}", atom);
            emu.set_last_error(0);
            emu.regs_mut().rax = atom;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "RegisterClassExW" => {
            let wndclass_ptr = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let atom = 0xc000u64 + (wndclass_ptr & 0xfff);
            log_red!(emu, "user32!RegisterClassExW => atom:0x{:x}", atom);
            emu.set_last_error(0);
            emu.regs_mut().rax = atom;
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "RegisterClipboardFormatA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RegisterClipboardFormatA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0xc000;
        }
        "RegisterClipboardFormatW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RegisterClipboardFormatW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 0xc000;
        }
        "RegisterCloakedNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterCloakedNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterDManipHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterDManipHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterDeviceNotificationA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!RegisterDeviceNotificationA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegisterDeviceNotificationW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!RegisterDeviceNotificationW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegisterErrorReportingDialog" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterErrorReportingDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterForCustomDockTargets" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterForCustomDockTargets arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterForTooltipDismissNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterForTooltipDismissNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterFrostWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterFrostWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterGhostWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterGhostWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterHotKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterLogonProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterLogonProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterMessagePumpHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterMessagePumpHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPointerDeviceNotifications" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterPointerDeviceNotifications arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPointerInputTarget" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterPointerInputTarget arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPointerInputTargetEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterPointerInputTargetEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPowerSettingNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterPowerSettingNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterRawInputDevices" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterRawInputDevices arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterServicesProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterServicesProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterSessionPort" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterSessionPort h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterShellHookWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterShellHookWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterSuspendResumeNotification" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterSuspendResumeNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterSystemThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterSystemThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTasklist" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterTasklist arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchHitTestingWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchHitTestingWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchpadCapableThread" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchpadCapableThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchpadCapableWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterTouchpadCapableWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterUserApiHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RegisterUserApiHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterWindowMessageA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!RegisterWindowMessageA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RegisterWindowMessageW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!RegisterWindowMessageW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReleaseCapture" => {
            log_red!(emu, "user32!ReleaseCapture");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseDwmHitTestWaiters" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            helper::handler_close(handle);
            log_red!(emu, "user32!ReleaseDwmHitTestWaiters handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RemoveClipboardFormatListener" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RemoveClipboardFormatListener h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveInjectionDevice" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RemoveInjectionDevice h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveMenu" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!RemoveMenu p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemovePropA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RemovePropA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RemovePropW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RemovePropW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "RemoveThreadTSFEventAwareness" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!RemoveThreadTSFEventAwareness 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "RemoveVisualIdentifier" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!RemoveVisualIdentifier h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReplyMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ReplyMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReportInertia" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ReportInertia arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResolveDesktopForWOW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ResolveDesktopForWOW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ReuseDDElParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ReuseDDElParam arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScreenToClient" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let pt = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            log_red!(emu, "user32!ScreenToClient hwnd:0x{:x} pt:0x{:x}", hwnd, pt);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollChildren" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ScrollChildren arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollDC" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!ScrollDC arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollWindow" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ScrollWindow p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollWindowEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ScrollWindowEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendDlgItemMessageA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SendDlgItemMessageA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SendDlgItemMessageW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SendDlgItemMessageW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SendIMEMessageExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SendIMEMessageExA '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SendIMEMessageExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SendIMEMessageExW '{}'  p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SendInput" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!SendInput h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendMessageCallbackA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let wparam = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let lparam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SendMessageCallbackA hwnd:0x{:x} msg:0x{:x} wp:0x{:x} lp:0x{:x}",
                hwnd,
                msg,
                wparam,
                lparam
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "SendMessageCallbackW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let wparam = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let lparam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SendMessageCallbackW hwnd:0x{:x} msg:0x{:x} wp:0x{:x} lp:0x{:x}",
                hwnd,
                msg,
                wparam,
                lparam
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "SendMessageTimeoutA" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let wparam = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let lparam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SendMessageTimeoutA hwnd:0x{:x} msg:0x{:x} wp:0x{:x} lp:0x{:x}",
                hwnd,
                msg,
                wparam,
                lparam
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "SendMessageTimeoutW" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let msg = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let wparam = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let lparam = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SendMessageTimeoutW hwnd:0x{:x} msg:0x{:x} wp:0x{:x} lp:0x{:x}",
                hwnd,
                msg,
                wparam,
                lparam
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
        }
        "SendNotifyMessageA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SendNotifyMessageA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SendNotifyMessageW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SendNotifyMessageW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetActiveWindow" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!SetActiveWindow hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = hwnd;
        }
        "SetAdditionalForegroundBoostProcesses" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetAdditionalForegroundBoostProcesses h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCapture" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!SetCapture hwnd:0x{:x}", hwnd);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SetCaretBlinkTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetCaretBlinkTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCaretPos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!SetCaretPos h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetClassLongA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetClassLongA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "SetClassLongW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetClassLongW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "SetClassWord" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetClassWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetClipboardData" => {
            let format = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let hdata = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetClipboardData format:{} data:0x{:x}",
                format,
                hdata
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = hdata;
        }
        "SetClipboardViewer" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetClipboardViewer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCoalescableTimer" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!SetCoalescableTimer 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetCoreWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetCoreWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCoveredWindowStates" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetCoveredWindowStates h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCursorContents" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetCursorContents h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDebugErrorLevel" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDebugErrorLevel h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDeskWallpaper" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDeskWallpaper h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDesktopColorTransform" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDesktopColorTransform h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDialogControlDpiChangeBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDialogControlDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDialogDpiChangeBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDialogDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDisplayAutoRotationPreferences" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDisplayAutoRotationPreferences h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDisplayConfig" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDisplayConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDlgItemInt" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDlgItemInt p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDlgItemTextA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetDlgItemTextA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetDlgItemTextW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetDlgItemTextW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetDoubleClickTime" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetDoubleClickTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFeatureReportResponse" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetFeatureReportResponse h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetForegroundRedirectionForActivationObject" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetForegroundRedirectionForActivationObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFullscreenMagnifierOffsetsDWMUpdated" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetFullscreenMagnifierOffsetsDWMUpdated h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetGestureConfig" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetGestureConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetInternalWindowPos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetInternalWindowPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetKeyboardState" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!SetKeyboardState");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetLastErrorEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetLastErrorEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetLayeredWindowAttributes" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetLayeredWindowAttributes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopColorEffect" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopColorEffect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopMagnification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopMagnification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopMagnifierOffsetsDWMUpdated" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopMagnifierOffsetsDWMUpdated h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopSamplingMode" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopSamplingMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationLensCtxInformation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMagnificationLensCtxInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMaxTouchpadSensitivity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMaxTouchpadSensitivity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenu" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!SetMenu p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuContextHelpId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMenuContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuDefaultItem" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMenuDefaultItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!SetMenuInfo h:0x{:x} arg2:0x{:x}", handle, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuItemBitmaps" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMenuItemBitmaps h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuItemInfoA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetMenuItemInfoA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetMenuItemInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetMenuItemInfoW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetMessageExtraInfo" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMessageExtraInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMessageQueue" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMessageQueue h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMirrorRendering" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetMirrorRendering h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetParent" => {
            let hwnd_child = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let hwnd_parent = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetParent child:0x{:x} parent:0x{:x}",
                hwnd_child,
                hwnd_parent
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = hwnd_parent;
        }
        "SetPhysicalCursorPos" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetPhysicalCursorPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetPointerDeviceInputSpace" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetPointerDeviceInputSpace h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDPIAware" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessDPIAware h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDefaultLayout" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessDefaultLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDpiAwarenessContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDpiAwarenessInternal" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessDpiAwarenessInternal h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessLaunchForegroundPolicy" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessLaunchForegroundPolicy h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessRestrictionExemption" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessRestrictionExemption h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessWindowStation" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProcessWindowStation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProgmanWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetProgmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetPropA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SetPropA '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetPropW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SetPropW '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetRect" => {
            let rect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let left = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let top = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let right = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let bottom = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
            if rect != 0 {
                emu.maps.write_dword(rect, left);
                emu.maps.write_dword(rect + 4, top);
                emu.maps.write_dword(rect + 8, right);
                emu.maps.write_dword(rect + 12, bottom);
            }
            log_red!(
                emu,
                "user32!SetRect ({},{},{},{}) => 0x{:x}",
                left,
                top,
                right,
                bottom,
                rect
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetRectEmpty" => {
            let rect = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            if rect != 0 {
                emu.maps.write_dword(rect, 0);
                emu.maps.write_dword(rect + 4, 0);
                emu.maps.write_dword(rect + 8, 0);
                emu.maps.write_dword(rect + 12, 0);
            }
            log_red!(emu, "user32!SetRectEmpty 0x{:x}", rect);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetScrollInfo" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetScrollInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetScrollPos" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetScrollPos p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetScrollRange" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetScrollRange p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetShellChangeNotifyWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetShellChangeNotifyWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetShellWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetShellWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetShellWindowEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetShellWindowEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSysColors" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!SetSysColors p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSysColorsTemp" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetSysColorsTemp h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSystemCursor" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetSystemCursor h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSystemMenu" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetSystemMenu h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetTaskmanWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetTaskmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadCursorCreationScaling" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetThreadCursorCreationScaling h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDesktop" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetThreadDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDpiAwarenessContext" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetThreadDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDpiHostingBehavior" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetThreadDpiHostingBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadInputBlocked" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!SetThreadInputBlocked 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetTimer" => {
            let hwnd = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            let id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
            let elapse = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
            let callback = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
            let timer_handle = helper::handler_create("timer");
            log_red!(
                emu,
                "user32!SetTimer hwnd:0x{:x} id:{} ms:{} cb:0x{:x} => 0x{:x}",
                hwnd,
                id,
                elapse,
                callback,
                timer_handle
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = timer_handle;
        }
        "SetUserObjectCapability" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetUserObjectCapability h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUserObjectInformationA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetUserObjectInformationA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUserObjectInformationW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetUserObjectInformationW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetUserObjectSecurity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetUserObjectSecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWinEventHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!SetWinEventHook 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SetWindowBand" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowBand h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowCompositionAttribute" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowCompositionAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowCompositionTransition" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowCompositionTransition h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowContextHelpId" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowDisplayAffinity" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowDisplayAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowFeedbackSetting" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowFeedbackSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowMessageCapability" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowMessageCapability h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowPlacement" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowPlacement h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowRgn" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowRgn h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowRgnEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowRgnEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowStationUser" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowStationUser h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowWord" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SetWindowWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowsHookA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetWindowsHookA '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetWindowsHookExAW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetWindowsHookExAW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "SetWindowsHookExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            let hndl = helper::handler_create("hook");
            log_red!(
                emu,
                "user32!SetWindowsHookExW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x} => 0x{:x}",
                s,
                p1,
                p2,
                p3,
                hndl
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = hndl;
        }
        "SetWindowsHookW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetWindowsHookW '{}'  p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "ShellForegroundBoostProcess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellForegroundBoostProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellHandwritingDelegateInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellHandwritingDelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellHandwritingHandleDelegatedInput" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellHandwritingHandleDelegatedInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellHandwritingUndelegateInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellHandwritingUndelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellMigrateWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellMigrateWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellRegisterHotKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellRegisterHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellSetWindowPos" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShellSetWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowCaret" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!ShowCaret arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowOwnedPopups" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShowOwnedPopups arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowScrollBar" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!ShowScrollBar p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowStartGlass" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShowStartGlass arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowSystemCursor" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShowSystemCursor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowWindowAsync" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ShowWindowAsync arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShutdownBlockReasonCreate" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!ShutdownBlockReasonCreate 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ShutdownBlockReasonDestroy" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!ShutdownBlockReasonDestroy 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "ShutdownBlockReasonQuery" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!ShutdownBlockReasonQuery 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SignalRedirectionStartComplete" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!SignalRedirectionStartComplete 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "SkipPointerFrameMessages" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SkipPointerFrameMessages arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SoftModalMessageBox" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SoftModalMessageBox arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SoundSentry" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SoundSentry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SubtractRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(emu, "user32!SubtractRect p1:0x{:x} p2:0x{:x}", p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SuppressWindowDisplayChange" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SuppressWindowDisplayChange arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwapMouseButton" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SwapMouseButton arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchDesktop" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SwitchDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchDesktopWithFade" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SwitchDesktopWithFade arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchToThisWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SwitchToThisWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SystemParametersInfoForDpi" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!SystemParametersInfoForDpi arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SystemParametersInfoW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SystemParametersInfoW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "TabbedTextOutA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!TabbedTextOutA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
        }
        "TabbedTextOutW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            let p7 = emu.maps.read_dword(emu.regs().get_esp() + 28).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!TabbedTextOutW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..8 {
                emu.stack_pop32(false);
            }
        }
        "TileChildWindows" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TileChildWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TileWindows" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TileWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ToAscii" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ToAscii p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..5 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ToAsciiEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ToAsciiEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ToUnicode" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ToUnicode p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ToUnicodeEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ToUnicodeEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TrackMouseEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!TrackMouseEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "TrackPopupMenu" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            let p6 = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TrackPopupMenu p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..7 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TrackPopupMenuEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let p4 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
            let p5 = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TrackPopupMenuEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            for _ in 0..6 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TranslateAccelerator" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TranslateAccelerator p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TranslateAcceleratorA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!TranslateAcceleratorA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "TranslateAcceleratorW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!TranslateAcceleratorW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "TranslateMDISysAccel" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TranslateMDISysAccel arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TranslateMessageEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!TranslateMessageEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UndelegateInput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UndelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnhookWinEvent" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!UnhookWinEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnhookWindowsHook" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnhookWindowsHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnhookWindowsHookEx" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            log_red!(emu, "user32!UnhookWindowsHookEx");
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnionRect" => {
            let dst = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let src1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let src2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            if dst != 0 && src1 != 0 && src2 != 0 {
                let l1 = emu.maps.read_dword(src1).unwrap_or(0) as i32;
                let t1 = emu.maps.read_dword(src1 + 4).unwrap_or(0) as i32;
                let r1 = emu.maps.read_dword(src1 + 8).unwrap_or(0) as i32;
                let b1 = emu.maps.read_dword(src1 + 12).unwrap_or(0) as i32;
                let l2 = emu.maps.read_dword(src2).unwrap_or(0) as i32;
                let t2 = emu.maps.read_dword(src2 + 4).unwrap_or(0) as i32;
                let r2 = emu.maps.read_dword(src2 + 8).unwrap_or(0) as i32;
                let b2 = emu.maps.read_dword(src2 + 12).unwrap_or(0) as i32;
                emu.maps.write_dword(dst, std::cmp::min(l1, l2) as u32);
                emu.maps.write_dword(dst + 4, std::cmp::min(t1, t2) as u32);
                emu.maps.write_dword(dst + 8, std::cmp::max(r1, r2) as u32);
                emu.maps.write_dword(dst + 12, std::cmp::max(b1, b2) as u32);
            }
            log_red!(
                emu,
                "user32!UnionRect dst:0x{:x} s1:0x{:x} s2:0x{:x}",
                dst,
                src1,
                src2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UnloadKeyboardLayout" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnloadKeyboardLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockWindowStation" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!UnlockWindowStation 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "UnpackDDElParam" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnpackDDElParam arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterClassA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!UnregisterClassA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UnregisterClassW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!UnregisterClassW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "UnregisterDeviceNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterDeviceNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterHotKey" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterMessagePumpHook" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterMessagePumpHook h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterPointerInputTarget" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterPointerInputTarget h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterPointerInputTargetEx" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterPointerInputTargetEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterPowerSettingNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterPowerSettingNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterSessionPort" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterSessionPort h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterSuspendResumeNotification" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterSuspendResumeNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterTouchWindow" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterTouchWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterUserApiHook" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UnregisterUserApiHook h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateDefaultDesktopThumbnail" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UpdateDefaultDesktopThumbnail arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateLayeredWindow" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UpdateLayeredWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateLayeredWindowIndirect" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UpdateLayeredWindowIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdatePerUserSystemParameters" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UpdatePerUserSystemParameters arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateWindowInputSinkHints" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UpdateWindowInputSinkHints arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "User32InitializeImmEntryTable" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!User32InitializeImmEntryTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserClientDllInitialize" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UserClientDllInitialize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserHandleGrantAccess" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UserHandleGrantAccess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserLpkPSMTextOut" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UserLpkPSMTextOut arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserLpkTabbedTextOut" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UserLpkTabbedTextOut arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserRealizePalette" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UserRealizePalette arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserRegisterWowHandlers" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!UserRegisterWowHandlers h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VRipOutput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!VRipOutput arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VTagOutput" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!VTagOutput arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ValidateRect" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!ValidateRect p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ValidateRgn" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!ValidateRgn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VkKeyScanA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanA '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "VkKeyScanExA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanExA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VkKeyScanExW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanExW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "VkKeyScanW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanW '{}'", s);
            for _ in 0..1 {
                emu.stack_pop32(false);
            }
        }
        "WCSToMBEx" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!WCSToMBEx arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WINNLSEnableIME" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!WINNLSEnableIME arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WINNLSGetEnableStatus" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!WINNLSGetEnableStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WINNLSGetIMEHotkey" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!WINNLSGetIMEHotkey arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WaitForInputIdle" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!WaitForInputIdle 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitForRedirectionStartComplete" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!WaitForRedirectionStartComplete 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WaitMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!WaitMessage 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "WinHelpA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!WinHelpA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WinHelpW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let p3 = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!WinHelpW '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                s,
                p1,
                p2,
                p3
            );
            for _ in 0..4 {
                emu.stack_pop32(false);
            }
            emu.regs_mut().rax = 1;
        }
        "WindowFromDC" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!WindowFromDC arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WindowFromPhysicalPoint" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!WindowFromPhysicalPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WindowFromPoint" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(emu, "user32!WindowFromPoint p1:0x{:x}", p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "Wow64Transition" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!Wow64Transition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_UserTestTokenForInteractive" => {
            let handle = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!_UserTestTokenForInteractive h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "gSharedInfo" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!gSharedInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "gapfnScSendMessage" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            log_red!(
                emu,
                "user32!gapfnScSendMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "keybd_event" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!keybd_event 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "mouse_event" => {
            let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
            log_red!(emu, "user32!mouse_event 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
            for _ in 0..1u32 {
                emu.stack_pop32(false);
            }
        }
        "wvsprintfA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!wvsprintfA '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "wvsprintfW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!wvsprintfW '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "GetClassLongPtrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongPtrA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetClassLongPtrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongPtrW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetWindowLongPtrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetWindowLongPtrA '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "GetWindowLongPtrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetWindowLongPtrW '{}' p1:0x{:x}", s, p1);
            for _ in 0..2 {
                emu.stack_pop32(false);
            }
        }
        "SetClassLongPtrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetClassLongPtrA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "SetClassLongPtrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetClassLongPtrW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "SetWindowLongPtrA" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetWindowLongPtrA '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }
        "SetWindowLongPtrW" => {
            let p0 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
            let p1 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
            let p2 = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(
                emu,
                "user32!SetWindowLongPtrW '{}' p1:0x{:x} p2:0x{:x}",
                s,
                p1,
                p2
            );
            for _ in 0..3 {
                emu.stack_pop32(false);
            }
        }

        _ => {
            if emu.cfg.skip_unimplemented {
                let nparams = emu.banzai.get_params(api);
                if nparams >= 0 {
                    for _ in 0..nparams {
                        emu.stack_pop32(false);
                    }
                }
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} (nparams={}) at 0x{:x}",
                    addr,
                    api,
                    nparams,
                    emu.regs().rip
                );
                emu.regs_mut().rax = 0;
                return String::new();
            }

            if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                serialization::Serialization::dump(
                    emu,
                    emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                );
            }

            log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
            log::warn!(
                "calling unimplemented API 0x{:x} {} at 0x{:x}",
                addr,
                api,
                emu.regs().rip
            );
            return api.to_ascii_lowercase();
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
    emu.regs_mut().rax = 0x00010010;
}

fn wsprintfW(emu: &mut emu::Emu) {
    let out = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let in_fmt = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let fmt = emu.maps.read_wide_string(in_fmt);
    let mut result = String::new();
    let mut arg_offset: u64 = 8;
    let mut chars = fmt.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            if chars.peek() == Some(&'%') {
                result.push('%');
                chars.next();
            } else {
                let specifier = chars.next();
                let val = emu
                    .maps
                    .read_dword(emu.regs().get_esp() + arg_offset)
                    .unwrap_or(0) as u64;
                arg_offset += 4;
                match specifier {
                    Some('d') | Some('i') => result.push_str(&format!("{}", val as i32)),
                    Some('u') => result.push_str(&format!("{}", val)),
                    Some('x') => result.push_str(&format!("{:x}", val)),
                    Some('X') => result.push_str(&format!("{:X}", val)),
                    Some('s') => {
                        let s = emu.maps.read_wide_string(val);
                        result.push_str(&s);
                    }
                    Some('c') => result.push(val as u8 as char),
                    _ => result.push('?'),
                }
            }
        } else {
            result.push(c);
        }
    }

    if out != 0 {
        emu.maps.write_wide_string(out, &result);
    }

    log_red!(emu, "user32!wsprintfW fmt:`{}` out:`{}`", fmt, &result);

    let nparams = 2 + (arg_offset as u32 - 8) / 4;
    for _ in 0..nparams {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = result.len() as u64;
}

fn GetProcessWindowStation(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetProcessWindowStation");
    let hndl = helper::handler_create("user32://winsta");
    emu.regs_mut().rax = hndl;
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
    let ui_param = arg(emu, 1);
    let pv_param = arg(emu, 2) as u64;
    let _f_winini = arg(emu, 3);
    log_red!(
        emu,
        "user32!SystemParametersInfoA uiAction:0x{:x} uiParam:{} pvParam:0x{:x}",
        action,
        ui_param,
        pv_param
    );
    if pv_param != 0 {
        match action {
            0x0030 => {
                // SPI_GETWORKAREA - fill RECT
                emu.maps.write_dword(pv_param, 0);
                emu.maps.write_dword(pv_param + 4, 0);
                emu.maps.write_dword(pv_param + 8, 1920);
                emu.maps.write_dword(pv_param + 12, 1040);
            }
            0x0048 => {
                // SPI_GETNONCLIENTMETRICS
                emu.maps.write_dword(pv_param, ui_param);
            }
            _ => {}
        }
    }
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
    let hndl = helper::handler_create("user32://icon");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
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
    let hndl = helper::handler_create("user32://cursor");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 2);
}
fn RegisterClassA(emu: &mut emu::Emu) {
    let wndclass = arg(emu, 0) as u64;
    let class_name = if wndclass != 0 {
        let name_ptr = emu.maps.read_dword(wndclass + 36).unwrap_or(0) as u64;
        if name_ptr != 0 {
            emu.maps.read_string(name_ptr)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    log_red!(emu, "user32!RegisterClassA class:'{}'", class_name);
    emu.regs_mut().rax = 0xC001;
    stdcall_ret(emu, 1);
}
fn RegisterClassW(emu: &mut emu::Emu) {
    let wndclass = arg(emu, 0) as u64;
    let class_name = if wndclass != 0 {
        let name_ptr = emu.maps.read_dword(wndclass + 36).unwrap_or(0) as u64;
        if name_ptr != 0 {
            emu.maps.read_wide_string(name_ptr)
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    log_red!(emu, "user32!RegisterClassW class:'{}'", class_name);
    emu.regs_mut().rax = 0xC001;
    stdcall_ret(emu, 1);
}
fn GetDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    log_red!(emu, "user32!GetDC hwnd: 0x{:x}", hwnd);
    let hndl = helper::handler_create("user32://dc");
    assert!(hndl < 0x00000001_00000000);
    emu.regs_mut().rax = hndl;
    stdcall_ret(emu, 1);
}
fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let hdc = arg(emu, 1) as u64;
    log_red!(emu, "user32!ReleaseDC hwnd: 0x{:x} hdc: 0x{:x}", hwnd, hdc);
    if helper::handler_exist(hdc) {
        helper::handler_close(hdc);
    }
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
    let lp_rect = arg(emu, 0) as u64;
    let dw_style = arg(emu, 1);
    let b_menu = arg(emu, 2);

    if lp_rect != 0 {
        let left = emu.maps.read_dword(lp_rect).unwrap_or(0) as i32;
        let top = emu.maps.read_dword(lp_rect + 4).unwrap_or(0) as i32;
        let right = emu.maps.read_dword(lp_rect + 8).unwrap_or(0) as i32;
        let bottom = emu.maps.read_dword(lp_rect + 12).unwrap_or(0) as i32;

        // Typical window frame: 8px border, 31px title bar
        let border = 8i32;
        let title = 31i32;
        let menu_h = if b_menu != 0 { 20i32 } else { 0i32 };

        emu.maps.write_dword(lp_rect, (left - border) as u32);
        emu.maps
            .write_dword(lp_rect + 4, (top - title - menu_h) as u32);
        emu.maps.write_dword(lp_rect + 8, (right + border) as u32);
        emu.maps.write_dword(lp_rect + 12, (bottom + border) as u32);

        log_red!(
            emu,
            "user32!AdjustWindowRect rect:({},{},{},{}) style:0x{:x} menu:{} => ({},{},{},{})",
            left,
            top,
            right,
            bottom,
            dw_style,
            b_menu,
            left - border,
            top - title - menu_h,
            right + border,
            bottom + border
        );
    } else {
        log_red!(
            emu,
            "user32!AdjustWindowRect rect:NULL style:0x{:x} menu:{}",
            dw_style,
            b_menu
        );
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn AdjustWindowRectEx(emu: &mut emu::Emu) {
    let lp_rect = arg(emu, 0) as u64;
    let dw_style = arg(emu, 1);
    let b_menu = arg(emu, 2);
    let dw_ex_style = arg(emu, 3);

    if lp_rect != 0 {
        let left = emu.maps.read_dword(lp_rect).unwrap_or(0) as i32;
        let top = emu.maps.read_dword(lp_rect + 4).unwrap_or(0) as i32;
        let right = emu.maps.read_dword(lp_rect + 8).unwrap_or(0) as i32;
        let bottom = emu.maps.read_dword(lp_rect + 12).unwrap_or(0) as i32;

        let border = 8i32;
        let title = 31i32;
        let menu_h = if b_menu != 0 { 20i32 } else { 0i32 };
        // WS_EX_CLIENTEDGE adds a 2px sunken edge
        let edge = if dw_ex_style & 0x200 != 0 { 2i32 } else { 0i32 };

        emu.maps.write_dword(lp_rect, (left - border - edge) as u32);
        emu.maps
            .write_dword(lp_rect + 4, (top - title - menu_h - edge) as u32);
        emu.maps
            .write_dword(lp_rect + 8, (right + border + edge) as u32);
        emu.maps
            .write_dword(lp_rect + 12, (bottom + border + edge) as u32);

        log_red!(
            emu,
            "user32!AdjustWindowRectEx rect:({},{},{},{}) style:0x{:x} menu:{} exstyle:0x{:x}",
            left,
            top,
            right,
            bottom,
            dw_style,
            b_menu,
            dw_ex_style
        );
    } else {
        log_red!(
            emu,
            "user32!AdjustWindowRectEx rect:NULL style:0x{:x} menu:{} exstyle:0x{:x}",
            dw_style,
            b_menu,
            dw_ex_style
        );
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

fn CreateWindowExA(emu: &mut emu::Emu) {
    let class = emu.maps.read_string(arg(emu, 1) as u64);
    let title = emu.maps.read_string(arg(emu, 2) as u64);
    let uri = format!("hwnd://{}", if title.is_empty() { &class } else { &title });
    let hwnd = helper::handler_create(&uri);
    log_red!(
        emu,
        "user32!CreateWindowExA class:'{}' title:'{}' => 0x{:x}",
        class,
        title,
        hwnd
    );
    emu.regs_mut().rax = hwnd;
    stdcall_ret(emu, 12);
}

fn CreateWindowExW(emu: &mut emu::Emu) {
    let class = emu.maps.read_wide_string(arg(emu, 1) as u64);
    let title = emu.maps.read_wide_string(arg(emu, 2) as u64);
    let uri = format!("hwnd://{}", if title.is_empty() { &class } else { &title });
    let hwnd = helper::handler_create(&uri);
    log_red!(
        emu,
        "user32!CreateWindowExW class:'{}' title:'{}' => 0x{:x}",
        class,
        title,
        hwnd
    );
    emu.regs_mut().rax = hwnd;
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
    let hwnd = arg(emu, 0);
    log_red!(emu, "user32!DestroyWindow hwnd:0x{:x}", hwnd);
    if helper::handler_exist(hwnd as u64) {
        helper::handler_close(hwnd as u64);
    }
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "user32!GetWindowRect arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
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
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    log_red!(
        emu,
        "user32!GetClientRect arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn SetWindowPos(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    let arg3 = emu.maps.read_dword(esp + 12).unwrap_or(0) as u64;
    let arg4 = emu.maps.read_dword(esp + 16).unwrap_or(0) as u64;
    let arg5 = emu.maps.read_dword(esp + 20).unwrap_or(0) as u64;
    let arg6 = emu.maps.read_dword(esp + 24).unwrap_or(0) as u64;
    let arg7 = emu.maps.read_dword(esp + 28).unwrap_or(0) as u64;
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "user32!SetWindowPos arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x} arg5:0x{:x} arg6:0x{:x} arg7:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4,
        arg5,
        arg6,
        arg7
    );
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
    let hwnd = arg(emu, 0) as u64;
    let buf = arg(emu, 1) as u64;
    let max_count = arg(emu, 2);
    let title = "Window";
    let len = title.len().min(max_count as usize - 1);
    if buf != 0 && max_count > 0 {
        emu.maps.write_string(buf, &title[..len]);
    }
    log_red!(
        emu,
        "user32!GetWindowTextA hwnd:0x{:x} buf:0x{:x} => '{}'",
        hwnd,
        buf,
        &title[..len]
    );
    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 3);
}

fn GetWindowTextW(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0) as u64;
    let buf = arg(emu, 1) as u64;
    let max_count = arg(emu, 2);
    let title = "Window";
    let len = title.len().min(max_count as usize - 1);
    if buf != 0 && max_count > 0 {
        emu.maps.write_wide_string(buf, &title[..len]);
    }
    log_red!(
        emu,
        "user32!GetWindowTextW hwnd:0x{:x} buf:0x{:x} => '{}'",
        hwnd,
        buf,
        &title[..len]
    );
    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 3);
}

fn GetWindowLongA(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let index = arg(emu, 1) as i32;
    log_red!(emu, "user32!GetWindowLongA hwnd:0x{:x} idx:{}", hwnd, index);
    let result: u64 = match index {
        -16 => 0x00CF_0000, // GWL_STYLE -> WS_OVERLAPPEDWINDOW
        -20 => 0,           // GWL_EXSTYLE
        -6 => 0x0040_0000,  // GWL_HINSTANCE
        -12 => 0,           // GWL_ID
        _ => 0,
    };
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 2);
}

fn GetWindowLongW(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let index = arg(emu, 1) as i32;
    log_red!(emu, "user32!GetWindowLongW hwnd:0x{:x} idx:{}", hwnd, index);
    let result: u64 = match index {
        -16 => 0x00CF_0000,
        -20 => 0,
        -6 => 0x0040_0000,
        -12 => 0,
        _ => 0,
    };
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 2);
}

fn SetWindowLongA(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let index = arg(emu, 1) as i32;
    let new_long = arg(emu, 2);
    log_red!(
        emu,
        "user32!SetWindowLongA hwnd:0x{:x} idx:{} val:0x{:x}",
        hwnd,
        index,
        new_long
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn SetWindowLongW(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let index = arg(emu, 1) as i32;
    let new_long = arg(emu, 2);
    log_red!(
        emu,
        "user32!SetWindowLongW hwnd:0x{:x} idx:{} val:0x{:x}",
        hwnd,
        index,
        new_long
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn PeekMessageA(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0) as u64;
    let hwnd = arg(emu, 1);
    let msg_filter_min = arg(emu, 2);
    let msg_filter_max = arg(emu, 3);
    let remove_msg = arg(emu, 4);

    // Zero-fill MSG struct (28 bytes: hwnd, message, wParam, lParam, time, pt.x, pt.y)
    if lp_msg != 0 {
        for i in 0..7u64 {
            emu.maps.write_dword(lp_msg + i * 4, 0);
        }
    }

    log_red!(
        emu,
        "user32!PeekMessageA msg:0x{:x} hwnd:0x{:x} filter:{}-{} remove:0x{:x}",
        lp_msg,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0; // no message available
    stdcall_ret(emu, 5);
}

fn PeekMessageW(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0) as u64;
    let hwnd = arg(emu, 1);
    let msg_filter_min = arg(emu, 2);
    let msg_filter_max = arg(emu, 3);
    let remove_msg = arg(emu, 4);

    // Zero-fill MSG struct (28 bytes)
    if lp_msg != 0 {
        for i in 0..7u64 {
            emu.maps.write_dword(lp_msg + i * 4, 0);
        }
    }

    log_red!(
        emu,
        "user32!PeekMessageW msg:0x{:x} hwnd:0x{:x} filter:{}-{} remove:0x{:x}",
        lp_msg,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0; // no message available
    stdcall_ret(emu, 5);
}

fn GetMessageA(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0) as u64;
    let hwnd = arg(emu, 1);
    let msg_filter_min = arg(emu, 2);
    let msg_filter_max = arg(emu, 3);

    // Fill MSG struct with WM_QUIT (0x0012) so the message loop terminates
    if lp_msg != 0 {
        emu.maps.write_dword(lp_msg, 0); // hwnd
        emu.maps.write_dword(lp_msg + 4, 0x0012); // message = WM_QUIT
        emu.maps.write_dword(lp_msg + 8, 0); // wParam
        emu.maps.write_dword(lp_msg + 12, 0); // lParam
        emu.maps.write_dword(lp_msg + 16, 0); // time
        emu.maps.write_dword(lp_msg + 20, 0); // pt.x
        emu.maps.write_dword(lp_msg + 24, 0); // pt.y
    }

    log_red!(
        emu,
        "user32!GetMessageA msg:0x{:x} hwnd:0x{:x} filter:{}-{} => WM_QUIT",
        lp_msg,
        hwnd,
        msg_filter_min,
        msg_filter_max
    );
    emu.regs_mut().rax = 0; // 0 = WM_QUIT received
    stdcall_ret(emu, 4);
}

fn GetMessageW(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0) as u64;
    let hwnd = arg(emu, 1);
    let msg_filter_min = arg(emu, 2);
    let msg_filter_max = arg(emu, 3);

    // Fill MSG struct with WM_QUIT (0x0012) so the message loop terminates
    if lp_msg != 0 {
        emu.maps.write_dword(lp_msg, 0); // hwnd
        emu.maps.write_dword(lp_msg + 4, 0x0012); // message = WM_QUIT
        emu.maps.write_dword(lp_msg + 8, 0); // wParam
        emu.maps.write_dword(lp_msg + 12, 0); // lParam
        emu.maps.write_dword(lp_msg + 16, 0); // time
        emu.maps.write_dword(lp_msg + 20, 0); // pt.x
        emu.maps.write_dword(lp_msg + 24, 0); // pt.y
    }

    log_red!(
        emu,
        "user32!GetMessageW msg:0x{:x} hwnd:0x{:x} filter:{}-{} => WM_QUIT",
        lp_msg,
        hwnd,
        msg_filter_min,
        msg_filter_max
    );
    emu.regs_mut().rax = 0; // 0 = WM_QUIT received
    stdcall_ret(emu, 4);
}

fn TranslateMessage(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0);
    log_red!(emu, "user32!TranslateMessage msg:0x{:x}", lp_msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn DispatchMessageA(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0);
    log_red!(emu, "user32!DispatchMessageA msg:0x{:x}", lp_msg);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn DispatchMessageW(emu: &mut emu::Emu) {
    let lp_msg = arg(emu, 0);
    log_red!(emu, "user32!DispatchMessageW msg:0x{:x}", lp_msg);
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
    let hndl = helper::handler_create("user32://focus");
    emu.regs_mut().rax = hndl;
}

fn GetForegroundWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetForegroundWindow");
    let hndl = helper::handler_create("user32://foreground");
    emu.regs_mut().rax = hndl;
}

fn SetForegroundWindow(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "user32!SetForegroundWindow arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn GetCursorPos(emu: &mut emu::Emu) {
    let pt = arg(emu, 0) as u64;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    log_red!(emu, "user32!GetCursorPos arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}

fn SetCursorPos(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(esp + 8).unwrap_or(0) as u64;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "user32!SetCursorPos arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn SetCursor(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "user32!SetCursor arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn ShowCursor(emu: &mut emu::Emu) {
    let esp = emu.regs().get_esp();
    let arg1 = emu.maps.read_dword(esp + 4).unwrap_or(0) as u64;
    for _ in 0..1u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "user32!ShowCursor arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 1);
}

fn LoadStringA(emu: &mut emu::Emu) {
    let _hinstance = arg(emu, 0);
    let uid = arg(emu, 1);
    let buffer = arg(emu, 2) as u64;
    let buf_max = arg(emu, 3);
    log_red!(emu, "user32!LoadStringA id:{} buf:0x{:x}", uid, buffer);
    let s = format!("String_{}", uid);
    if buffer != 0 && buf_max > 0 {
        let write_len = s.len().min((buf_max as usize).saturating_sub(1));
        emu.maps.write_string(buffer, &s[..write_len]);
        emu.regs_mut().rax = write_len as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 4);
}

fn LoadStringW(emu: &mut emu::Emu) {
    let _hinstance = arg(emu, 0);
    let uid = arg(emu, 1);
    let buffer = arg(emu, 2) as u64;
    let buf_max = arg(emu, 3);
    log_red!(emu, "user32!LoadStringW id:{} buf:0x{:x}", uid, buffer);
    let s = format!("String_{}", uid);
    if buffer != 0 && buf_max > 0 {
        let write_len = s.len().min((buf_max as usize).saturating_sub(1));
        emu.maps.write_wide_string(buffer, &s[..write_len]);
        emu.regs_mut().rax = write_len as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
    stdcall_ret(emu, 4);
}

fn GetClassInfoExA(emu: &mut emu::Emu) {
    let hinstance = arg(emu, 0);
    let lp_class = arg(emu, 1) as u64;
    let lp_wndclass = arg(emu, 2) as u64;
    let class_name = if lp_class != 0 && lp_class > 0xffff {
        emu.maps.read_string(lp_class)
    } else {
        format!("atom:0x{:x}", lp_class)
    };
    log_red!(
        emu,
        "user32!GetClassInfoExA hinst:0x{:x} class:'{}' out:0x{:x}",
        hinstance,
        class_name,
        lp_wndclass
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn GetClassInfoExW(emu: &mut emu::Emu) {
    let hinstance = arg(emu, 0);
    let lp_class = arg(emu, 1) as u64;
    let lp_wndclass = arg(emu, 2) as u64;
    let class_name = if lp_class != 0 && lp_class > 0xffff {
        emu.maps.read_wide_string(lp_class)
    } else {
        format!("atom:0x{:x}", lp_class)
    };
    log_red!(
        emu,
        "user32!GetClassInfoExW hinst:0x{:x} class:'{}' out:0x{:x}",
        hinstance,
        class_name,
        lp_wndclass
    );
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}

fn map_virtual_key_impl(code: u32, map_type: u32) -> u32 {
    match map_type {
        0 => match code {
            // MAPVK_VK_TO_VSC: virtual-key code to scan code
            0x08 => 0x0E,
            0x09 => 0x0F,
            0x0D => 0x1C,
            0x10 => 0x2A,
            0x11 => 0x1D,
            0x12 => 0x38,
            0x14 => 0x3A,
            0x1B => 0x01,
            0x20 => 0x39,
            0x2D => 0x52,
            0x2E => 0x53,
            0x30 => 0x0B,
            0x31 => 0x02,
            0x32 => 0x03,
            0x33 => 0x04,
            0x34 => 0x05,
            0x35 => 0x06,
            0x36 => 0x07,
            0x37 => 0x08,
            0x38 => 0x09,
            0x39 => 0x0A,
            0x41 => 0x1E,
            0x42 => 0x30,
            0x43 => 0x2E,
            0x44 => 0x20,
            0x45 => 0x12,
            0x46 => 0x21,
            0x47 => 0x22,
            0x48 => 0x23,
            0x49 => 0x17,
            0x4A => 0x24,
            0x4B => 0x25,
            0x4C => 0x26,
            0x4D => 0x32,
            0x4E => 0x31,
            0x4F => 0x18,
            0x50 => 0x19,
            0x51 => 0x10,
            0x52 => 0x13,
            0x53 => 0x1F,
            0x54 => 0x14,
            0x55 => 0x16,
            0x56 => 0x2F,
            0x57 => 0x11,
            0x58 => 0x2D,
            0x59 => 0x15,
            0x5A => 0x2C,
            c if (0x70..=0x7B).contains(&c) => 0x3B + (c - 0x70),
            _ => 0,
        },
        1 | 3 => match code {
            // MAPVK_VSC_TO_VK / MAPVK_VSC_TO_VK_EX: scan code to virtual-key
            0x01 => 0x1B,
            0x02 => 0x31,
            0x03 => 0x32,
            0x04 => 0x33,
            0x05 => 0x34,
            0x06 => 0x35,
            0x07 => 0x36,
            0x08 => 0x37,
            0x09 => 0x38,
            0x0A => 0x39,
            0x0B => 0x30,
            0x0E => 0x08,
            0x0F => 0x09,
            0x10 => 0x51,
            0x11 => 0x57,
            0x12 => 0x45,
            0x13 => 0x52,
            0x14 => 0x54,
            0x15 => 0x59,
            0x16 => 0x55,
            0x17 => 0x49,
            0x18 => 0x4F,
            0x19 => 0x50,
            0x1C => 0x0D,
            0x1D => 0x11,
            0x1E => 0x41,
            0x1F => 0x53,
            0x20 => 0x44,
            0x21 => 0x46,
            0x22 => 0x47,
            0x23 => 0x48,
            0x24 => 0x4A,
            0x25 => 0x4B,
            0x26 => 0x4C,
            0x2A => 0x10,
            0x2C => 0x5A,
            0x2D => 0x58,
            0x2E => 0x43,
            0x2F => 0x56,
            0x30 => 0x42,
            0x31 => 0x4E,
            0x32 => 0x4D,
            0x38 => 0x12,
            0x39 => 0x20,
            0x3A => 0x14,
            0x52 => 0x2D,
            0x53 => 0x2E,
            c if (0x3B..=0x46).contains(&c) => 0x70 + (c - 0x3B),
            _ => 0,
        },
        2 => {
            // MAPVK_VK_TO_CHAR: virtual-key to character
            if (0x41..=0x5A).contains(&code) {
                return code;
            }
            if (0x30..=0x39).contains(&code) {
                return code;
            }
            match code {
                0x20 => 0x20,
                0x0D => 0x0D,
                0x09 => 0x09,
                0x08 => 0x08,
                _ => 0,
            }
        }
        _ => 0,
    }
}

fn MapVirtualKeyA(emu: &mut emu::Emu) {
    let code = arg(emu, 0);
    let map_type = arg(emu, 1);
    let result = map_virtual_key_impl(code, map_type);
    log_red!(
        emu,
        "user32!MapVirtualKeyA code:0x{:x} type:{} = 0x{:x}",
        code,
        map_type,
        result
    );
    emu.regs_mut().rax = result as u64;
    stdcall_ret(emu, 2);
}

fn MapVirtualKeyW(emu: &mut emu::Emu) {
    let code = arg(emu, 0);
    let map_type = arg(emu, 1);
    let result = map_virtual_key_impl(code, map_type);
    log_red!(
        emu,
        "user32!MapVirtualKeyW code:0x{:x} type:{} = 0x{:x}",
        code,
        map_type,
        result
    );
    emu.regs_mut().rax = result as u64;
    stdcall_ret(emu, 2);
}

fn GetKeyboardLayout(emu: &mut emu::Emu) {
    let thread_id = arg(emu, 0);
    log_red!(emu, "user32!GetKeyboardLayout thread:{}", thread_id);
    emu.regs_mut().rax = 0x0409_0409; // US English keyboard layout
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
    let hwnd = arg(emu, 0);
    let lp_paint = arg(emu, 1) as u64;
    let hdc: u32 = 0x1337_0002;

    // Fill PAINTSTRUCT (64 bytes):
    //   +0  HDC  hdc
    //   +4  BOOL fErase
    //   +8  RECT rcPaint (left, top, right, bottom)
    //  +24  BOOL fRestore (reserved)
    //  +28  BOOL fIncUpdate (reserved)
    //  +32  BYTE rgbReserved[32]
    if lp_paint != 0 {
        emu.maps.write_dword(lp_paint, hdc); // hdc
        emu.maps.write_dword(lp_paint + 4, 1); // fErase = TRUE
        emu.maps.write_dword(lp_paint + 8, 0); // rcPaint.left
        emu.maps.write_dword(lp_paint + 12, 0); // rcPaint.top
        emu.maps.write_dword(lp_paint + 16, 1920); // rcPaint.right
        emu.maps.write_dword(lp_paint + 20, 1080); // rcPaint.bottom
        emu.maps.write_dword(lp_paint + 24, 0); // fRestore
        emu.maps.write_dword(lp_paint + 28, 0); // fIncUpdate
        // Zero rgbReserved[32]
        for i in 0..8u64 {
            emu.maps.write_dword(lp_paint + 32 + i * 4, 0);
        }
    }

    log_red!(
        emu,
        "user32!BeginPaint hwnd:0x{:x} ps:0x{:x} => hdc:0x{:x}",
        hwnd,
        lp_paint,
        hdc
    );
    emu.regs_mut().rax = hdc as u64;
    stdcall_ret(emu, 2);
}

fn EndPaint(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let lp_paint = arg(emu, 1);
    log_red!(emu, "user32!EndPaint hwnd:0x{:x} ps:0x{:x}", hwnd, lp_paint);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}

fn InvalidateRect(emu: &mut emu::Emu) {
    let hwnd = arg(emu, 0);
    let lp_rect = arg(emu, 1) as u64;
    let b_erase = arg(emu, 2);

    if lp_rect != 0 {
        let left = emu.maps.read_dword(lp_rect).unwrap_or(0) as i32;
        let top = emu.maps.read_dword(lp_rect + 4).unwrap_or(0) as i32;
        let right = emu.maps.read_dword(lp_rect + 8).unwrap_or(0) as i32;
        let bottom = emu.maps.read_dword(lp_rect + 12).unwrap_or(0) as i32;
        log_red!(
            emu,
            "user32!InvalidateRect hwnd:0x{:x} rect:({},{},{},{}) erase:{}",
            hwnd,
            left,
            top,
            right,
            bottom,
            b_erase
        );
    } else {
        log_red!(
            emu,
            "user32!InvalidateRect hwnd:0x{:x} rect:NULL erase:{}",
            hwnd,
            b_erase
        );
    }

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}

fn FillRect(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let lp_rect = arg(emu, 1) as u64;
    let hbr = arg(emu, 2);

    if lp_rect != 0 {
        let left = emu.maps.read_dword(lp_rect).unwrap_or(0) as i32;
        let top = emu.maps.read_dword(lp_rect + 4).unwrap_or(0) as i32;
        let right = emu.maps.read_dword(lp_rect + 8).unwrap_or(0) as i32;
        let bottom = emu.maps.read_dword(lp_rect + 12).unwrap_or(0) as i32;
        log_red!(
            emu,
            "user32!FillRect hdc:0x{:x} rect:({},{},{},{}) hbr:0x{:x}",
            hdc,
            left,
            top,
            right,
            bottom,
            hbr
        );
    } else {
        log_red!(
            emu,
            "user32!FillRect hdc:0x{:x} rect:NULL hbr:0x{:x}",
            hdc,
            hbr
        );
    }

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
