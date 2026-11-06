use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi64;
use crate::winapi::window_table;
use crate::windows::constants;

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "MessageBoxA" => MessageBoxA(emu),
        "MessageBoxW" => MessageBoxW(emu),
        "GetDesktopWindow" => GetDesktopWindow(emu),
        "GetSystemMetrics" => GetSystemMetrics(emu),
        "SystemParametersInfoA" => SystemParametersInfoA(emu),
        "LoadIconA" => LoadIconA(emu),
        "LoadCursorA" => LoadCursorA(emu),
        "RegisterClassA" => RegisterClassA(emu),
        "RegisterClassW" => RegisterClassW(emu),
        "RegisterClassExA" => RegisterClassExA(emu),
        "RegisterClassExW" => RegisterClassExW(emu),
        "UnregisterClassA" => UnregisterClassA(emu),
        "UnregisterClassW" => UnregisterClassW(emu),
        "EnumWindows" => EnumWindows(emu),
        "GetProcessWindowStation" => GetProcessWindowStation(emu),
        "GetUserObjectInformationW" => GetUserObjectInformationW(emu),
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "CharLowerBuffW" => CharLowerBuffW(emu),
        "CharLowerBuffA" => CharLowerBuffA(emu),
        "CharUpperBuffW" => CharUpperBuffW(emu),
        "CharUpperBuffA" => CharUpperBuffA(emu),
        "FindWindowA" => FindWindowA(emu),
        "FindWindowW" => FindWindowW(emu),
        "FindWindowExA" => FindWindowExA(emu),
        "FindWindowExW" => FindWindowExW(emu),
        "GetKeyState" => GetKeyState(emu),
        "GetAsyncKeyState" => GetAsyncKeyState(emu),
        "SetWindowsHookExA" => SetWindowsHookExA(emu),
        "wsprintfA" => wsprintfA(emu),
        "wsprintfW" => wsprintfW(emu),
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
        "GetWindowLongPtrA" => GetWindowLongPtrA(emu),
        "GetWindowLongPtrW" => GetWindowLongPtrW(emu),
        "SetWindowLongPtrA" => SetWindowLongPtrA(emu),
        "SetWindowLongPtrW" => SetWindowLongPtrW(emu),
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
        "GetPhysicalCursorPos" => GetPhysicalCursorPos(emu),
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
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ActivateKeyboardLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("ActivateKeyboardLayout");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddClipboardFormatListener" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AddClipboardFormatListener arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddClipboardFormatListener");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AddVisualIdentifier" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AddVisualIdentifier arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("AddVisualIdentifier");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AdjustWindowRectExForDpi" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AdjustWindowRectExForDpi arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AlignRects" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!AlignRects arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AllowForegroundActivation" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AllowForegroundActivation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "AllowSetForegroundWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AllowSetForegroundWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AnimateWindow" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!AnimateWindow p1:0x{:x} p2:0x{:x}", p1, p2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AnyPopup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!AnyPopup arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppendMenuA" => {
            let hmenu = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let id_new_item = emu.regs().r8;
            let new_item_ptr = emu.regs().r9;
            let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
                emu.maps.read_string(new_item_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "user32!AppendMenuA menu:0x{:x} flags:0x{:x} id:{} '{}'",
                hmenu,
                flags,
                id_new_item,
                text
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AppendMenuW" => {
            let hmenu = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let id_new_item = emu.regs().r8;
            let new_item_ptr = emu.regs().r9;
            let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
                emu.maps.read_wide_string(new_item_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "user32!AppendMenuW menu:0x{:x} flags:0x{:x} id:{} '{}'",
                hmenu,
                flags,
                id_new_item,
                text
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ApplyWindowAction" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ApplyWindowAction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AreDpiAwarenessContextsEqual" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AreDpiAwarenessContextsEqual arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "ArrangeIconicWindows" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ArrangeIconicWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "AttachThreadInput" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!AttachThreadInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("AttachThreadInput");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BeginDeferWindowPos" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!BeginDeferWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("BeginDeferWindowPos");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BlockInput" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!BlockInput 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "BringWindowToTop" => {
            let p0 = emu.regs().rcx;
            log_red!(emu, "user32!BringWindowToTop");
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!BroadcastSystemMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!BroadcastSystemMessageA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BroadcastSystemMessageW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!BroadcastSystemMessageW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "BuildReasonArray" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!BuildReasonArray arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CalcMenuBar" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CalcMenuBar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CalculatePopupWindowPosition" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CalculatePopupWindowPosition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallMsgFilter" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CallMsgFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallMsgFilterA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CallMsgFilterA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallMsgFilterW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CallMsgFilterW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallNextHookEx" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!CallNextHookEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CallWindowProcA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
        }
        "CallWindowProcW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
        }
        "CancelShutdown" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CancelShutdown h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CascadeChildWindows" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CascadeChildWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CascadeWindows" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CascadeWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeClipboardChain" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ChangeClipboardChain arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeDisplaySettingsA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeDisplaySettingsA '{}' p1:0x{:x}", s, p1);
        }
        "ChangeDisplaySettingsExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
        }
        "ChangeDisplaySettingsExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
        }
        "ChangeDisplaySettingsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeDisplaySettingsW '{}' p1:0x{:x}", s, p1);
        }
        "ChangeMenuA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeMenuA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeMenuW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ChangeMenuW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeWindowMessageFilter" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ChangeWindowMessageFilter arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChangeWindowMessageFilterEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ChangeWindowMessageFilterEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CharLowerA" => {
            let ptr = emu.regs().rcx;
            if ptr < 0x10000 {
                // Single character in low word
                let ch = (ptr & 0xFF) as u8;
                let lower = (ch as char).to_ascii_lowercase() as u64;
                log_red!(
                    emu,
                    "user32!CharLowerA char: '{}' -> '{}'",
                    ch as char,
                    lower as u8 as char
                );
                emu.regs_mut().rax = lower;
            } else {
                // Pointer to string — lowercase in-place
                let s = emu.maps.read_string(ptr);
                log_red!(emu, "user32!CharLowerA str: '{}'", s);
                let lower = s.to_ascii_lowercase();
                emu.maps.write_string(ptr, &lower);
                emu.regs_mut().rax = ptr;
            }
        }
        "CharLowerW" => {
            let ptr = emu.regs().rcx;
            if ptr < 0x10000 {
                let ch = (ptr & 0xFFFF) as u16;
                let lower = if (0x41..=0x5A).contains(&ch) {
                    ch + 0x20
                } else {
                    ch
                };
                log_red!(emu, "user32!CharLowerW char: 0x{:x}", ch);
                emu.regs_mut().rax = lower as u64;
            } else {
                let s = emu.maps.read_wide_string(ptr);
                emu.maps.write_wide_string(ptr, &s.to_lowercase());
                log_red!(emu, "user32!CharLowerW '{}'", s);
                emu.regs_mut().rax = ptr;
            }
        }
        "CharNextA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharNextA '{}'", s);
            if p0 != 0 {
                let ch = emu.maps.read_byte(p0).unwrap_or(0);
                if ch != 0 {
                    emu.set_last_error(0);
                    emu.regs_mut().rax = p0 + 1;
                } else {
                    emu.regs_mut().rax = p0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CharNextExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharNextExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CharNextW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharNextW '{}'", s);
            if p0 != 0 {
                let ch = emu.maps.read_word(p0).unwrap_or(0);
                if ch != 0 {
                    emu.set_last_error(0);
                    emu.regs_mut().rax = p0 + 2;
                } else {
                    emu.regs_mut().rax = p0;
                }
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CharPrevA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharPrevA '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
        }
        "CharPrevExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharPrevExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CharPrevW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!CharPrevW '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
        }
        "CharToOemA" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "user32!CharToOemA src: 0x{:x} dst: 0x{:x}", src, dst);
            if src != 0 && dst != 0 {
                let s = emu.maps.read_string(src);
                emu.maps.write_string(dst, &s);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CharToOemBuffA" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            let len = emu.regs().r8 as u32;
            log_red!(
                emu,
                "user32!CharToOemBuffA src: 0x{:x} dst: 0x{:x} len: {}",
                src,
                dst,
                len
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            if src != 0 && dst != 0 {
                for i in 0..len as u64 {
                    let b = emu.maps.read_byte(src + i).unwrap_or(0);
                    emu.maps.write_byte(dst + i, b);
                }
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CharToOemBuffW" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            let len = emu.regs().r8 as u32;
            log_red!(
                emu,
                "user32!CharToOemBuffW src: 0x{:x} dst: 0x{:x} len: {}",
                src,
                dst,
                len
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            if src != 0 && dst != 0 {
                for i in 0..len as u64 {
                    let ch = emu.maps.read_word(src + i * 2).unwrap_or(0);
                    emu.maps.write_byte(dst + i, (ch & 0xFF) as u8);
                }
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CharToOemW" => {
            let src = emu.regs().rcx;
            let dst = emu.regs().rdx;
            log_red!(emu, "user32!CharToOemW src: 0x{:x} dst: 0x{:x}", src, dst);
            if src != 0 && dst != 0 {
                let s = emu.maps.read_wide_string(src);
                emu.maps.write_string(dst, &s);
            }
            emu.regs_mut().rax = 1; // TRUE
        }
        "CharUpperA" => {
            let ptr = emu.regs().rcx;
            if ptr < 0x10000 {
                let ch = (ptr & 0xFF) as u8;
                let upper = (ch as char).to_ascii_uppercase() as u64;
                log_red!(
                    emu,
                    "user32!CharUpperA char: '{}' -> '{}'",
                    ch as char,
                    upper as u8 as char
                );
                emu.regs_mut().rax = upper;
            } else {
                let s = emu.maps.read_string(ptr);
                log_red!(emu, "user32!CharUpperA str: '{}'", s);
                let upper = s.to_ascii_uppercase();
                emu.maps.write_string(ptr, &upper);
                emu.regs_mut().rax = ptr;
            }
        }
        "CharUpperW" => {
            let ptr = emu.regs().rcx;
            if ptr < 0x10000 {
                let ch = (ptr & 0xFFFF) as u16;
                if let Some(c) = char::from_u32(ch as u32) {
                    let upper = c.to_uppercase().next().unwrap_or(c) as u64;
                    log_red!(emu, "user32!CharUpperW char: '{}' -> upper", c);
                    emu.regs_mut().rax = upper;
                } else {
                    emu.set_last_error(0);
                    emu.regs_mut().rax = ptr;
                }
            } else {
                let s = emu.maps.read_wide_string(ptr);
                log_red!(emu, "user32!CharUpperW str: '{}'", s);
                let upper: String = s
                    .chars()
                    .map(|c| c.to_uppercase().next().unwrap_or(c))
                    .collect();
                emu.maps.write_wide_string(ptr, &upper);
                emu.regs_mut().rax = ptr;
            }
        }
        "CheckBannedOneCoreTransformApi" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CheckBannedOneCoreTransformApi arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckDBCSEnabledExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CheckDBCSEnabledExt arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CheckDBCSEnabledExt");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CheckDlgButton" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!CheckDlgButton p1:0x{:x} p2:0x{:x}", p1, p2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckMenuItem" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!CheckMenuItem p1:0x{:x} p2:0x{:x}", p1, p2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckMenuRadioItem" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CheckMenuRadioItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckProcessForClipboardAccess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CheckProcessForClipboardAccess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckProcessSession" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CheckProcessSession h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckRadioButton" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!CheckRadioButton p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "CheckWindowThreadDesktop" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CheckWindowThreadDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "ChildWindowFromPoint" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(
                emu,
                "user32!ChildWindowFromPoint p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ChildWindowFromPointEx" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!ChildWindowFromPointEx p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CliImmSetHotKey" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CliImmSetHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClientThreadSetup" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ClientThreadSetup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClientToScreen" => {
            let hwnd = emu.regs().rcx;
            let pt = emu.regs().rdx;
            log_red!(emu, "user32!ClientToScreen hwnd:0x{:x} pt:0x{:x}", hwnd, pt);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ClipCursor" => {
            let p0 = emu.regs().rcx;
            log_red!(emu, "user32!ClipCursor");
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CloseClipboard" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseClipboard handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CloseDesktop" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseDesktop handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CloseGestureInfoHandle" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseGestureInfoHandle handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CloseTouchInputHandle" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseTouchInputHandle handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CloseWindow" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseWindow handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "CloseWindowStation" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!CloseWindowStation handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "ConsoleControl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ConsoleControl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ControlMagnification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ControlMagnification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertPrimaryPointerToMouseDrag" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ConvertPrimaryPointerToMouseDrag arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "ConvertToInterceptWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ConvertToInterceptWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyAcceleratorTableA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "CopyAcceleratorTableW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "CopyIcon" => {
            let hicon = emu.regs().rcx;
            log_red!(emu, "user32!CopyIcon hIcon:0x{:x}", hicon);
            let hndl = helper::handler_create("user32://icon");
            emu.regs_mut().rax = hndl;
        }
        "CopyImage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!CopyImage arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("CopyImage");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CopyRect" => {
            let dst = emu.regs().rcx;
            let src = emu.regs().rdx;
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
            emu.regs_mut().rax = 1;
        }
        "CountClipboardFormats" => {
            log_red!(emu, "user32!CountClipboardFormats");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "CreateAcceleratorTableA" => {
            let accel_ptr = emu.regs().rcx;
            let num_entries = emu.regs().rdx as u32;
            log_red!(
                emu,
                "user32!CreateAcceleratorTableA accel:0x{:x} count:{}",
                accel_ptr,
                num_entries
            );
            let hndl = helper::handler_create("user32://CreateAcceleratorTableA");
            emu.regs_mut().rax = hndl;
        }
        "CreateAcceleratorTableW" => {
            let accel_ptr = emu.regs().rcx;
            let num_entries = emu.regs().rdx as u32;
            log_red!(
                emu,
                "user32!CreateAcceleratorTableW accel:0x{:x} count:{}",
                accel_ptr,
                num_entries
            );
            let hndl = helper::handler_create("user32://CreateAcceleratorTableW");
            emu.regs_mut().rax = hndl;
        }
        "CreateCaret" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateCaret");
            log_red!(emu, "user32!CreateCaret => 0x{:x}", hndl);
            emu.regs_mut().rax = hndl;
        }
        "CreateCursor" => {
            let hinstance = emu.regs().rcx;
            let x_hotspot = emu.regs().rdx as i32;
            let y_hotspot = emu.regs().r8 as i32;
            let width = emu.regs().r9 as i32;
            let height = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
            let and_plane = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let xor_plane = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateCursor hinst:0x{:x} hot:({},{}) size:{}x{}",
                hinstance,
                x_hotspot,
                y_hotspot,
                width,
                height
            );
            let hndl = helper::handler_create("user32://CreateCursor");
            emu.regs_mut().rax = hndl;
        }
        "CreateDCompositionHwndTarget" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateDCompositionHwndTarget");
            log_red!(emu, "user32!CreateDCompositionHwndTarget => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateDesktopA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDesktopExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDesktopExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDesktopW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDialogIndirectParamA" => {
            let hinstance = emu.regs().rcx;
            let template_ptr = emu.regs().rdx;
            let hwnd_parent = emu.regs().r8;
            let dlg_proc = emu.regs().r9;
            let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamA hinst:0x{:x} parent:0x{:x} proc:0x{:x}",
                hinstance,
                hwnd_parent,
                dlg_proc
            );
            let hndl = helper::handler_create("user32://CreateDialogIndirectParamA");
            emu.regs_mut().rax = hndl;
        }
        "CreateDialogIndirectParamAorW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CreateDialogIndirectParamW" => {
            let hinstance = emu.regs().rcx;
            let template_ptr = emu.regs().rdx;
            let hwnd_parent = emu.regs().r8;
            let dlg_proc = emu.regs().r9;
            let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateDialogIndirectParamW hinst:0x{:x} parent:0x{:x} proc:0x{:x}",
                hinstance,
                hwnd_parent,
                dlg_proc
            );
            let hndl = helper::handler_create("user32://CreateDialogIndirectParamW");
            emu.regs_mut().rax = hndl;
        }
        "CreateDialogParamA" => {
            let hinstance = emu.regs().rcx;
            let template_name = emu.regs().rdx;
            let hwnd_parent = emu.regs().r8;
            let dlg_proc = emu.regs().r9;
            let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let name = if template_name > 0xffff {
                emu.maps.read_string(template_name)
            } else {
                format!("#{}", template_name)
            };
            log_red!(
                emu,
                "user32!CreateDialogParamA hinst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
                hinstance,
                name,
                hwnd_parent,
                dlg_proc
            );
            let uri = format!("user32://CreateDialogParamA/{}", name);
            let hndl = helper::handler_create(&uri);
            emu.regs_mut().rax = hndl;
        }
        "CreateDialogParamW" => {
            let hinstance = emu.regs().rcx;
            let template_name = emu.regs().rdx;
            let hwnd_parent = emu.regs().r8;
            let dlg_proc = emu.regs().r9;
            let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let name = if template_name > 0xffff {
                emu.maps.read_wide_string(template_name)
            } else {
                format!("#{}", template_name)
            };
            log_red!(
                emu,
                "user32!CreateDialogParamW hinst:0x{:x} tmpl:'{}' parent:0x{:x} proc:0x{:x}",
                hinstance,
                name,
                hwnd_parent,
                dlg_proc
            );
            let uri = format!("user32://CreateDialogParamW/{}", name);
            let hndl = helper::handler_create(&uri);
            emu.regs_mut().rax = hndl;
        }
        "CreateIcon" => {
            let hinstance = emu.regs().rcx;
            let width = emu.regs().rdx as i32;
            let height = emu.regs().r8 as i32;
            let planes = emu.regs().r9 as u8;
            let bits_pixel = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u8;
            let and_bits = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let xor_bits = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "user32!CreateIcon hinst:0x{:x} size:{}x{} planes:{} bpp:{}",
                hinstance,
                width,
                height,
                planes,
                bits_pixel
            );
            let hndl = helper::handler_create("user32://CreateIcon");
            emu.regs_mut().rax = hndl;
        }
        "CreateIconFromResource" => {
            let presbits = emu.regs().rcx;
            let dw_res_size = emu.regs().rdx as u32;
            let is_icon = emu.regs().r8 as u32;
            let dw_ver = emu.regs().r9 as u32;
            log_red!(
                emu,
                "user32!CreateIconFromResource data:0x{:x} size:{} icon:{} ver:0x{:x}",
                presbits,
                dw_res_size,
                is_icon,
                dw_ver
            );
            let hndl = helper::handler_create("user32://CreateIconFromResource");
            emu.regs_mut().rax = hndl;
        }
        "CreateIconFromResourceEx" => {
            let presbits = emu.regs().rcx;
            let dw_res_size = emu.regs().rdx as u32;
            let is_icon = emu.regs().r8 as u32;
            let dw_ver = emu.regs().r9 as u32;
            let cx = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
            let cy = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as i32;
            let flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0) as u32;
            log_red!(
                emu,
                "user32!CreateIconFromResourceEx data:0x{:x} size:{} icon:{} cx:{} cy:{} flags:0x{:x}",
                presbits,
                dw_res_size,
                is_icon,
                cx,
                cy,
                flags
            );
            let hndl = helper::handler_create("user32://CreateIconFromResourceEx");
            emu.regs_mut().rax = hndl;
        }
        "CreateIconIndirect" => {
            let iconinfo_ptr = emu.regs().rcx;
            log_red!(emu, "user32!CreateIconIndirect info:0x{:x}", iconinfo_ptr);
            let hndl = helper::handler_create("user32://CreateIconIndirect");
            emu.regs_mut().rax = hndl;
        }
        "CreateMDIWindowA" => {
            let ex_style = emu.regs().rcx;
            let class_ptr = emu.regs().rdx;
            let title_ptr = emu.regs().r8;
            let style = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateMDIWindowW" => {
            let ex_style = emu.regs().rcx;
            let class_ptr = emu.regs().rdx;
            let title_ptr = emu.regs().r8;
            let style = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
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
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateSyntheticPointerDevice");
            log_red!(emu, "user32!CreateSyntheticPointerDevice => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateSyntheticPointerDevice2" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateSyntheticPointerDevice2");
            log_red!(emu, "user32!CreateSyntheticPointerDevice2 => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateSystemThreads" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateSystemThreads");
            log_red!(emu, "user32!CreateSystemThreads => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateWindowInBand" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateWindowInBand");
            log_red!(emu, "user32!CreateWindowInBand => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateWindowInBandEx" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateWindowInBandEx");
            log_red!(emu, "user32!CreateWindowInBandEx => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateWindowIndirect" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("CreateWindowIndirect");
            log_red!(emu, "user32!CreateWindowIndirect => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateWindowStationA" => {
            let ex_style = emu.regs().rcx;
            let class_ptr = emu.regs().rdx;
            let title_ptr = emu.regs().r8;
            let style = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CreateWindowStationW" => {
            let ex_style = emu.regs().rcx;
            let class_ptr = emu.regs().rdx;
            let title_ptr = emu.regs().r8;
            let style = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "CsrBroadcastSystemMessageExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "CtxInitUser32" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!CtxInitUser32 arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("CtxInitUser32");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeAbandonTransaction" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeAbandonTransaction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeAccessData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeAccessData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeAddData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!DdeAddData arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("DdeAddData");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeClientTransaction" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeClientTransaction arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCmpStringHandles" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeCmpStringHandles h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeConnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!DdeConnect arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let handle = helper::handler_create("DdeConnect");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeConnectList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeConnectList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("DdeConnectList");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCreateDataHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeCreateDataHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("DdeCreateDataHandle");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCreateStringHandleA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeCreateStringHandleA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeCreateStringHandleW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeCreateStringHandleW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeDisconnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeDisconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeDisconnectList" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeDisconnectList arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeEnableCallback" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeEnableCallback arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeFreeDataHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeFreeDataHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeFreeStringHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeFreeStringHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeGetData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!DdeGetData arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeGetLastError" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeGetLastError arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeGetQualityOfService" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeGetQualityOfService arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeImpersonateClient" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeImpersonateClient arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeInitializeA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeInitializeA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeInitializeW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeInitializeW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeKeepStringHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeKeepStringHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeNameService" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeNameService arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdePostAdvise" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdePostAdvise arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryConvInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeQueryConvInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryNextServer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeQueryNextServer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryStringA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeQueryStringA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeQueryStringW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DdeQueryStringW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeReconnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeReconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeSetQualityOfService" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeSetQualityOfService arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeSetUserHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeSetUserHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeUnaccessData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeUnaccessData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DdeUninitialize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DdeUninitialize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefDlgProcA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefDlgProcA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefDlgProcW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefDlgProcW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefFrameProcA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefFrameProcA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefFrameProcW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefFrameProcW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefMDIChildProcA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefMDIChildProcA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefMDIChildProcW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DefMDIChildProcW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DefRawInputProc" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DefRawInputProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeferWindowPos" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DeferWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeferWindowPosAndBand" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DeferWindowPosAndBand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DelegateInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DeleteMenu" => {
            let hmenu = emu.regs().rcx;
            let u_position = emu.regs().rdx as u32;
            let u_flags = emu.regs().r8 as u32;
            log_red!(
                emu,
                "user32!DeleteMenu menu:0x{:x} pos:{} flags:0x{:x}",
                hmenu,
                u_position,
                u_flags
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.regs_mut().rax = 1;
        }
        "DeregisterShellHookWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DeregisterShellHookWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DestroyAcceleratorTable" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyAcceleratorTable handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DestroyCaret" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyCaret handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DestroyCursor" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyCursor handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DestroyDCompositionHwndTarget" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(
                emu,
                "user32!DestroyDCompositionHwndTarget handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
        }
        "DestroyIcon" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyIcon handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DestroyMenu" => {
            let hmenu = emu.regs().rcx;
            log_red!(emu, "user32!DestroyMenu hmenu:0x{:x}", hmenu);
            if hmenu != 0 && helper::handler_exist(hmenu) {
                helper::handler_close(hmenu);
            }
            emu.regs_mut().rax = 1;
        }
        "DestroyReasons" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!DestroyReasons handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "DestroySyntheticPointerDevice" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(
                emu,
                "user32!DestroySyntheticPointerDevice handle:0x{:x}",
                handle
            );
            emu.regs_mut().rax = 1;
        }
        "DialogBoxIndirectParamA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
        }
        "DialogBoxIndirectParamAorW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DialogBoxIndirectParamW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
        }
        "DialogBoxParamA" => {
            let hinstance = emu.regs().rcx;
            let template_ptr = emu.regs().rdx;
            let hwnd_parent = emu.regs().r8;
            let dlg_proc = emu.regs().r9;
            let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let template_name = if template_ptr > 0xffff {
                emu.maps.read_string(template_ptr)
            } else {
                format!("#{}", template_ptr)
            };
            log_red!(
                emu,
                "user32!DialogBoxParamA '{}' proc:0x{:x} param:0x{:x}",
                template_name,
                dlg_proc,
                init_param
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // IDOK
        }
        "DialogBoxParamW" => {
            let hinstance = emu.regs().rcx;
            let template_ptr = emu.regs().rdx;
            let hwnd_parent = emu.regs().r8;
            let dlg_proc = emu.regs().r9;
            let init_param = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let template_name = if template_ptr > 0xffff {
                emu.maps.read_wide_string(template_ptr)
            } else {
                format!("#{}", template_ptr)
            };
            log_red!(
                emu,
                "user32!DialogBoxParamW '{}' proc:0x{:x} param:0x{:x}",
                template_name,
                dlg_proc,
                init_param
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // IDOK
        }
        "DisableProcessWindowsGhosting" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DisableProcessWindowsGhosting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisplayConfigGetDeviceInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DisplayConfigGetDeviceInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisplayConfigSetDeviceInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DisplayConfigSetDeviceInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DisplayExitWindowsWarnings" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DisplayExitWindowsWarnings arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirListA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirListComboBoxA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListComboBoxA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirListComboBoxW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListComboBoxW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirListW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirListW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectComboBoxExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectComboBoxExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectComboBoxExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectComboBoxExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DlgDirSelectExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DlgDirSelectExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DoSoundConnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DoSoundConnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("DoSoundConnect");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DoSoundDisconnect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DoSoundDisconnect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DragDetect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!DragDetect arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DragObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!DragObject h:0x{:x} arg2:0x{:x}", handle, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawAnimatedRects" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DrawAnimatedRects arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawCaption" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DrawCaption arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawCaptionTempA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DrawCaptionTempA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawCaptionTempW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!DrawCaptionTempW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawEdge" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!DrawEdge p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawFocusRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!DrawFocusRect p1:0x{:x}", p1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawFrame" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!DrawFrame arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawFrameControl" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!DrawFrameControl p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawIcon" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!DrawIcon p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawIconEx" => {
            let hdc = emu.regs().rcx;
            let x_left = emu.regs().rdx as i32;
            let y_top = emu.regs().r8 as i32;
            let h_icon = emu.regs().r9;
            let cx_width = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i32;
            let cy_width = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
            let _ist_step_if_ani_cur = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let _hbr_flicker_free_draw = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let _di_flags = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
            log_red!(
                emu,
                "user32!DrawIconEx hdc:0x{:x} x:{} y:{} icon:0x{:x} cx:{} cy:{}",
                hdc,
                x_left,
                y_top,
                h_icon,
                cx_width,
                cy_width
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawMenuBar" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!DrawMenuBar hwnd:0x{:x}", hwnd);
            let result = if helper::handler_exist(hwnd) {
                1u64
            } else {
                0u64
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = result;
        }
        "DrawMenuBarTemp" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DrawMenuBarTemp arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawStateA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p6 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let p7 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawStateW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p6 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let p7 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DrawTextExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
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
        }
        "DrawTextExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
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
        }
        "DwmGetDxRgn" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmGetDxRgn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmGetDxSharedSurface" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmGetDxSharedSurface arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmGetRemoteSessionOcclusionEvent" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!DwmGetRemoteSessionOcclusionEvent 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmGetRemoteSessionOcclusionState" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmGetRemoteSessionOcclusionState arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmKernelShutdown" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmKernelShutdown arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmKernelStartup" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmKernelStartup arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("DwmKernelStartup");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmLockScreenUpdates" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!DwmLockScreenUpdates 0x{:x}", arg1);
            let handle = helper::handler_create("DwmLockScreenUpdates");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "DwmValidateWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmValidateWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "DwmWindowNotificationsEnabled" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!DwmWindowNotificationsEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("DwmWindowNotificationsEnabled");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EditWndProc" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EditWndProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
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
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!EnableMenuItem p1:0x{:x} p2:0x{:x}", p1, p2);
            let handle = helper::handler_create("EnableMenuItem");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableMouseInPointer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnableMouseInPointer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableMouseInPointer");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableNonClientDpiScaling" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnableNonClientDpiScaling arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableNonClientDpiScaling");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableOneCoreTransformMode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnableOneCoreTransformMode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableOneCoreTransformMode");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableScrollBar" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!EnableScrollBar p1:0x{:x} p2:0x{:x}", p1, p2);
            let handle = helper::handler_create("EnableScrollBar");
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableSessionForMMCSS" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnableSessionForMMCSS arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableSessionForMMCSS");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnableWindowShellWindowManagementBehavior" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnableWindowShellWindowManagementBehavior arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnableWindowShellWindowManagementBehavior");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndDeferWindowPos" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EndDeferWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndDeferWindowPosEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EndDeferWindowPosEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndDialog" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!EndDialog p1:0x{:x}", p1);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndMenu" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!EndMenu arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EndTask" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!EndTask arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnterMoveSizeLoop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnterMoveSizeLoop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnterMoveSizeLoop");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnterReaderModeHelper" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnterReaderModeHelper arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("EnterReaderModeHelper");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumChildWindows" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!EnumChildWindows p1:0x{:x} p2:0x{:x}", p1, p2);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumClipboardFormats" => {
            let format = emu.regs().rcx;
            log_red!(emu, "user32!EnumClipboardFormats format:{}", format);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "EnumDesktopWindows" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EnumDesktopWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDesktopsA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDesktopsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplayDevicesA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplayDevicesW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplayMonitors" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!EnumDisplayMonitors p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumDisplaySettingsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumPropsA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumPropsExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumPropsExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumPropsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumThreadWindows" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!EnumThreadWindows p1:0x{:x} p2:0x{:x}", p1, p2);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumWindowStationsA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EnumWindowStationsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EqualRect" => {
            let rect1 = emu.regs().rcx;
            let rect2 = emu.regs().rdx;
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
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if eq { 1 } else { 0 };
        }
        "EvaluateProximityToPolygon" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EvaluateProximityToPolygon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "EvaluateProximityToRect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!EvaluateProximityToRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExcludeUpdateRgn" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!ExcludeUpdateRgn p1:0x{:x}", p1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ExitWindowsEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ExitWindowsEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlashWindow" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!FlashWindow p1:0x{:x}", p1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FlashWindowEx" => {
            let p0 = emu.regs().rcx;
            log_red!(emu, "user32!FlashWindowEx");
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FrameRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!FrameRect p1:0x{:x} p2:0x{:x}", p1, p2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "FreeDDElParam" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!FreeDDElParam handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "FrostCrashedWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!FrostCrashedWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetActiveWindow" => {
            log_red!(emu, "user32!GetActiveWindow");
            emu.set_last_error(0);
            emu.regs_mut().rax = helper::handler_create("hwnd://active");
        }
        "GetAltTabInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetAltTabInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAltTabInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAltTabInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAncestor" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!GetAncestor h:0x{:x} arg2:0x{:x}", handle, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAppCompatFlags" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetAppCompatFlags h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAppCompatFlags2" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetAppCompatFlags2 h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAutoRotationState" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetAutoRotationState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetAwarenessFromDpiAwarenessContext" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetAwarenessFromDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCIMSSM" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!GetCIMSSM h:0x{:x} arg2:0x{:x}", handle, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCapture" => {
            log_red!(emu, "user32!GetCapture");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetCaretBlinkTime" => {
            log_red!(emu, "user32!GetCaretBlinkTime");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetCaretPos" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!GetCaretPos h:0x{:x} arg2:0x{:x}", handle, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClassInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClassInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClassLongA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongA '{}' p1:0x{:x}", s, p1);
        }
        "GetClassLongPtrA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongPtrA '{}' p1:0x{:x}", s, p1);
        }
        "GetClassLongPtrW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongPtrW '{}' p1:0x{:x}", s, p1);
        }
        "GetClassLongW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetClassLongW '{}' p1:0x{:x}", s, p1);
        }
        "GetClassNameA" => {
            let hwnd = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let max_count = emu.regs().r8 as usize;
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
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = class_name.len() as u64;
        }
        "GetClassNameW" => {
            let hwnd = emu.regs().rcx;
            let buf = emu.regs().rdx;
            let max_count = emu.regs().r8 as usize;
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
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = class_name.len() as u64;
        }
        "GetClassWord" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetClassWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipCursor" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetClipCursor h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardAccessToken" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetClipboardAccessToken h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardData" => {
            let format = emu.regs().rcx;
            log_red!(emu, "user32!GetClipboardData format:{}", format);
            let handle = helper::handler_create("ClipboardData");
            emu.set_last_error(0);
            emu.regs_mut().rax = handle;
        }
        "GetClipboardFormatNameA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "GetClipboardFormatNameW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "GetClipboardMetadata" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetClipboardMetadata h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardOwner" => {
            log_red!(emu, "user32!GetClipboardOwner");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetClipboardSequenceNumber" => {
            log_red!(emu, "user32!GetClipboardSequenceNumber");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetClipboardViewer" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetClipboardViewer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetComboBoxInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetComboBoxInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentInputMessageSource" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetCurrentInputMessageSource h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCurrentMonitorTopologyId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetCurrentMonitorTopologyId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCursor" => {
            log_red!(emu, "user32!GetCursor");
            let hndl = helper::handler_create("user32://cursor");
            emu.regs_mut().rax = hndl;
        }
        "GetCursorFrameInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetCursorFrameInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetCursorInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetCursorInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDCEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            log_red!(
                emu,
                "user32!GetDCEx arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let hndl = helper::handler_create("user32://GetDCEx");
            emu.regs_mut().rax = hndl;
        }
        "GetDCompositionHwndBitmap" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDCompositionHwndBitmap h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDesktopID" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDesktopID h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDialogBaseUnits" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDialogBaseUnits h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDialogControlDpiChangeBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDialogControlDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDialogDpiChangeBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDialogDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDisplayAutoRotationPreferences" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDisplayAutoRotationPreferences h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDisplayConfigBufferSizes" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDisplayConfigBufferSizes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDlgCtrlID" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!GetDlgCtrlID hwnd:0x{:x}", hwnd);
            let id = if helper::handler_exist(hwnd) {
                1u64
            } else {
                0u64
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = id;
        }
        "GetDlgItem" => {
            let hdlg = emu.regs().rcx;
            let id = emu.regs().rdx as u32;
            let hwnd = helper::handler_create("dlg_item");
            log_red!(
                emu,
                "user32!GetDlgItem dlg:0x{:x} id:{} => 0x{:x}",
                hdlg,
                id,
                hwnd
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hwnd;
        }
        "GetDlgItemInt" => {
            let hdlg = emu.regs().rcx;
            let item_id = emu.regs().rdx;
            let hndl = helper::handler_create(&format!("dlgitem_{}", item_id));
            log_red!(
                emu,
                "user32!GetDlgItemInt dlg:0x{:x} id:{} => 0x{:x}",
                hdlg,
                item_id,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "GetDlgItemTextA" => {
            let _hdlg = emu.regs().rcx;
            let _id = emu.regs().rdx as u32;
            let buf = emu.regs().r8;
            let max_count = emu.regs().r9 as u32;
            log_red!(emu, "user32!GetDlgItemTextA id:{} buf:0x{:x}", _id, buf);
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            if buf != 0 && max_count > 0 {
                emu.maps.write_byte(buf, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "GetDlgItemTextW" => {
            let _hdlg = emu.regs().rcx;
            let _id = emu.regs().rdx as u32;
            let buf = emu.regs().r8;
            let max_count = emu.regs().r9 as u32;
            log_red!(emu, "user32!GetDlgItemTextW id:{} buf:0x{:x}", _id, buf);
            if buf != 0 && max_count > 0 {
                emu.maps.write_word(buf, 0);
            }
            emu.regs_mut().rax = 0;
        }
        "GetDoubleClickTime" => {
            log_red!(emu, "user32!GetDoubleClickTime");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetDpiAwarenessContextForProcess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDpiAwarenessContextForProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiForMonitorInternal" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDpiForMonitorInternal h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiForSystem" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDpiForSystem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiForWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDpiForWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetDpiFromDpiAwarenessContext" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetDpiFromDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetExtendedPointerDeviceProperty" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetExtendedPointerDeviceProperty h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGUIThreadInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetGUIThreadInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGestureConfig" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetGestureConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGestureExtraArgs" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetGestureExtraArgs h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGestureInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetGestureInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetGuiResources" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetGuiResources h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetIconInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!GetIconInfo h:0x{:x} arg2:0x{:x}", handle, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetIconInfoExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetIconInfoExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetInputDesktop" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetInputDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetInputLocaleInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetInputLocaleInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
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
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetInternalWindowPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKBCodePage" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetKBCodePage h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKeyNameTextA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "GetKeyNameTextW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "GetKeyboardLayoutList" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetKeyboardLayoutList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKeyboardLayoutNameA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKeyboardLayoutNameW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetKeyboardState" => {
            let buf = emu.regs().rcx;
            if buf != 0 {
                for i in 0..256u64 {
                    emu.maps.write_byte(buf + i, 0);
                }
            }
            log_red!(emu, "user32!GetKeyboardState buf:0x{:x}", buf);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLastActivePopup" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetLastActivePopup h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLastInputInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetLastInputInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetLayeredWindowAttributes" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetLayeredWindowAttributes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetListBoxInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetListBoxInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationDesktopColorEffect" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopColorEffect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationDesktopMagnification" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopMagnification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationDesktopSamplingMode" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMagnificationDesktopSamplingMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMagnificationLensCtxInformation" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMagnificationLensCtxInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenu" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!GetMenu hwnd:0x{:x}", hwnd);
            let hmenu = if helper::handler_exist(hwnd) {
                helper::handler_create("menu")
            } else {
                0
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = hmenu;
        }
        "GetMenuBarInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuBarInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuCheckMarkDimensions" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuCheckMarkDimensions h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuContextHelpId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuDefaultItem" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuDefaultItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!GetMenuInfo h:0x{:x} arg2:0x{:x}", handle, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemCount" => {
            let hmenu = emu.regs().rcx;
            log_red!(emu, "user32!GetMenuItemCount hmenu:0x{:x}", hmenu);
            let count = if helper::handler_exist(hmenu) {
                1u64
            } else {
                0xffffffffu64
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = count;
        }
        "GetMenuItemID" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuItemID h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuItemRect" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuItemRect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuState" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMenuState h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuStringA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMenuStringW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetMessageExtraInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMessageExtraInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetMessagePos" => {
            log_red!(emu, "user32!GetMessagePos");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetMessageTime" => {
            helper::advance_tick(emu, 1);
            let tick = emu.tick as u64;
            log_red!(emu, "user32!GetMessageTime => {}", tick);
            emu.set_last_error(0);
            emu.regs_mut().rax = tick;
        }
        "GetMonitorInfoA" => {
            let hmonitor = emu.regs().rcx;
            let info_ptr = emu.regs().rdx;
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
            emu.regs_mut().rax = 1;
        }
        "GetMonitorInfoW" => {
            let hmonitor = emu.regs().rcx;
            let info_ptr = emu.regs().rdx;
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
            emu.regs_mut().rax = 1;
        }
        "GetMouseMovePointsEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetMouseMovePointsEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNextDlgGroupItem" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetNextDlgGroupItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetNextDlgTabItem" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetNextDlgTabItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetOpenClipboardWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetOpenClipboardWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("GetOpenClipboardWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetParent" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!GetParent hwnd:0x{:x}", hwnd);
            let parent = if helper::handler_exist(hwnd) {
                helper::handler_create("parent_hwnd")
            } else {
                0
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = parent;
        }
        "GetPointerCursorId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerCursorId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDevice" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDevice h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceCursors" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDeviceCursors h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceInputSpace" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDeviceInputSpace h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceOrientation" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDeviceOrientation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceProperties" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDeviceProperties h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDeviceRects" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDeviceRects h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerDevices" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerDevices h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameArrivalTimes" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameArrivalTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFramePenInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFramePenInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFramePenInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFramePenInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTimes" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameTimes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchpadInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchpadInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerFrameTouchpadInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerFrameTouchpadInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerInputTransform" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerInputTransform h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerPenInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerPenInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerPenInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerPenInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerTouchInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerTouchInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchpadInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerTouchpadInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerTouchpadInfoHistory" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerTouchpadInfoHistory h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPointerType" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPointerType h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPriorityClipboardFormat" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetPriorityClipboardFormat h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessDefaultLayout" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetProcessDefaultLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessDpiAwarenessInternal" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetProcessDpiAwarenessInternal h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProcessUIContextInformation" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetProcessUIContextInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetProgmanWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetProgmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPropA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetPropA '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetPropW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!GetPropW '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetQueueStatus" => {
            log_red!(emu, "user32!GetQueueStatus");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetRawInputBuffer" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetRawInputBuffer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputData" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetRawInputData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputDeviceInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputDeviceInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawInputDeviceList" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetRawInputDeviceList h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRawPointerDeviceData" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetRawPointerDeviceData h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetReasonTitleFromReasonCode" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetReasonTitleFromReasonCode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetRegisteredRawInputDevices" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetRegisteredRawInputDevices h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("GetRegisteredRawInputDevices");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollBarInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetScrollBarInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollInfo" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!GetScrollInfo p1:0x{:x} p2:0x{:x}", p1, p2);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollPos" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!GetScrollPos p1:0x{:x}", p1);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetScrollRange" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!GetScrollRange p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSendMessageReceiver" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetSendMessageReceiver h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetShellChangeNotifyWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetShellChangeNotifyWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
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
            let hmenu = emu.regs().rcx;
            let npos = emu.regs().rdx;
            log_red!(emu, "user32!GetSubMenu menu:0x{:x} pos:{}", hmenu, npos);
            let hndl = helper::handler_create("user32://menu");
            emu.regs_mut().rax = hndl;
        }
        "GetSysColor" => {
            let index = emu.regs().rcx;
            let _ = emu.maps.is_mapped(index);
            log_red!(emu, "user32!GetSysColor index:{}", index);
            let color: u64 = match index {
                0 => 0xd4d0c8,  // COLOR_SCROLLBAR
                1 => 0x004e98,  // COLOR_BACKGROUND
                2 => 0x0a246a,  // COLOR_ACTIVECAPTION
                4 => 0xd4d0c8,  // COLOR_MENU
                5 => 0xffffff,  // COLOR_WINDOW
                8 => 0x000000,  // COLOR_WINDOWTEXT
                15 => 0xd4d0c8, // COLOR_3DFACE
                _ => 0xd4d0c8,
            };
            emu.set_last_error(0);
            emu.regs_mut().rax = color;
        }
        "GetSysColorBrush" => {
            let p0 = emu.regs().rcx;
            let hndl = helper::handler_create("brush");
            log_red!(emu, "user32!GetSysColorBrush => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "GetSystemDpiForProcess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetSystemDpiForProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemMenu" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetSystemMenu h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetSystemMetricsForDpi" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetSystemMetricsForDpi h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTabbedTextExtentA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTabbedTextExtentW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTaskmanWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetTaskmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadDesktop" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetThreadDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadDpiAwarenessContext" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetThreadDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetThreadDpiHostingBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetThreadDpiHostingBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTitleBarInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetTitleBarInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTopLevelWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetTopLevelWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetTopWindow" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!GetTopWindow arg1:0x{:x}", arg1);
            let hndl = helper::handler_create("user32://GetTopWindow");
            emu.regs_mut().rax = hndl;
        }
        "GetTouchInputInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetTouchInputInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUnpredictedMessagePos" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetUnpredictedMessagePos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUpdateRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!GetUpdateRect p1:0x{:x} p2:0x{:x}", p1, p2);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUpdateRgn" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!GetUpdateRgn p1:0x{:x} p2:0x{:x}", p1, p2);
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUpdatedClipboardFormats" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetUpdatedClipboardFormats h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUserObjectInformationA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetUserObjectSecurity" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetUserObjectSecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWinStationInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWinStationInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindow" => {
            let hwnd = emu.regs().rcx;
            let cmd = emu.regs().rdx;
            log_red!(emu, "user32!GetWindow hwnd:0x{:x} cmd:{}", hwnd, cmd);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "GetWindowBand" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowBand h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowCompositionAttribute" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowCompositionAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowCompositionInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowCompositionInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowContextHelpId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowDC" => {
            let hwnd = emu.regs().rcx;
            let hdc = helper::handler_create("dc");
            log_red!(emu, "user32!GetWindowDC hwnd:0x{:x} => 0x{:x}", hwnd, hdc);
            emu.set_last_error(0);
            emu.regs_mut().rax = hdc;
        }
        "GetWindowDisplayAffinity" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowDisplayAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowDpiAwarenessContext" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowDpiHostingBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowDpiHostingBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowFeedbackSetting" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowFeedbackSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowMinimizeRect" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowMinimizeRect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowModuleFileName" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowModuleFileName h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowModuleFileNameA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowModuleFileNameW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowPlacement" => {
            let hwnd = emu.regs().rcx;
            let lpwndpl = emu.regs().rdx;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowProcessHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowProcessHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowRgn" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowRgn h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowRgnBox" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowRgnBox h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowRgnEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowRgnEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GetWindowTextLengthA" => {
            let hwnd = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            let max_count = emu.regs().r8;
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
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = title.len() as u64;
        }
        "GetWindowTextLengthW" => {
            let hwnd = emu.regs().rcx;
            let buf_ptr = emu.regs().rdx;
            let max_count = emu.regs().r8;
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
            let out_ptr = emu.regs().r8;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = title.len() as u64;
        }
        "GetWindowThreadProcessId" => {
            let hwnd = emu.regs().rcx;
            let pid_ptr = emu.regs().rdx;
            log_red!(emu, "user32!GetWindowThreadProcessId hwnd:0x{:x}", hwnd);
            if pid_ptr != 0 {
                emu.maps.write_dword(pid_ptr, 0x123);
            }
            emu.regs_mut().rax = 0x123;
        }
        "GetWindowWord" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GetWindowWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GhostWindowFromHungWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!GhostWindowFromHungWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GrayStringA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p6 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let p7 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "GrayStringW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p6 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let p7 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HandleDelegatedInput" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!HandleDelegatedInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HideCaret" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!HideCaret arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HiliteMenuItem" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!HiliteMenuItem arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "HungWindowFromGhostWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!HungWindowFromGhostWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPGetIMEA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPGetIMEA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPGetIMEW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPGetIMEW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPQueryIMEA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPQueryIMEA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPQueryIMEW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPQueryIMEW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPSetIMEA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPSetIMEA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IMPSetIMEW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IMPSetIMEW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ImpersonateDdeClientWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ImpersonateDdeClientWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
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
            let rect = emu.regs().rcx;
            let dx = emu.regs().rdx as i32;
            let dy = emu.regs().r8 as i32;
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
            emu.regs_mut().rax = 1;
        }
        "InheritWindowMonitor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InheritWindowMonitor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitDManipHook" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitDManipHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitDManipHook");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeGenericHidInjection" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitializeGenericHidInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializeGenericHidInjection");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeInputDeviceInjection" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitializeInputDeviceInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializeInputDeviceInjection");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeLpkHooks" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitializeLpkHooks arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializeLpkHooks");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializePointerDeviceInjection" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitializePointerDeviceInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializePointerDeviceInjection");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializePointerDeviceInjectionEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitializePointerDeviceInjectionEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializePointerDeviceInjectionEx");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InitializeTouchInjection" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InitializeTouchInjection arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("InitializeTouchInjection");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectDeviceInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectDeviceInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectGenericHidInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectGenericHidInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectKeyboardInput" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectKeyboardInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectMouseInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectMouseInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectPointerInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectPointerInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectSyntheticPointerInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectSyntheticPointerInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InjectTouchInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InjectTouchInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InputSpaceRegionFromPoint" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InputSpaceRegionFromPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InsertMenuA" => {
            let hmenu = emu.regs().rcx;
            let position = emu.regs().rdx as u32;
            let flags = emu.regs().r8 as u32;
            let id_new_item = emu.regs().r9;
            let new_item_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
                emu.maps.read_string(new_item_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "user32!InsertMenuA menu:0x{:x} pos:{} flags:0x{:x} '{}'",
                hmenu,
                position,
                flags,
                text
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InsertMenuItemA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InsertMenuItemW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InsertMenuW" => {
            let hmenu = emu.regs().rcx;
            let position = emu.regs().rdx as u32;
            let flags = emu.regs().r8 as u32;
            let id_new_item = emu.regs().r9;
            let new_item_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let text = if new_item_ptr != 0 && (flags & 0x0040) == 0 {
                emu.maps.read_wide_string(new_item_ptr)
            } else {
                String::new()
            };
            log_red!(
                emu,
                "user32!InsertMenuW menu:0x{:x} pos:{} flags:0x{:x} '{}'",
                hmenu,
                position,
                flags,
                text
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternalGetWindowIcon" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InternalGetWindowIcon arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InternalGetWindowText" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InternalGetWindowText arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IntersectRect" => {
            let dst = emu.regs().rcx;
            let src1 = emu.regs().rdx;
            let src2 = emu.regs().r8;
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
        }
        "InvalidateRgn" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!InvalidateRgn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "InvertRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!InvertRect p1:0x{:x}", p1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaA '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaNumericA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaNumericA '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaNumericW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaNumericW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsCharAlphaW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharAlphaW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsCharLowerA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharLowerA '{}'", s);
        }
        "IsCharLowerW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharLowerW '{}'", s);
        }
        "IsCharUpperA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharUpperA '{}'", s);
        }
        "IsCharUpperW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsCharUpperW '{}'", s);
        }
        "IsChild" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!IsChild arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsClipboardFormatAvailable" => {
            let p0 = emu.regs().rcx;
            log_red!(emu, "user32!IsClipboardFormatAvailable");
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsDialogMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsDialogMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsDialogMessageA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsDialogMessageA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsDialogMessageW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!IsDialogMessageW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsDlgButtonChecked" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!IsDlgButtonChecked p1:0x{:x}", p1);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsGUIThread" => {
            log_red!(emu, "user32!IsGUIThread");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "IsHungAppWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsHungAppWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsIconic" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!IsIconic hwnd:0x{:x}", hwnd);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsImmersiveProcess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsImmersiveProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsInDesktopWindowBand" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsInDesktopWindowBand arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsInterceptWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsInterceptWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsMenu" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!IsMenu arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsMouseInPointerEnabled" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsMouseInPointerEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("IsMouseInPointerEnabled");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsOneCoreTransformMode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsOneCoreTransformMode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsProcessDPIAware" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsProcessDPIAware h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsRectEmpty" => {
            let rect = emu.regs().rcx;
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
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if empty { 1 } else { 0 };
        }
        "IsSETEnabled" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsSETEnabled arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("IsSETEnabled");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsServerSideWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsServerSideWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsThreadDesktopComposited" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsThreadDesktopComposited h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsThreadMessageQueueAttached" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsThreadMessageQueueAttached h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("IsThreadMessageQueueAttached");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsThreadTSFEventAware" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!IsThreadTSFEventAware 0x{:x}", arg1);
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsTopLevelWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsTopLevelWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsTouchWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsTouchWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsValidDpiAwarenessContext" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsValidDpiAwarenessContext arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsWinEventHookInstalled" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!IsWinEventHookInstalled 0x{:x}", arg1);
            let handle = helper::handler_create("IsWinEventHookInstalled");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowArranged" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsWindowArranged arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsWindowDisplayChangeSuppressed" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsWindowDisplayChangeSuppressed arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowEnabled" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!IsWindowEnabled hwnd:0x{:x}", hwnd);
            let handle = helper::handler_create("IsWindowEnabled");
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowInDestroy" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsWindowInDestroy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "IsWindowRedirectedForPrint" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsWindowRedirectedForPrint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsWindowUnicode" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsWindowUnicode arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsWow64Message" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!IsWow64Message arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "IsZoomed" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!IsZoomed hwnd:0x{:x}", hwnd);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "KillTimer" => {
            let hwnd = emu.regs().rcx;
            let timer_id = emu.regs().rdx;
            helper::handler_close(timer_id);
            log_red!(emu, "user32!KillTimer hwnd:0x{:x} timer:{}", hwnd, timer_id);
            emu.regs_mut().rax = 1;
        }
        "LoadAcceleratorsA" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadAcceleratorsA arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let hndl = helper::handler_create("user32://LoadAcceleratorsA");
            emu.regs_mut().rax = hndl;
        }
        "LoadAcceleratorsW" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadAcceleratorsW arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let hndl = helper::handler_create("user32://LoadAcceleratorsW");
            emu.regs_mut().rax = hndl;
        }
        "LoadBitmapA" => {
            let hinst = emu.regs().rcx;
            let name = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadBitmapA hInst:0x{:x} name:0x{:x}",
                hinst,
                name
            );
            let hndl = helper::handler_create("user32://bitmap");
            emu.regs_mut().rax = hndl;
        }
        "LoadBitmapW" => {
            let hinst = emu.regs().rcx;
            let name = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadBitmapW hInst:0x{:x} name:0x{:x}",
                hinst,
                name
            );
            let hndl = helper::handler_create("user32://bitmap");
            emu.regs_mut().rax = hndl;
        }
        "LoadCursorFromFileA" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!LoadCursorFromFileA arg1:0x{:x}", arg1);
            let hndl = helper::handler_create("user32://LoadCursorFromFileA");
            emu.regs_mut().rax = hndl;
        }
        "LoadCursorFromFileW" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!LoadCursorFromFileW arg1:0x{:x}", arg1);
            let hndl = helper::handler_create("user32://LoadCursorFromFileW");
            emu.regs_mut().rax = hndl;
        }
        "LoadCursorW" => {
            let hinstance = emu.regs().rcx;
            let cursor_name = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadCursorW hinst:0x{:x} cursor:0x{:x}",
                hinstance,
                cursor_name
            );
            let hndl = helper::handler_create("user32://LoadCursorW");
            emu.regs_mut().rax = hndl;
        }
        "LoadIconW" => {
            let hinstance = emu.regs().rcx;
            let icon_name = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadIconW hinst:0x{:x} icon:0x{:x}",
                hinstance,
                icon_name
            );
            let hndl = helper::handler_create("user32://LoadIconW");
            emu.regs_mut().rax = hndl;
        }
        "LoadImageA" => {
            let hinstance = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let img_type = emu.regs().r8 as u32;
            let cx = emu.regs().r9 as i32;
            let cy = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
            let fu_load = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
            let name = if name_ptr > 0xffff {
                emu.maps.read_string(name_ptr)
            } else {
                format!("#{}", name_ptr)
            };
            log_red!(
                emu,
                "user32!LoadImageA hinst:0x{:x} name:'{}' type:{} cx:{} cy:{} flags:0x{:x}",
                hinstance,
                name,
                img_type,
                cx,
                cy,
                fu_load
            );
            let uri = format!("user32://LoadImageA/{}", name);
            let hndl = helper::handler_create(&uri);
            emu.regs_mut().rax = hndl;
        }
        "LoadImageW" => {
            let hinstance = emu.regs().rcx;
            let name_ptr = emu.regs().rdx;
            let img_type = emu.regs().r8 as u32;
            let cx = emu.regs().r9 as i32;
            let cy = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
            let fu_load = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0) as u32;
            let name = if name_ptr > 0xffff {
                emu.maps.read_wide_string(name_ptr)
            } else {
                format!("#{}", name_ptr)
            };
            log_red!(
                emu,
                "user32!LoadImageW hinst:0x{:x} name:'{}' type:{} cx:{} cy:{} flags:0x{:x}",
                hinstance,
                name,
                img_type,
                cx,
                cy,
                fu_load
            );
            let uri = format!("user32://LoadImageW/{}", name);
            let hndl = helper::handler_create(&uri);
            emu.regs_mut().rax = hndl;
        }
        "LoadKeyboardLayoutA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadKeyboardLayoutA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadKeyboardLayoutEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadKeyboardLayoutEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("LoadKeyboardLayoutEx");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadKeyboardLayoutW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!LoadKeyboardLayoutW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadLocalFonts" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadLocalFonts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LoadLocalFonts");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LoadMenuA" => {
            let hinst = emu.regs().rcx;
            log_red!(emu, "user32!LoadMenuA hInst:0x{:x}", hinst);
            let hndl = helper::handler_create("user32://menu");
            emu.regs_mut().rax = hndl;
        }
        "LoadMenuIndirectA" => {
            let template = emu.regs().rcx;
            log_red!(emu, "user32!LoadMenuIndirectA tmpl:0x{:x}", template);
            let hndl = helper::handler_create("user32://menu");
            emu.regs_mut().rax = hndl;
        }
        "LoadMenuIndirectW" => {
            let template = emu.regs().rcx;
            log_red!(emu, "user32!LoadMenuIndirectW tmpl:0x{:x}", template);
            let hndl = helper::handler_create("user32://menu");
            emu.regs_mut().rax = hndl;
        }
        "LoadMenuW" => {
            let hinst = emu.regs().rcx;
            log_red!(emu, "user32!LoadMenuW hInst:0x{:x}", hinst);
            let hndl = helper::handler_create("user32://menu");
            emu.regs_mut().rax = hndl;
        }
        "LoadRemoteFonts" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LoadRemoteFonts arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("LoadRemoteFonts");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LockSetForegroundWindow" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!LockSetForegroundWindow 0x{:x}", arg1);
            let handle = helper::handler_create("LockSetForegroundWindow");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LockWindowStation" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!LockWindowStation 0x{:x}", arg1);
            let handle = helper::handler_create("LockWindowStation");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LockWindowUpdate" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!LockWindowUpdate 0x{:x}", arg1);
            let handle = helper::handler_create("LockWindowUpdate");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LockWorkStation" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!LockWorkStation 0x{:x}", arg1);
            let handle = helper::handler_create("LockWorkStation");
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LogicalToPhysicalPoint" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LogicalToPhysicalPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LogicalToPhysicalPointForPerMonitorDPI" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LogicalToPhysicalPointForPerMonitorDPI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LookupIconIdFromDirectory" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LookupIconIdFromDirectory arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "LookupIconIdFromDirectoryEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!LookupIconIdFromDirectoryEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MBToWCSEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!MBToWCSEx arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MBToWCSExt" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!MBToWCSExt arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MB_GetString" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MB_GetString arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MITGetCursorUpdateHandle" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MITGetCursorUpdateHandle h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MITSetLastInputRecipient" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MITSetLastInputRecipient arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MITSynthesizeTouchInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MITSynthesizeTouchInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MakeThreadTSFEventAware" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MakeThreadTSFEventAware arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapDialogRect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MapDialogRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("MapDialogRect");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapPointsByVisualIdentifier" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MapPointsByVisualIdentifier arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("MapPointsByVisualIdentifier");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapVirtualKeyExA" | "MapVirtualKeyExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "MapVisualRelativePoints" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MapVisualRelativePoints arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("MapVisualRelativePoints");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MapWindowPoints" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!MapWindowPoints p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let handle = helper::handler_create("MapWindowPoints");
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MenuItemFromPoint" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MenuItemFromPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MenuWindowProcA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!MenuWindowProcA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MenuWindowProcW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!MenuWindowProcW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MessageBeep" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!MessageBeep arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MessageBoxExA" => {
            let hwnd = emu.regs().rcx;
            let text_ptr = emu.regs().rdx;
            let caption_ptr = emu.regs().r8;
            let mtype = emu.regs().r9 as u32;
            let _lang_id = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
            log_red!(
                emu,
                "user32!MessageBoxExA '{}' '{}' type:0x{:x}",
                caption,
                text,
                mtype
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // IDOK
        }
        "MessageBoxExW" => {
            let hwnd = emu.regs().rcx;
            let text_ptr = emu.regs().rdx;
            let caption_ptr = emu.regs().r8;
            let mtype = emu.regs().r9 as u32;
            let _lang_id = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
            log_red!(
                emu,
                "user32!MessageBoxExW '{}' '{}' type:0x{:x}",
                caption,
                text,
                mtype
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // IDOK
        }
        "MessageBoxIndirectA" => {
            let hwnd = emu.regs().rcx;
            let text_ptr = emu.regs().rdx;
            let caption_ptr = emu.regs().r8;
            let mb_type = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MessageBoxIndirectW" => {
            let hwnd = emu.regs().rcx;
            let text_ptr = emu.regs().rdx;
            let caption_ptr = emu.regs().r8;
            let mb_type = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MessageBoxTimeoutA" => {
            let hwnd = emu.regs().rcx;
            let text_ptr = emu.regs().rdx;
            let caption_ptr = emu.regs().r8;
            let mb_type = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MessageBoxTimeoutW" => {
            let hwnd = emu.regs().rcx;
            let text_ptr = emu.regs().rdx;
            let caption_ptr = emu.regs().r8;
            let mb_type = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ModifyMenuA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ModifyMenuA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ModifyMenuW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ModifyMenuW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MonitorFromPoint" => {
            let pt_x = emu.regs().rcx as i32;
            let pt_y = emu.regs().rdx as i32;
            let flags = emu.regs().r8 as u32;
            let hmon = helper::handler_create("monitor");
            log_red!(
                emu,
                "user32!MonitorFromPoint ({},{}) flags:0x{:x} => 0x{:x}",
                pt_x,
                pt_y,
                flags,
                hmon
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hmon;
        }
        "MonitorFromRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!MonitorFromRect p1:0x{:x}", p1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MonitorFromWindow" => {
            let hwnd = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let hmon = helper::handler_create("monitor");
            log_red!(
                emu,
                "user32!MonitorFromWindow hwnd:0x{:x} flags:0x{:x} => 0x{:x}",
                hwnd,
                flags,
                hmon
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = hmon;
        }
        "MoveWindow" => {
            let hwnd = emu.regs().rcx;
            let x = emu.regs().rdx as i32;
            let y = emu.regs().r8 as i32;
            let width = emu.regs().r9 as i32;
            let height = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i32;
            let repaint = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "MsgWaitForMultipleObjects" => {
            let count = emu.regs().rcx as u32;
            let handles = emu.regs().rdx;
            let wait_all = emu.regs().r8 != 0;
            let ms = emu.regs().r9 as u32;
            let wake_mask = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
            helper::advance_tick(emu, ms as u64);
            log_red!(
                emu,
                "user32!MsgWaitForMultipleObjects count:{} all:{} ms:{} mask:0x{:x}",
                count,
                wait_all,
                ms,
                wake_mask
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "MsgWaitForMultipleObjectsEx" => {
            let count = emu.regs().rcx as u32;
            let handles = emu.regs().rdx;
            let ms = emu.regs().r8 as u32;
            let wake_mask = emu.regs().r9 as u32;
            let flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
            helper::advance_tick(emu, ms as u64);
            log_red!(
                emu,
                "user32!MsgWaitForMultipleObjectsEx count:{} ms:{} mask:0x{:x} flags:0x{:x}",
                count,
                ms,
                wake_mask,
                flags
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "NotifyOverlayWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!NotifyOverlayWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "NotifyWinEvent" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            log_red!(
                emu,
                "user32!NotifyWinEvent arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OemKeyScan" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!OemKeyScan h:0x{:x} arg2:0x{:x}", handle, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OemToCharA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!OemToCharA '{}' p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OemToCharBuffA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OemToCharBuffW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OemToCharW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!OemToCharW '{}' p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OffsetRect" => {
            let rect = emu.regs().rcx;
            let dx = emu.regs().rdx as i32;
            let dy = emu.regs().r8 as i32;
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
            emu.regs_mut().rax = 1;
        }
        "OpenClipboard" => {
            let hwnd_owner = emu.regs().rcx;
            let hndl = helper::handler_create("clipboard");
            log_red!(
                emu,
                "user32!OpenClipboard owner:0x{:x} => 0x{:x}",
                hwnd_owner,
                hndl
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenDesktopA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenDesktopW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenIcon" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("OpenIcon");
            log_red!(emu, "user32!OpenIcon => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OpenInputDesktop" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("OpenInputDesktop");
            log_red!(emu, "user32!OpenInputDesktop => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OpenThreadDesktop" => {
            let arg1 = emu.regs().rcx;
            let hndl = helper::handler_create("OpenThreadDesktop");
            log_red!(emu, "user32!OpenThreadDesktop => 0x{:x}", hndl);
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "OpenWindowStationA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "OpenWindowStationW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackDDElParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PackDDElParam arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PackTouchHitTestingProximityEvaluation" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PackTouchHitTestingProximityEvaluation arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "PaintDesktop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PaintDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PaintMenuBar" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PaintMenuBar arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PaintMonitor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PaintMonitor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PhysicalToLogicalPoint" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PhysicalToLogicalPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PhysicalToLogicalPointForPerMonitorDPI" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PhysicalToLogicalPointForPerMonitorDPI arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PostThreadMessageA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PostThreadMessageW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrintWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PrintWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconsA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconsA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivateExtractIconsW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!PrivateExtractIconsW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PrivateRegisterICSProc" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!PrivateRegisterICSProc arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("PrivateRegisterICSProc");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "PtInRect" => {
            let rect = emu.regs().rcx;
            let pt_x = emu.regs().rdx as i32;
            let pt_y = emu.regs().r8 as i32;
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
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = if inside { 1 } else { 0 };
        }
        "QueryBSDRWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!QueryBSDRWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QueryDisplayConfig" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!QueryDisplayConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "QuerySendMessage" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!QuerySendMessage h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMAddInputObserver" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMAddInputObserver arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RIMAddInputObserver");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMAreSiblingDevices" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMAreSiblingDevices arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "RIMDeviceIoControl" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMDeviceIoControl arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMEnableMonitorMappingForDevice" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMEnableMonitorMappingForDevice arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RIMEnableMonitorMappingForDevice");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMFreeInputBuffer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMFreeInputBuffer arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDevicePreparsedData" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMGetDevicePreparsedData arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDevicePreparsedDataLockfree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMGetDevicePreparsedDataLockfree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RIMGetDevicePreparsedDataLockfree");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDeviceProperties" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMGetDeviceProperties arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetDevicePropertiesLockfree" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMGetDevicePropertiesLockfree arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RIMGetDevicePropertiesLockfree");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetPhysicalDeviceRect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMGetPhysicalDeviceRect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMGetSourceProcessId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMGetSourceProcessId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMObserveNextInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMObserveNextInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMOnAsyncPnpWorkNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMOnAsyncPnpWorkNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMOnPnpNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMOnPnpNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMOnTimerNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMOnTimerNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMQueryDevicePath" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMQueryDevicePath arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMReadInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMReadInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMRegisterForInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMRegisterForInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RIMRegisterForInput");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMRegisterForInputEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMRegisterForInputEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RIMRegisterForInputEx");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMRemoveInputObserver" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMRemoveInputObserver arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMSetExtendedDeviceProperty" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMSetExtendedDeviceProperty arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMSetTestModeStatus" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMSetTestModeStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMUnregisterForInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMUnregisterForInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RIMUpdateInputObserverRegistration" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RIMUpdateInputObserverRegistration arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RaiseLowerShellWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RaiseLowerShellWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealChildWindowFromPoint" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(
                emu,
                "user32!RealChildWindowFromPoint p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealGetWindowClass" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RealGetWindowClass arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealGetWindowClassA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RealGetWindowClassA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RealGetWindowClassW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RealGetWindowClassW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReasonCodeNeedsBugID" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ReasonCodeNeedsBugID arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "ReasonCodeNeedsComment" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ReasonCodeNeedsComment arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "RecordShutdownReason" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RecordShutdownReason arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RedrawWindow" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!RedrawWindow p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterBSDRWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterBSDRWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterBSDRWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterClipboardFormatA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RegisterClipboardFormatA '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xc000;
        }
        "RegisterClipboardFormatW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RegisterClipboardFormatW '{}'", s);
            emu.set_last_error(0);
            emu.regs_mut().rax = 0xc000;
        }
        "RegisterCloakedNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterCloakedNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterCloakedNotification");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterDManipHook" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterDManipHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterDManipHook");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterDeviceNotificationA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterDeviceNotificationW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterErrorReportingDialog" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterErrorReportingDialog arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterErrorReportingDialog");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterForCustomDockTargets" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterForCustomDockTargets arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterForCustomDockTargets");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterForTooltipDismissNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterForTooltipDismissNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterForTooltipDismissNotification");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterFrostWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterFrostWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterFrostWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterGhostWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterGhostWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterGhostWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterHotKey" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterHotKey");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterLogonProcess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterLogonProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterLogonProcess");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterMessagePumpHook" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterMessagePumpHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterMessagePumpHook");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPointerDeviceNotifications" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterPointerDeviceNotifications arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterPointerDeviceNotifications");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPointerInputTarget" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterPointerInputTarget arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterPointerInputTarget");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPointerInputTargetEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterPointerInputTargetEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterPointerInputTargetEx");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterPowerSettingNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterPowerSettingNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterPowerSettingNotification");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterRawInputDevices" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterRawInputDevices arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterRawInputDevices");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterServicesProcess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterServicesProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterServicesProcess");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterSessionPort" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterSessionPort h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterSessionPort");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterShellHookWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterShellHookWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterShellHookWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterSuspendResumeNotification" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterSuspendResumeNotification arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterSuspendResumeNotification");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterSystemThread" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterSystemThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterSystemThread");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTasklist" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterTasklist arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterTasklist");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchHitTestingWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterTouchHitTestingWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterTouchHitTestingWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterTouchWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterTouchWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchpadCapableThread" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterTouchpadCapableThread h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("RegisterTouchpadCapableThread");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterTouchpadCapableWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterTouchpadCapableWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterTouchpadCapableWindow");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterUserApiHook" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RegisterUserApiHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("RegisterUserApiHook");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterWindowMessageA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RegisterWindowMessageW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReleaseCapture" => {
            let handle = emu.regs().rcx;
            log_red!(emu, "user32!ReleaseCapture 0x{:x}", handle);
            helper::handler_close(handle);
            emu.regs_mut().rax = 1;
        }
        "ReleaseDwmHitTestWaiters" => {
            let handle = emu.regs().rcx;
            helper::handler_close(handle);
            log_red!(emu, "user32!ReleaseDwmHitTestWaiters handle:0x{:x}", handle);
            emu.regs_mut().rax = 1;
        }
        "RemoveClipboardFormatListener" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RemoveClipboardFormatListener h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveInjectionDevice" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RemoveInjectionDevice h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveMenu" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!RemoveMenu p1:0x{:x} p2:0x{:x}", p1, p2);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemovePropA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RemovePropA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemovePropW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!RemovePropW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveThreadTSFEventAwareness" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RemoveThreadTSFEventAwareness arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "RemoveVisualIdentifier" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!RemoveVisualIdentifier h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReplyMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ReplyMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReportInertia" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ReportInertia arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ResolveDesktopForWOW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!ResolveDesktopForWOW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ReuseDDElParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ReuseDDElParam arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScreenToClient" => {
            let hwnd = emu.regs().rcx;
            let pt = emu.regs().rdx;
            log_red!(emu, "user32!ScreenToClient hwnd:0x{:x} pt:0x{:x}", hwnd, pt);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollChildren" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ScrollChildren arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollDC" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!ScrollDC arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollWindow" => {
            let hwnd = emu.regs().rcx;
            let x_amount = emu.regs().rdx as i32;
            let y_amount = emu.regs().r8 as i32;
            let lp_rect = emu.regs().r9;
            let lp_clip_rect = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            log_red!(
                emu,
                "user32!ScrollWindow hwnd:0x{:x} dx:{} dy:{} rect:0x{:x} clip:0x{:x}",
                hwnd,
                x_amount,
                y_amount,
                lp_rect,
                lp_clip_rect
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ScrollWindowEx" => {
            let hwnd = emu.regs().rcx;
            let dx = emu.regs().rdx as i32;
            let dy = emu.regs().r8 as i32;
            let _lp_scroll = emu.regs().r9;
            let _lp_clip = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hrgn_update = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let lp_rc_update = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let _flags = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            if lp_rc_update != 0 {
                emu.maps.write_dword(lp_rc_update, 0);
                emu.maps.write_dword(lp_rc_update + 4, 0);
                emu.maps.write_dword(lp_rc_update + 8, 0);
                emu.maps.write_dword(lp_rc_update + 12, 0);
            }
            log_red!(
                emu,
                "user32!ScrollWindowEx hwnd:0x{:x} dx:{} dy:{} hrgn:0x{:x}",
                hwnd,
                dx,
                dy,
                hrgn_update
            );
            emu.set_last_error(0);
            emu.regs_mut().rax = 1; // SIMPLEREGION
        }
        "SendDlgItemMessageA" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let s = emu.maps.read_string(str_ptr);
            log_red!(emu, "user32!SendDlgItemMessageA '{}'  0x{:x}", s, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendDlgItemMessageW" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let s = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "user32!SendDlgItemMessageW '{}'  0x{:x}", s, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendIMEMessageExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SendIMEMessageExA '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendIMEMessageExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SendIMEMessageExW '{}'  p1:0x{:x}", s, p1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendInput" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!SendInput h:0x{:x} arg2:0x{:x}", handle, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendMessageCallbackA" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let name = emu.maps.read_string(str_ptr);
            log_red!(emu, "user32!SendMessageCallbackA '{}' 0x{:x}", name, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendMessageCallbackW" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let name = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "user32!SendMessageCallbackW '{}' 0x{:x}", name, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendMessageTimeoutA" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let name = emu.maps.read_string(str_ptr);
            log_red!(emu, "user32!SendMessageTimeoutA '{}' 0x{:x}", name, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendMessageTimeoutW" => {
            let str_ptr = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let name = emu.maps.read_wide_string(str_ptr);
            log_red!(emu, "user32!SendMessageTimeoutW '{}' 0x{:x}", name, arg2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendNotifyMessageA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SendNotifyMessageW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetActiveWindow" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!SetActiveWindow hwnd:0x{:x}", hwnd);
            if hwnd != 0 {
                let _ = helper::handler_create("ActiveWindow");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = hwnd;
        }
        "SetAdditionalForegroundBoostProcesses" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetAdditionalForegroundBoostProcesses h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("SetAdditionalForegroundBoostProcesses");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCapture" => {
            let hwnd = emu.regs().rcx;
            log_red!(emu, "user32!SetCapture hwnd:0x{:x}", hwnd);
            if hwnd != 0 {
                let _ = helper::handler_create("Capture");
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 0;
        }
        "SetCaretBlinkTime" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetCaretBlinkTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCaretPos" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!SetCaretPos h:0x{:x} arg2:0x{:x}", handle, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetClassLongA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "SetClassLongPtrA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "SetClassLongPtrW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "SetClassLongW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "SetClassWord" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetClassWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetClipboardData" => {
            let format = emu.regs().rcx;
            let hdata = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetClipboardData format:{} data:0x{:x}",
                format,
                hdata
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = hdata;
        }
        "SetClipboardViewer" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetClipboardViewer h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCoalescableTimer" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "user32!SetCoalescableTimer arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCoreWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetCoreWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCoveredWindowStates" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetCoveredWindowStates h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetCursorContents" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetCursorContents h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDebugErrorLevel" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDebugErrorLevel h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDeskWallpaper" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDeskWallpaper h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDesktopColorTransform" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDesktopColorTransform h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDialogControlDpiChangeBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDialogControlDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDialogDpiChangeBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDialogDpiChangeBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDisplayAutoRotationPreferences" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDisplayAutoRotationPreferences h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDisplayConfig" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDisplayConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDlgItemInt" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!SetDlgItemInt p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDlgItemTextA" => {
            let hdlg = emu.regs().rcx;
            let id = emu.regs().rdx as u32;
            let text_ptr = emu.regs().r8;
            let text = if text_ptr != 0 {
                emu.maps.read_string(text_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!SetDlgItemTextA id:{} '{}'", id, text);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDlgItemTextW" => {
            let hdlg = emu.regs().rcx;
            let id = emu.regs().rdx as u32;
            let text_ptr = emu.regs().r8;
            let text = if text_ptr != 0 {
                emu.maps.read_wide_string(text_ptr)
            } else {
                String::new()
            };
            log_red!(emu, "user32!SetDlgItemTextW id:{} '{}'", id, text);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetDoubleClickTime" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetDoubleClickTime h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFeatureReportResponse" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetFeatureReportResponse h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetForegroundRedirectionForActivationObject" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetForegroundRedirectionForActivationObject h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetFullscreenMagnifierOffsetsDWMUpdated" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetFullscreenMagnifierOffsetsDWMUpdated h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetGestureConfig" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetGestureConfig h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetInternalWindowPos" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetInternalWindowPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetKeyboardState" => {
            let buf = emu.regs().rcx;
            log_red!(emu, "user32!SetKeyboardState buf:0x{:x}", buf);
            if buf != 0 && emu.maps.is_mapped(buf) {
                let _ = emu.maps.read_dword(buf).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetLastErrorEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetLastErrorEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetLayeredWindowAttributes" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetLayeredWindowAttributes h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopColorEffect" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopColorEffect h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopMagnification" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopMagnification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopMagnifierOffsetsDWMUpdated" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopMagnifierOffsetsDWMUpdated h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationDesktopSamplingMode" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMagnificationDesktopSamplingMode h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMagnificationLensCtxInformation" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMagnificationLensCtxInformation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMaxTouchpadSensitivity" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMaxTouchpadSensitivity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenu" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!SetMenu p1:0x{:x}", p1);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuContextHelpId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMenuContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuDefaultItem" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMenuDefaultItem h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!SetMenuInfo h:0x{:x} arg2:0x{:x}", handle, arg2);
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuItemBitmaps" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMenuItemBitmaps h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuItemInfoA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMenuItemInfoW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMessageExtraInfo" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMessageExtraInfo h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMessageQueue" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMessageQueue h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("SetMessageQueue");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetMirrorRendering" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetMirrorRendering h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetParent" => {
            let hwnd_child = emu.regs().rcx;
            let hwnd_parent = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetParent child:0x{:x} parent:0x{:x}",
                hwnd_child,
                hwnd_parent
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = hwnd_parent;
        }
        "SetPhysicalCursorPos" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetPhysicalCursorPos h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetPointerDeviceInputSpace" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetPointerDeviceInputSpace h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDPIAware" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessDPIAware h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDefaultLayout" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessDefaultLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDpiAwarenessContext" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessDpiAwarenessInternal" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessDpiAwarenessInternal h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessLaunchForegroundPolicy" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessLaunchForegroundPolicy h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("SetProcessLaunchForegroundPolicy");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessRestrictionExemption" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessRestrictionExemption h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProcessWindowStation" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProcessWindowStation h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetProgmanWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetProgmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetPropA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SetPropA '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetPropW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!SetPropW '{}'  p1:0x{:x} p2:0x{:x}", s, p1, p2);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetRect" => {
            let rect = emu.regs().rcx;
            let left = emu.regs().rdx as u32;
            let top = emu.regs().r8 as u32;
            let right = emu.regs().r9 as u32;
            let bottom = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as u32;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetRectEmpty" => {
            let rect = emu.regs().rcx;
            if rect != 0 {
                emu.maps.write_dword(rect, 0);
                emu.maps.write_dword(rect + 4, 0);
                emu.maps.write_dword(rect + 8, 0);
                emu.maps.write_dword(rect + 12, 0);
            }
            log_red!(emu, "user32!SetRectEmpty 0x{:x}", rect);
            emu.regs_mut().rax = 1;
        }
        "SetScrollInfo" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!SetScrollInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let out_ptr = emu.regs().r9;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetScrollPos" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            log_red!(
                emu,
                "user32!SetScrollPos p1:0x{:x} p2:0x{:x} p3:0x{:x}",
                p1,
                p2,
                p3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetScrollRange" => {
            let hwnd = emu.regs().rcx;
            let n_bar = emu.regs().rdx as i32;
            let n_min_pos = emu.regs().r8 as i32;
            let n_max_pos = emu.regs().r9 as i32;
            let b_redraw = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            log_red!(
                emu,
                "user32!SetScrollRange hwnd:0x{:x} bar:{} min:{} max:{} redraw:{}",
                hwnd,
                n_bar,
                n_min_pos,
                n_max_pos,
                b_redraw
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetShellChangeNotifyWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetShellChangeNotifyWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetShellWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetShellWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetShellWindowEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetShellWindowEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSysColors" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!SetSysColors p1:0x{:x} p2:0x{:x}", p1, p2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSysColorsTemp" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetSysColorsTemp h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSystemCursor" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetSystemCursor h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetSystemMenu" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetSystemMenu h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetTaskmanWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetTaskmanWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadCursorCreationScaling" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetThreadCursorCreationScaling h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDesktop" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetThreadDesktop h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDpiAwarenessContext" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetThreadDpiAwarenessContext h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadDpiHostingBehavior" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetThreadDpiHostingBehavior h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetThreadInputBlocked" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetThreadInputBlocked arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetTimer" => {
            let hwnd = emu.regs().rcx;
            let id = emu.regs().rdx;
            let elapse = emu.regs().r8 as u32;
            let callback = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = timer_handle;
        }
        "SetUserObjectCapability" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetUserObjectCapability h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUserObjectInformationA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUserObjectInformationW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetUserObjectSecurity" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetUserObjectSecurity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWinEventHook" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            let arg3 = emu.regs().r8;
            let arg4 = emu.regs().r9;
            let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            log_red!(
                emu,
                "user32!SetWinEventHook arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
                arg1,
                arg2,
                arg3
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowBand" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowBand h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowCompositionAttribute" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowCompositionAttribute h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowCompositionTransition" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowCompositionTransition h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowContextHelpId" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowContextHelpId h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowDisplayAffinity" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowDisplayAffinity h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowFeedbackSetting" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowFeedbackSetting h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowMessageCapability" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowMessageCapability h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowPlacement" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowPlacement h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowRgn" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowRgn h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowRgnEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowRgnEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowStationUser" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowStationUser h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowWord" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SetWindowWord h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowsHookA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowsHookExAW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SetWindowsHookExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = hndl;
        }
        "SetWindowsHookW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellForegroundBoostProcess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellForegroundBoostProcess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellHandwritingDelegateInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellHandwritingDelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellHandwritingHandleDelegatedInput" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellHandwritingHandleDelegatedInput h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellHandwritingUndelegateInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellHandwritingUndelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellMigrateWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellMigrateWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellRegisterHotKey" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellRegisterHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("ShellRegisterHotKey");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShellSetWindowPos" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShellSetWindowPos arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowCaret" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!ShowCaret arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowOwnedPopups" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShowOwnedPopups arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowScrollBar" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!ShowScrollBar p1:0x{:x} p2:0x{:x}", p1, p2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowStartGlass" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShowStartGlass arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ShowStartGlass");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowSystemCursor" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShowSystemCursor arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShowWindowAsync" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShowWindowAsync arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShutdownBlockReasonCreate" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShutdownBlockReasonCreate arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("ShutdownBlockReasonCreate");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShutdownBlockReasonDestroy" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShutdownBlockReasonDestroy arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ShutdownBlockReasonQuery" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ShutdownBlockReasonQuery arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SignalRedirectionStartComplete" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SignalRedirectionStartComplete arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("SignalRedirectionStartComplete");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SkipPointerFrameMessages" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SkipPointerFrameMessages arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SoftModalMessageBox" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SoftModalMessageBox arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SoundSentry" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SoundSentry arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SubtractRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(emu, "user32!SubtractRect p1:0x{:x} p2:0x{:x}", p1, p2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SuppressWindowDisplayChange" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SuppressWindowDisplayChange arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwapMouseButton" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SwapMouseButton arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchDesktop" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SwitchDesktop arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchDesktopWithFade" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SwitchDesktopWithFade arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SwitchToThisWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SwitchToThisWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SystemParametersInfoForDpi" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!SystemParametersInfoForDpi arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "SystemParametersInfoW" => {
            let ui_action = emu.regs().rcx as u32;
            let ui_param = emu.regs().rdx as u32;
            let pv_param = emu.regs().r8;
            let _f_win_ini = emu.regs().r9;
            log_red!(
                emu,
                "user32!SystemParametersInfoW action:0x{:x} param:{} pvParam:0x{:x}",
                ui_action,
                ui_param,
                pv_param
            );
            if pv_param != 0 {
                match ui_action {
                    0x0030 => {
                        emu.maps.write_dword(pv_param, 0);
                        emu.maps.write_dword(pv_param + 4, 0);
                        emu.maps.write_dword(pv_param + 8, 1920);
                        emu.maps.write_dword(pv_param + 12, 1040);
                    }
                    0x0048 => {
                        emu.maps.write_dword(pv_param, ui_param);
                    }
                    _ => {}
                }
            }
            emu.regs_mut().rax = 1;
        }
        "TabbedTextOutA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p6 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let p7 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
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
        }
        "TabbedTextOutW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
            let p4 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let p5 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let p6 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
            let p7 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
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
        }
        "TileChildWindows" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!TileChildWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TileWindows" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!TileWindows arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ToAscii" => {
            let virt_key = emu.regs().rcx as u32;
            let scan_code = emu.regs().rdx as u32;
            let _lp_key_state = emu.regs().r8;
            let lp_char = emu.regs().r9;
            let _flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let ch = if (0x20..=0x7e).contains(&virt_key) {
                virt_key as u8
            } else {
                0
            };
            if lp_char != 0 {
                emu.maps.write_word(lp_char, ch as u16);
            }
            log_red!(
                emu,
                "user32!ToAscii vk:0x{:x} scan:0x{:x} => 0x{:x}",
                virt_key,
                scan_code,
                ch
            );
            emu.regs_mut().rax = if ch != 0 { 1 } else { 0 };
        }
        "ToAsciiEx" => {
            let virt_key = emu.regs().rcx as u32;
            let scan_code = emu.regs().rdx as u32;
            let _lp_key_state = emu.regs().r8;
            let lp_char = emu.regs().r9;
            let _flags = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let _hkl = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let ch = if (0x20..=0x7e).contains(&virt_key) {
                virt_key as u8
            } else {
                0
            };
            if lp_char != 0 {
                emu.maps.write_word(lp_char, ch as u16);
            }
            log_red!(
                emu,
                "user32!ToAsciiEx vk:0x{:x} scan:0x{:x} => 0x{:x}",
                virt_key,
                scan_code,
                ch
            );
            emu.regs_mut().rax = if ch != 0 { 1 } else { 0 };
        }
        "ToUnicode" => {
            let virt_key = emu.regs().rcx as u32;
            let scan_code = emu.regs().rdx as u32;
            let _lp_key_state = emu.regs().r8;
            let pwsz_buff = emu.regs().r9;
            let cch_buff = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i32;
            let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let ch = if (0x20..=0x7e).contains(&virt_key) {
                virt_key as u16
            } else {
                0
            };
            if pwsz_buff != 0 && cch_buff > 0 {
                emu.maps.write_word(pwsz_buff, ch);
                if cch_buff > 1 {
                    emu.maps.write_word(pwsz_buff + 2, 0);
                }
            }
            log_red!(
                emu,
                "user32!ToUnicode vk:0x{:x} scan:0x{:x} => 0x{:x}",
                virt_key,
                scan_code,
                ch
            );
            emu.regs_mut().rax = if ch != 0 { 1 } else { 0 };
        }
        "ToUnicodeEx" => {
            let virt_key = emu.regs().rcx as u32;
            let scan_code = emu.regs().rdx as u32;
            let _lp_key_state = emu.regs().r8;
            let pwsz_buff = emu.regs().r9;
            let cch_buff = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0) as i32;
            let _flags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let _hkl = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            let ch = if (0x20..=0x7e).contains(&virt_key) {
                virt_key as u16
            } else {
                0
            };
            if pwsz_buff != 0 && cch_buff > 0 {
                emu.maps.write_word(pwsz_buff, ch);
                if cch_buff > 1 {
                    emu.maps.write_word(pwsz_buff + 2, 0);
                }
            }
            log_red!(
                emu,
                "user32!ToUnicodeEx vk:0x{:x} scan:0x{:x} => 0x{:x}",
                virt_key,
                scan_code,
                ch
            );
            emu.regs_mut().rax = if ch != 0 { 1 } else { 0 };
        }
        "TrackMouseEvent" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!TrackMouseEvent arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TrackPopupMenu" => {
            let hmenu = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let x = emu.regs().r8 as i32;
            let y = emu.regs().r9 as i32;
            let _reserved = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let hwnd = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            let _lp_tpm = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
            log_red!(
                emu,
                "user32!TrackPopupMenu hmenu:0x{:x} flags:0x{:x} x:{} y:{} hwnd:0x{:x}",
                hmenu,
                flags,
                x,
                y,
                hwnd
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TrackPopupMenuEx" => {
            let hmenu = emu.regs().rcx;
            let flags = emu.regs().rdx as u32;
            let x = emu.regs().r8 as i32;
            let y = emu.regs().r9 as i32;
            let hwnd = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
            let _lptpm = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
            log_red!(
                emu,
                "user32!TrackPopupMenuEx hmenu:0x{:x} flags:0x{:x} x:{} y:{} hwnd:0x{:x}",
                hmenu,
                flags,
                x,
                y,
                hwnd
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TranslateAccelerator" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            log_red!(
                emu,
                "user32!TranslateAccelerator p1:0x{:x} p2:0x{:x}",
                p1,
                p2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TranslateAcceleratorA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "TranslateAcceleratorW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
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
        }
        "TranslateMDISysAccel" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!TranslateMDISysAccel arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "TranslateMessageEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!TranslateMessageEx arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let src = emu.regs().rcx;
            if src != 0 && emu.maps.is_mapped(src) {
                let _input = emu.maps.read_string(src);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UndelegateInput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UndelegateInput arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnhookWinEvent" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!UnhookWinEvent 0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnhookWindowsHook" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnhookWindowsHook arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnhookWindowsHookEx" => {
            let hhk = emu.regs().rcx;
            log_red!(emu, "user32!UnhookWindowsHookEx hhk:0x{:x}", hhk);
            if hhk != 0 {
                helper::handler_close(hhk);
            }
            emu.regs_mut().rax = 1;
        }
        "UnionRect" => {
            let dst = emu.regs().rcx;
            let src1 = emu.regs().rdx;
            let src2 = emu.regs().r8;
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
            emu.regs_mut().rax = 1;
        }
        "UnloadKeyboardLayout" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnloadKeyboardLayout h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnlockWindowStation" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!UnlockWindowStation 0x{:x}", arg1);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnpackDDElParam" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnpackDDElParam arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterDeviceNotification" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterDeviceNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterHotKey" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterHotKey h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterMessagePumpHook" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterMessagePumpHook h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterPointerInputTarget" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterPointerInputTarget h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterPointerInputTargetEx" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterPointerInputTargetEx h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterPowerSettingNotification" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterPowerSettingNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterSessionPort" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterSessionPort h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterSuspendResumeNotification" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterSuspendResumeNotification h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterTouchWindow" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterTouchWindow h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UnregisterUserApiHook" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UnregisterUserApiHook h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                helper::handler_close(h);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateDefaultDesktopThumbnail" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UpdateDefaultDesktopThumbnail arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateLayeredWindow" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UpdateLayeredWindow arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateLayeredWindowIndirect" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UpdateLayeredWindowIndirect arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdatePerUserSystemParameters" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UpdatePerUserSystemParameters arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UpdateWindowInputSinkHints" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UpdateWindowInputSinkHints arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "User32InitializeImmEntryTable" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!User32InitializeImmEntryTable arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("User32InitializeImmEntryTable");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserClientDllInitialize" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UserClientDllInitialize arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("UserClientDllInitialize");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserHandleGrantAccess" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UserHandleGrantAccess h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserLpkPSMTextOut" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UserLpkPSMTextOut arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserLpkTabbedTextOut" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UserLpkTabbedTextOut arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserRealizePalette" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UserRealizePalette arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "UserRegisterWowHandlers" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!UserRegisterWowHandlers h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            let handle = helper::handler_create("UserRegisterWowHandlers");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VRipOutput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!VRipOutput arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "VTagOutput" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!VTagOutput arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "ValidateRect" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!ValidateRect p1:0x{:x}", p1);
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "ValidateRgn" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!ValidateRgn arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "VkKeyScanA" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanA '{}'", s);
        }
        "VkKeyScanExA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanExA '{}' p1:0x{:x}", s, p1);
        }
        "VkKeyScanExW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanExW '{}' p1:0x{:x}", s, p1);
        }
        "VkKeyScanW" => {
            let p0 = emu.regs().rcx;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!VkKeyScanW '{}'", s);
        }
        "WCSToMBEx" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(emu, "user32!WCSToMBEx arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WINNLSEnableIME" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WINNLSEnableIME arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("WINNLSEnableIME");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WINNLSGetEnableStatus" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WINNLSGetEnableStatus arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("WINNLSGetEnableStatus");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WINNLSGetIMEHotkey" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WINNLSGetIMEHotkey arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WaitForInputIdle" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WaitForInputIdle arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WaitForRedirectionStartComplete" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WaitForRedirectionStartComplete arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let handle = helper::handler_create("WaitForRedirectionStartComplete");
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, handle);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WaitMessage" => {
            let arg1 = emu.regs().rcx;
            log_red!(emu, "user32!WaitMessage 0x{:x}", arg1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WinHelpA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WinHelpW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let p3 = emu.regs().r9;
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
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WindowFromDC" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WindowFromDC arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WindowFromPhysicalPoint" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!WindowFromPhysicalPoint arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "WindowFromPoint" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            log_red!(emu, "user32!WindowFromPoint p1:0x{:x}", p1);
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "_UserTestTokenForInteractive" => {
            let handle = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!_UserTestTokenForInteractive h:0x{:x} arg2:0x{:x}",
                handle,
                arg2
            );
            emu.set_last_error(0);
            let h = emu.regs().rcx;
            if helper::handler_exist(h) {
                emu.regs_mut().rax = 1;
            } else {
                emu.regs_mut().rax = 0;
            }
        }
        "gSharedInfo" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!gSharedInfo arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let out_ptr = emu.regs().rdx;
            if out_ptr != 0 {
                emu.maps.write_qword(out_ptr, 0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "gapfnScSendMessage" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!gapfnScSendMessage arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let val_ptr = emu.regs().rdx;
            if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
                let _val = emu.maps.read_string(val_ptr);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "keybd_event" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!keybd_event arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "mouse_event" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!mouse_event arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            helper::advance_tick(emu, 1);
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }
        "wvsprintfA" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!wvsprintfA '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
        }
        "wvsprintfW" => {
            let p0 = emu.regs().rcx;
            let p1 = emu.regs().rdx;
            let p2 = emu.regs().r8;
            let s = if p0 != 0 && p0 > 0xffff {
                emu.maps.read_wide_string(p0)
            } else {
                format!("0x{:x}", p0)
            };
            log_red!(emu, "user32!wvsprintfW '{}' p1:0x{:x} p2:0x{:x}", s, p1, p2);
        }
        "Wow64Transition" => {
            let arg1 = emu.regs().rcx;
            let arg2 = emu.regs().rdx;
            log_red!(
                emu,
                "user32!Wow64Transition arg1:0x{:x} arg2:0x{:x}",
                arg1,
                arg2
            );
            let first_param = emu.regs().rcx;
            if first_param != 0 && emu.maps.is_mapped(first_param) {
                let _ = emu.maps.read_dword(first_param).unwrap_or(0);
            }
            emu.set_last_error(0);
            emu.regs_mut().rax = 1;
        }

        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

fn MessageBoxA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let utype = emu.regs().r9;

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

    log_red!(
        emu,
        "user32!MessageBoxA hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
        hwnd,
        caption,
        text,
        utype
    );

    emu.regs_mut().rax = 1; // IDOK
}

pub fn MessageBoxW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let text_ptr = emu.regs().rdx;
    let caption_ptr = emu.regs().r8;
    let utype = emu.regs().r9;

    let text = emu.maps.read_wide_string(text_ptr);
    let caption = if caption_ptr != 0 {
        emu.maps.read_wide_string(caption_ptr)
    } else {
        String::new()
    };

    log_red!(
        emu,
        "user32!MessageBoxW hwnd=0x{:x} caption='{}' text='{}' type=0x{:x}",
        hwnd,
        caption,
        text,
        utype
    );

    emu.regs_mut().rax = 1; // IDOK
}

fn GetDesktopWindow(emu: &mut emu::Emu) {
    let h = helper::handler_create("desktop_window");
    log_red!(emu, "user32!GetDesktopWindow => 0x{:x}", h);
    emu.regs_mut().rax = h;
}

/*
int GetSystemMetrics(
  [in] int nIndex
);
*/
fn GetSystemMetrics(emu: &mut emu::Emu) {
    let nindex = emu.regs().rcx as i32;
    let result: u64 = match nindex {
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
        "user32!GetSystemMetrics nindex: {} = {}",
        nindex,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}

/*
BOOL SystemParametersInfoA(
  [in]      UINT  uiAction,
  [in]      UINT  uiParam,
  [in, out] PVOID pvParam,
  [in]      UINT  fWinIni
);
*/
fn SystemParametersInfoA(emu: &mut emu::Emu) {
    let ui_action = emu.regs().rcx as u32;
    let ui_param = emu.regs().rdx as u32;
    let pv_param = emu.regs().r8;
    let _f_win_ini = emu.regs().r9;
    log_red!(
        emu,
        "user32!SystemParametersInfoA action:0x{:x} param:{} pvParam:0x{:x}",
        ui_action,
        ui_param,
        pv_param
    );
    if pv_param != 0 {
        match ui_action {
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
}

/*
HICON LoadIconA(
[in, optional] HINSTANCE hInstance,
[in]           LPCSTR    lpIconName
);
*/
fn LoadIconA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let lpiconname = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadIconA hInst:0x{:x} name:0x{:x}",
        hinstance,
        lpiconname
    );
    emu.regs_mut().rax = helper::handler_create("user32://icon");
}

/*
HCURSOR LoadCursorA(
  [in, optional] HINSTANCE hInstance,
  [in]           LPCSTR    lpCursorName
);
*/
fn LoadCursorA(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let lpcursorname = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadCursorA hInst:0x{:x} name:0x{:x}",
        hinstance,
        lpcursorname
    );
    emu.regs_mut().rax = helper::handler_create("user32://cursor");
}

/*
ATOM RegisterClassA(
  [in] const WNDCLASSA *lpWndClass
);
*/
fn RegisterClassA(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    let atom = 0xc000u64 + (lpwndclass & 0xfff);
    log_red!(
        emu,
        "user32!RegisterClassA wndclass:0x{:x} => atom:0x{:x}",
        lpwndclass,
        atom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = atom;
    log_red!(emu, "** {} user32!RegisterClassA {}", emu.pos, lpwndclass);
    register_class(emu, lpwndclass, false);
}

fn RegisterClassW(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    let atom = 0xc000u64 + (lpwndclass & 0xfff);
    log_red!(
        emu,
        "user32!RegisterClassW wndclass:0x{:x} => atom:0x{:x}",
        lpwndclass,
        atom
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = atom;
    log_red!(emu, "** {} user32!RegisterClassW {}", emu.pos, lpwndclass);
    register_class(emu, lpwndclass, true);
}

/*
ATOM RegisterClassExA(
  [in] const WNDCLASSEXA *unnamedParam1
);
*/
fn RegisterClassExA(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "user32!RegisterClassExA 0x{:x}", lpwndclass);
    register_class(emu, lpwndclass, false);
}

/*
ATOM RegisterClassExW(
  [in] const WNDCLASSEXW *unnamedParam1
);
*/
fn RegisterClassExW(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "user32!RegisterClassExW 0x{:x}", lpwndclass);
    register_class(emu, lpwndclass, true);
}

/*
BOOL UnregisterClassW(
  [in]           LPCWSTR   lpClassName,
  [in, optional] HINSTANCE hInstance
);
*/
fn UnregisterClassA(emu: &mut emu::Emu) {
    unregister_class(emu, "UnregisterClassA", false);
}

fn UnregisterClassW(emu: &mut emu::Emu) {
    unregister_class(emu, "UnregisterClassW", true);
}

/// Shared body of UnregisterClass{A,W}: TRUE in rax when the class was removed.
fn unregister_class(emu: &mut emu::Emu, api: &str, wide: bool) {
    let class_ptr = emu.regs().rcx;
    let name = class_arg(emu, class_ptr, wide).unwrap_or_default();
    log_red!(emu, "user32!{} class:'{}'", api, name);
    let error = match window_table::unregister_class(&name) {
        Ok(()) => {
            emu.regs_mut().rax = 1;
            return;
        }
        Err(window_table::UnregisterError::NotRegistered) => constants::ERROR_CLASS_DOES_NOT_EXIST,
        Err(window_table::UnregisterError::HasWindows) => constants::ERROR_CLASS_HAS_WINDOWS,
    };
    emu.set_last_error(error);
    emu.regs_mut().rax = 0;
}

/// WNDCLASS and WNDCLASSEX both keep lpszClassName at +0x40 on x64.
const WNDCLASS_CLASS_NAME_OFF: u64 = 0x40;

/// Shared body of RegisterClass[Ex]{A,W}: returns the class ATOM in rax.
fn register_class(emu: &mut emu::Emu, lpwndclass: u64, wide: bool) {
    let name_ptr = match emu.maps.read_qword(lpwndclass + WNDCLASS_CLASS_NAME_OFF) {
        Some(ptr) if lpwndclass != 0 => ptr,
        _ => {
            emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
            emu.regs_mut().rax = 0;
            return;
        }
    };
    match class_arg(emu, name_ptr, wide) {
        Some(name) => emu.regs_mut().rax = window_table::register_class(&name) as u64,
        None => {
            emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
            emu.regs_mut().rax = 0;
        }
    }
}

/// lpClassName as CreateWindowEx/FindWindow take it: NULL, an ATOM
/// (MAKEINTATOM, below 0x10000) or a string.
fn class_arg(emu: &mut emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        return None;
    }
    if ptr < 0x10000 {
        return Some(window_table::class_name(ptr as u16).unwrap_or_else(|| format!("#{}", ptr)));
    }
    Some(read_text(emu, ptr, wide))
}

/// Optional window title argument (NULL = no filter).
fn title_arg(emu: &mut emu::Emu, ptr: u64, wide: bool) -> Option<String> {
    if ptr == 0 {
        None
    } else {
        Some(read_text(emu, ptr, wide))
    }
}

fn read_text(emu: &mut emu::Emu, ptr: u64, wide: bool) -> String {
    if wide {
        emu.maps.read_wide_string(ptr)
    } else {
        emu.maps.read_string(ptr)
    }
}

/*
BOOL EnumWindows(
  [in] WNDENUMPROC lpEnumFunc,
  [in] LPARAM      lParam
);
*/
fn EnumWindows(emu: &mut emu::Emu) {
    let enum_func = emu.regs().rcx;
    let lparam = emu.regs().rdx;
    log_red!(
        emu,
        "user32!EnumWindows lpEnumFunc: 0x{:x} lParam: 0x{:x}",
        enum_func,
        lparam
    );

    // BOOL CALLBACK EnumWindowsProc(HWND, LPARAM); FALSE stops the walk and
    // makes EnumWindows return FALSE too.
    for hwnd in window_table::top_level_windows() {
        match emu.call64(enum_func, &[hwnd, lparam]) {
            Ok(keep_going) if keep_going as u32 != 0 => {}
            Ok(_) => {
                emu.regs_mut().rax = 0;
                return;
            }
            Err(e) => {
                log::warn!(
                    "EnumWindows callback 0x{:x} failed: {}",
                    enum_func,
                    e.message
                );
                emu.regs_mut().rax = 0;
                return;
            }
        }
    }
    emu.regs_mut().rax = 1;
}

/*
HWINSTA GetProcessWindowStation();
*/
fn GetProcessWindowStation(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetProcessWindowStation");
    emu.regs_mut().rax = helper::handler_create("WinSta0");
}

const UOI_FLAGS: u64 = 1;
const UOI_NAME: u64 = 2;
const UOI_TYPE: u64 = 3;
const WSF_VISIBLE: u32 = 1;

/*
BOOL GetUserObjectInformationW(
  [in]            HANDLE  hObj,
  [in]            int     nIndex,
  [out, optional] PVOID   pvInfo,
  [in]            DWORD   nLength,
  [out, optional] LPDWORD lpnLengthNeeded
);
*/
fn GetUserObjectInformationW(emu: &mut emu::Emu) {
    let hobj = emu.regs().rcx;
    let index = emu.regs().rdx & 0xffff_ffff;
    let info = emu.regs().r8;
    let length = emu.regs().r9 & 0xffff_ffff;
    let needed_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    log_red!(
        emu,
        "user32!GetUserObjectInformationW hObj: 0x{:x} nIndex: {} pvInfo: 0x{:x} nLength: {}",
        hobj,
        index,
        info,
        length
    );

    let data = match index {
        // USEROBJECTFLAGS { fInherit, fReserved, dwFlags }: an interactive, visible station
        UOI_FLAGS => [0u32, 0, WSF_VISIBLE]
            .iter()
            .flat_map(|v| v.to_le_bytes())
            .collect::<Vec<u8>>(),
        UOI_NAME => {
            let name = helper::handler_get_uri(hobj);
            let name = if name.is_empty() {
                "WinSta0".to_string()
            } else {
                name
            };
            wide_z(&name)
        }
        UOI_TYPE => wide_z("WindowStation"),
        _ => {
            emu.set_last_error(constants::ERROR_INVALID_PARAMETER);
            emu.regs_mut().rax = 0;
            return;
        }
    };

    if needed_ptr != 0 {
        emu.maps.write_dword(needed_ptr, data.len() as u32);
    }
    if info == 0 || length < data.len() as u64 {
        emu.set_last_error(constants::ERROR_INSUFFICIENT_BUFFER);
        emu.regs_mut().rax = 0;
        return;
    }

    emu.maps.write_bytes(info, &data);
    emu.regs_mut().rax = 1;
}

/// UTF-16LE bytes of `s` including the terminating NUL.
fn wide_z(s: &str) -> Vec<u8> {
    s.encode_utf16()
        .chain(std::iter::once(0))
        .flat_map(|c| c.to_le_bytes())
        .collect()
}

/*
HDC GetDC(
  [in] HWND hWnd
);
*/
fn GetDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!GetDC hwnd=0x{:x}", hwnd);
    emu.regs_mut().rax = helper::handler_create("hdc://");
}

/*
int ReleaseDC(
  [in] HWND hWnd,
  [in] HDC  hDC
);
*/
fn ReleaseDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let hdc = emu.regs().rdx;
    log_red!(emu, "user32!ReleaseDC hwnd=0x{:x} hdc=0x{:x}", hwnd, hdc);
    helper::handler_close(hdc);
    emu.regs_mut().rax = 1;
}

fn CharLowerBuffW(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!CharLowerBuffW buf=0x{:x} len={}", buf, len);
    for i in 0..len as u64 {
        if let Some(ch) = emu.maps.read_word(buf + i * 2)
            && let Some(c) = char::from_u32(ch as u32)
        {
            let lower = c.to_lowercase().next().unwrap_or(c) as u16;
            emu.maps.write_word(buf + i * 2, lower);
        }
    }
    emu.regs_mut().rax = len as u64;
}

fn CharLowerBuffA(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!CharLowerBuffA buf=0x{:x} len={}", buf, len);
    for i in 0..len as u64 {
        if let Some(ch) = emu.maps.read_byte(buf + i) {
            emu.maps
                .write_byte(buf + i, (ch as char).to_ascii_lowercase() as u8);
        }
    }
    emu.regs_mut().rax = len as u64;
}

fn CharUpperBuffW(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!CharUpperBuffW buf=0x{:x} len={}", buf, len);
    for i in 0..len as u64 {
        if let Some(ch) = emu.maps.read_word(buf + i * 2)
            && let Some(c) = char::from_u32(ch as u32)
        {
            let upper = c.to_uppercase().next().unwrap_or(c) as u16;
            emu.maps.write_word(buf + i * 2, upper);
        }
    }
    emu.regs_mut().rax = len as u64;
}

fn CharUpperBuffA(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let len = emu.regs().rdx as u32;
    log_red!(emu, "user32!CharUpperBuffA buf=0x{:x} len={}", buf, len);
    for i in 0..len as u64 {
        if let Some(ch) = emu.maps.read_byte(buf + i) {
            emu.maps
                .write_byte(buf + i, (ch as char).to_ascii_uppercase() as u8);
        }
    }
    emu.regs_mut().rax = len as u64;
}

pub fn FindWindowA(emu: &mut emu::Emu) {
    let class_ptr = emu.regs().rcx;
    let window_ptr = emu.regs().rdx;
    find_window(emu, "FindWindowA", 0, 0, class_ptr, window_ptr, false);
}

pub fn FindWindowW(emu: &mut emu::Emu) {
    let class_ptr = emu.regs().rcx;
    let window_ptr = emu.regs().rdx;
    find_window(emu, "FindWindowW", 0, 0, class_ptr, window_ptr, true);
}

/*
HWND FindWindowExA(
  [in, optional] HWND   hWndParent,
  [in, optional] HWND   hWndChildAfter,
  [in, optional] LPCSTR lpszClass,
  [in, optional] LPCSTR lpszWindow
);
*/
pub fn FindWindowExA(emu: &mut emu::Emu) {
    let (parent, after) = (emu.regs().rcx, emu.regs().rdx);
    let (class_ptr, window_ptr) = (emu.regs().r8, emu.regs().r9);
    find_window(
        emu,
        "FindWindowExA",
        parent,
        after,
        class_ptr,
        window_ptr,
        false,
    );
}

pub fn FindWindowExW(emu: &mut emu::Emu) {
    let (parent, after) = (emu.regs().rcx, emu.regs().rdx);
    let (class_ptr, window_ptr) = (emu.regs().r8, emu.regs().r9);
    find_window(
        emu,
        "FindWindowExW",
        parent,
        after,
        class_ptr,
        window_ptr,
        true,
    );
}

/// Shared body of FindWindow[Ex]{A,W}: the HWND in rax, 0 when not found.
fn find_window(
    emu: &mut emu::Emu,
    api: &str,
    parent: u64,
    after: u64,
    class_ptr: u64,
    window_ptr: u64,
    wide: bool,
) {
    let class_name = class_arg(emu, class_ptr, wide);
    let window_name = title_arg(emu, window_ptr, wide);
    let hwnd =
        window_table::find_window(parent, after, class_name.as_deref(), window_name.as_deref());

    log_red!(
        emu,
        "user32!{} parent=0x{:x} after=0x{:x} class={:?} window={:?} = 0x{:x}",
        api,
        parent,
        after,
        class_name,
        window_name,
        hwnd
    );

    emu.regs_mut().rax = hwnd;
}

pub fn GetKeyState(emu: &mut emu::Emu) {
    let vkey = emu.regs().rcx as u32;
    log_red!(emu, "user32!GetKeyState vKey=0x{:x}", vkey);
    emu.regs_mut().rax = 0; // key not pressed
}

pub fn GetAsyncKeyState(emu: &mut emu::Emu) {
    let vkey = emu.regs().rcx as u32;
    log_red!(emu, "user32!GetAsyncKeyState vKey=0x{:x}", vkey);
    emu.regs_mut().rax = 0; // key not pressed
}

pub fn SetWindowsHookExA(emu: &mut emu::Emu) {
    let id_hook = emu.regs().rcx as i32;
    let lpfn = emu.regs().rdx;
    let hmod = emu.regs().r8;
    let thread_id = emu.regs().r9 as u32;

    log_red!(
        emu,
        "user32!SetWindowsHookExA idHook={} lpfn=0x{:x} hMod=0x{:x} threadId={}",
        id_hook,
        lpfn,
        hmod,
        thread_id
    );

    emu.regs_mut().rax = helper::handler_create("hook://");
}

pub fn wsprintfA(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;
    let format_ptr = emu.regs().rdx;
    let format = emu.maps.read_string(format_ptr);

    let regs = [emu.regs().r8, emu.regs().r9];
    let rsp = emu.regs().rsp;
    let mut arg_idx: usize = 0;
    let mut result = String::new();
    let mut chars = format.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '%' {
            result.push(ch);
            continue;
        }
        match chars.peek() {
            Some('%') => {
                chars.next();
                result.push('%');
            }
            Some('s') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                if val != 0 {
                    result.push_str(&emu.maps.read_string(val));
                }
            }
            Some('d' | 'i') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push_str(&(val as i64).to_string());
            }
            Some('u') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push_str(&val.to_string());
            }
            Some('x' | 'X') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push_str(&format!("{:x}", val));
            }
            Some('c') => {
                chars.next();
                let val = if arg_idx < 2 {
                    regs[arg_idx]
                } else {
                    emu.maps
                        .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                        .unwrap_or(0)
                };
                arg_idx += 1;
                result.push((val & 0xFF) as u8 as char);
            }
            _ => {
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphabetic() {
                        chars.next();
                        let val = if arg_idx < 2 {
                            regs[arg_idx]
                        } else {
                            emu.maps
                                .read_qword(rsp + 0x28 + ((arg_idx - 2) as u64) * 8)
                                .unwrap_or(0)
                        };
                        arg_idx += 1;
                        result.push_str(&val.to_string());
                        break;
                    }
                    result.push(c);
                    chars.next();
                }
            }
        }
    }

    log_red!(
        emu,
        "user32!wsprintfA buffer=0x{:x} fmt='{}' => '{}'",
        buffer,
        format,
        result
    );

    emu.maps.write_string(buffer, &result);
    emu.regs_mut().rax = result.len() as u64;
}

pub fn wsprintfW(emu: &mut emu::Emu) {
    let buffer = emu.regs().rcx;
    let format_ptr = emu.regs().rdx;
    let format = emu.maps.read_wide_string(format_ptr);

    log_red!(
        emu,
        "user32!wsprintfW buffer=0x{:x} fmt='{}'",
        buffer,
        format
    );

    emu.maps.write_wide_string(buffer, &format);
    emu.regs_mut().rax = format.len() as u64;
}

fn AdjustWindowRect(emu: &mut emu::Emu) {
    let lprect = emu.regs().rcx;
    let style = emu.regs().rdx;
    let bmenu = emu.regs().r8;
    log_red!(
        emu,
        "user32!AdjustWindowRect rect:0x{:x} style:0x{:x} menu:{}",
        lprect,
        style,
        bmenu
    );
    if lprect != 0 && emu.maps.is_mapped(lprect) {
        let left = emu.maps.read_dword(lprect).unwrap_or(0) as i32 - 8;
        let top = emu.maps.read_dword(lprect + 4).unwrap_or(0) as i32 - 30;
        let right = emu.maps.read_dword(lprect + 8).unwrap_or(0) as i32 + 8;
        let bottom = emu.maps.read_dword(lprect + 12).unwrap_or(0) as i32 + 8;
        emu.maps.write_dword(lprect, left as u32);
        emu.maps.write_dword(lprect + 4, top as u32);
        emu.maps.write_dword(lprect + 8, right as u32);
        emu.maps.write_dword(lprect + 12, bottom as u32);
    }
    emu.regs_mut().rax = 1;
}

fn AdjustWindowRectEx(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "user32!AdjustWindowRectEx arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 1;
}

fn CreateWindowExA(emu: &mut emu::Emu) {
    create_window(emu, "CreateWindowExA", false);
}

fn CreateWindowExW(emu: &mut emu::Emu) {
    create_window(emu, "CreateWindowExW", true);
}

/// hWndParent is CreateWindowEx's 9th argument.
const CREATE_WINDOW_PARENT_OFF: u64 = 0x40;

/// Shared body of CreateWindowEx{A,W}: records the window, HWND in rax.
fn create_window(emu: &mut emu::Emu, api: &str, wide: bool) {
    let (class_ptr, title_ptr) = (emu.regs().rdx, emu.regs().r8);
    let parent = emu
        .maps
        .read_qword(emu.regs().rsp + CREATE_WINDOW_PARENT_OFF)
        .unwrap_or(0);
    let class = class_arg(emu, class_ptr, wide).unwrap_or_default();
    let title = title_arg(emu, title_ptr, wide).unwrap_or_default();
    let hwnd = window_table::create_window(parent, &class, &title);
    log_red!(
        emu,
        "user32!{} class:'{}' title:'{}' parent:0x{:x} = 0x{:x}",
        api,
        class,
        title,
        parent,
        hwnd
    );
    emu.regs_mut().rax = hwnd;
}

fn ShowWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!ShowWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn UpdateWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!UpdateWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn DestroyWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!DestroyWindow hwnd:0x{:x}", hwnd);
    if helper::handler_exist(hwnd) {
        helper::handler_close(hwnd);
    }
    emu.regs_mut().rax = 1;
    if window_table::destroy_window(hwnd) {
        emu.regs_mut().rax = 1;
    } else {
        emu.set_last_error(constants::ERROR_INVALID_WINDOW_HANDLE);
        emu.regs_mut().rax = 0;
    }
}

fn PostMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageA hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx,
        emu.regs().r8,
        emu.regs().r9
    );
    emu.regs_mut().rax = 1;
}

fn PostMessageW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageW hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx,
        emu.regs().r8,
        emu.regs().r9
    );
    emu.regs_mut().rax = 1;
}

fn SendMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SendMessageA hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx,
        emu.regs().r8,
        emu.regs().r9
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

fn SendMessageW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SendMessageW hwnd:0x{:x} msg:0x{:x} wParam:0x{:x} lParam:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx,
        emu.regs().r8,
        emu.regs().r9
    );
    emu.regs_mut().rax = 0;
}

fn DefWindowProcA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!DefWindowProcA hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 0;
}

fn DefWindowProcW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!DefWindowProcW hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 0;
}

fn GetWindowRect(emu: &mut emu::Emu) {
    let rect = emu.regs().rdx;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "user32!GetWindowRect");
    emu.regs_mut().rax = 1;
}

fn GetClientRect(emu: &mut emu::Emu) {
    let rect = emu.regs().rdx;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1904);
        emu.maps.write_dword(rect + 12, 1041);
    }
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "user32!GetClientRect arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = 1;
}

fn SetWindowPos(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let arg6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let arg7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
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
}

fn SetWindowTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(emu.regs().rdx);
    log_red!(emu, "user32!SetWindowTextA '{}'", text);
    emu.regs_mut().rax = 1;
}

fn SetWindowTextW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(emu.regs().rdx);
    log_red!(emu, "user32!SetWindowTextW '{}'", text);
    emu.regs_mut().rax = 1;
}

fn GetWindowTextA(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "user32!GetWindowTextA hwnd=0x{:x} buf=0x{:x} max={}",
        hwnd,
        buf,
        max_count
    );
    if buf != 0 && max_count > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.regs_mut().rax = 0;
}

fn GetWindowTextW(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let buf = emu.regs().rdx;
    let max_count = emu.regs().r8 as usize;
    log_red!(
        emu,
        "user32!GetWindowTextW hwnd=0x{:x} buf=0x{:x} max={}",
        hwnd,
        buf,
        max_count
    );
    if buf != 0 && max_count > 0 {
        emu.maps.write_word(buf, 0);
    }
    emu.regs_mut().rax = 0;
}

fn get_window_long_value(index: i32) -> u64 {
    match index {
        -16 => 0x00CF0000, // GWL_STYLE -> WS_OVERLAPPEDWINDOW
        -20 => 0x00000100, // GWL_EXSTYLE -> WS_EX_WINDOWEDGE
        -6 => 0,           // GWL_HINSTANCE
        -12 => 0,          // GWL_ID
        -21 => 0,          // GWL_USERDATA
        -4 => 0,           // GWL_WNDPROC
        _ => 0,
    }
}

fn GetWindowLongA(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let result = get_window_long_value(idx);
    log_red!(emu, "user32!GetWindowLongA idx:{} => 0x{:x}", idx, result);
    emu.regs_mut().rax = result;
}

fn GetWindowLongW(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let result = get_window_long_value(idx);
    log_red!(emu, "user32!GetWindowLongW idx:{} => 0x{:x}", idx, result);
    emu.regs_mut().rax = result;
}

fn SetWindowLongA(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(emu, "user32!SetWindowLongA idx:{} new:0x{:x}", idx, new_val);
    emu.regs_mut().rax = get_window_long_value(idx);
}

fn SetWindowLongW(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(emu, "user32!SetWindowLongW idx:{} new:0x{:x}", idx, new_val);
    emu.regs_mut().rax = get_window_long_value(idx);
}

fn GetWindowLongPtrA(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let result = get_window_long_value(idx);
    log_red!(
        emu,
        "user32!GetWindowLongPtrA idx:{} => 0x{:x}",
        idx,
        result
    );
    emu.regs_mut().rax = result;
}

fn GetWindowLongPtrW(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let result = get_window_long_value(idx);
    log_red!(
        emu,
        "user32!GetWindowLongPtrW idx:{} => 0x{:x}",
        idx,
        result
    );
    emu.regs_mut().rax = result;
}

fn SetWindowLongPtrA(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(
        emu,
        "user32!SetWindowLongPtrA idx:{} new:0x{:x}",
        idx,
        new_val
    );
    emu.regs_mut().rax = get_window_long_value(idx);
}

fn SetWindowLongPtrW(emu: &mut emu::Emu) {
    let idx = emu.regs().rdx as i32;
    let new_val = emu.regs().r8;
    log_red!(
        emu,
        "user32!SetWindowLongPtrW idx:{} new:0x{:x}",
        idx,
        new_val
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = get_window_long_value(idx);
}

fn PeekMessageA(emu: &mut emu::Emu) {
    let msg_ptr = emu.regs().rcx;
    let hwnd = emu.regs().rdx;
    let msg_filter_min = emu.regs().r8;
    let msg_filter_max = emu.regs().r9;
    let remove_msg = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if msg_ptr != 0 {
        for i in 0..7u64 {
            emu.maps.write_qword(msg_ptr + i * 8, 0);
        }
    }
    log_red!(
        emu,
        "user32!PeekMessageA msg:0x{:x} hwnd:0x{:x} min:{} max:{} rm:0x{:x}",
        msg_ptr,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0;
}

fn PeekMessageW(emu: &mut emu::Emu) {
    let msg_ptr = emu.regs().rcx;
    let hwnd = emu.regs().rdx;
    let msg_filter_min = emu.regs().r8;
    let msg_filter_max = emu.regs().r9;
    let remove_msg = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if msg_ptr != 0 {
        for i in 0..7u64 {
            emu.maps.write_qword(msg_ptr + i * 8, 0);
        }
    }
    log_red!(
        emu,
        "user32!PeekMessageW msg:0x{:x} hwnd:0x{:x} min:{} max:{} rm:0x{:x}",
        msg_ptr,
        hwnd,
        msg_filter_min,
        msg_filter_max,
        remove_msg
    );
    emu.regs_mut().rax = 0;
}

fn GetMessageA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "user32!GetMessageA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 0;
}

fn GetMessageW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    log_red!(
        emu,
        "user32!GetMessageW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x} arg4:0x{:x}",
        arg1,
        arg2,
        arg3,
        arg4
    );
    emu.regs_mut().rax = 0;
}

fn TranslateMessage(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!TranslateMessage arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0;
}

fn DispatchMessageA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!DispatchMessageA arg1:0x{:x}", arg1);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

fn DispatchMessageW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!DispatchMessageW arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0;
}

fn PostQuitMessage(emu: &mut emu::Emu) {
    let exit_code = emu.regs().rcx as i32;
    log_red!(emu, "user32!PostQuitMessage code:{}", exit_code);
}

fn IsWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn IsWindowVisible(emu: &mut emu::Emu) {
    log_red!(emu, "user32!IsWindowVisible hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn EnableWindow(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EnableWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 0;
}

fn SetFocus(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetFocus hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = emu.regs().rcx;
}

fn GetFocus(emu: &mut emu::Emu) {
    let hwnd = helper::handler_create("hwnd://focus");
    log_red!(emu, "user32!GetFocus => 0x{:x}", hwnd);
    emu.regs_mut().rax = hwnd;
}

fn GetForegroundWindow(emu: &mut emu::Emu) {
    let hwnd = helper::handler_create("hwnd://foreground");
    log_red!(emu, "user32!GetForegroundWindow => 0x{:x}", hwnd);
    emu.regs_mut().rax = hwnd;
}

fn SetForegroundWindow(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "user32!SetForegroundWindow hwnd:0x{:x}", hwnd);
    if helper::handler_exist(hwnd) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}

fn GetCursorPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetCursorPos");
    write_cursor_pos(emu);
}

/// Same point as GetCursorPos: the emulated desktop has no DPI scaling.
fn GetPhysicalCursorPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetPhysicalCursorPos");
    write_cursor_pos(emu);
}

/// POINT at rcx = the centre of the 1920x1080 desktop; TRUE in rax.
fn write_cursor_pos(emu: &mut emu::Emu) {
    let pt = emu.regs().rcx;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    emu.regs_mut().rax = 1;
}

fn SetCursorPos(emu: &mut emu::Emu) {
    let x = emu.regs().rcx;
    let y = emu.regs().rdx;
    log_red!(emu, "user32!SetCursorPos x:{} y:{}", x, y);
    emu.regs_mut().rax = 1;
}

fn SetCursor(emu: &mut emu::Emu) {
    let hcursor = emu.regs().rcx;
    log_red!(emu, "user32!SetCursor hCursor:0x{:x}", hcursor);
    emu.regs_mut().rax = hcursor;
}

fn ShowCursor(emu: &mut emu::Emu) {
    let bshow = emu.regs().rcx;
    log_red!(emu, "user32!ShowCursor bShow:{}", bshow);
    emu.regs_mut().rax = if bshow != 0 { 1 } else { 0u64.wrapping_sub(1) };
}

fn LoadStringA(emu: &mut emu::Emu) {
    let _hinst = emu.regs().rcx;
    let id = emu.regs().rdx;
    let buf = emu.regs().r8;
    let buf_max = emu.regs().r9 as usize;
    log_red!(emu, "user32!LoadStringA id:{} buf:0x{:x}", id, buf);
    if buf != 0 && buf_max > 0 {
        emu.maps.write_byte(buf, 0);
    }
    emu.regs_mut().rax = 0;
}

fn LoadStringW(emu: &mut emu::Emu) {
    let _hinst = emu.regs().rcx;
    let id = emu.regs().rdx;
    let buf = emu.regs().r8;
    let buf_max = emu.regs().r9 as usize;
    log_red!(emu, "user32!LoadStringW id:{} buf:0x{:x}", id, buf);
    if buf != 0 && buf_max > 0 {
        emu.maps.write_word(buf, 0);
    }
    emu.regs_mut().rax = 0;
}

fn GetClassInfoExA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "user32!GetClassInfoExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.regs_mut().rax = 0;
}

fn GetClassInfoExW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "user32!GetClassInfoExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}

fn map_virtual_key(code: u32, map_type: u32) -> u64 {
    match map_type {
        0 => {
            // MAPVK_VK_TO_VSC: virtual key -> scan code
            match code {
                0x08 => 0x0E,                               // VK_BACK
                0x09 => 0x0F,                               // VK_TAB
                0x0D => 0x1C,                               // VK_RETURN
                0x1B => 0x01,                               // VK_ESCAPE
                0x20 => 0x39,                               // VK_SPACE
                0x41..=0x5A => (code - 0x41 + 0x1E) as u64, // A-Z approx
                0x30..=0x39 => {
                    // 0-9
                    let sc = [0x0B, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
                    sc[(code - 0x30) as usize] as u64
                }
                _ => 0,
            }
        }
        1 => {
            // MAPVK_VSC_TO_VK: scan code -> virtual key
            match code {
                0x01 => 0x1B, // ESC
                0x0E => 0x08, // BACKSPACE
                0x0F => 0x09, // TAB
                0x1C => 0x0D, // ENTER
                0x39 => 0x20, // SPACE
                _ => 0,
            }
        }
        2 if (0x30..=0x39).contains(&code) || (0x41..=0x5A).contains(&code) => code as u64,
        2 => 0,
        _ => 0,
    }
}

fn MapVirtualKeyA(emu: &mut emu::Emu) {
    let code = emu.regs().rcx as u32;
    let map_type = emu.regs().rdx as u32;
    let result = map_virtual_key(code, map_type);
    log_red!(
        emu,
        "user32!MapVirtualKeyA code:0x{:x} type:{} => 0x{:x}",
        code,
        map_type,
        result
    );
    emu.regs_mut().rax = result;
}

fn MapVirtualKeyW(emu: &mut emu::Emu) {
    let code = emu.regs().rcx as u32;
    let map_type = emu.regs().rdx as u32;
    let result = map_virtual_key(code, map_type);
    log_red!(
        emu,
        "user32!MapVirtualKeyW code:0x{:x} type:{} => 0x{:x}",
        code,
        map_type,
        result
    );
    emu.regs_mut().rax = result;
}

fn GetKeyboardLayout(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    log_red!(emu, "user32!GetKeyboardLayout arg1:0x{:x}", arg1);
    emu.regs_mut().rax = 0x0409_0409;
}

fn GetKeyboardType(emu: &mut emu::Emu) {
    let ntype = emu.regs().rcx as i32;
    let result: u64 = match ntype {
        0 => 4,
        1 => 0,
        2 => 12,
        _ => 0,
    };
    log_red!(emu, "user32!GetKeyboardType type:{} = {}", ntype, result);
    emu.regs_mut().rax = result;
}

fn BeginPaint(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    let ps_ptr = emu.regs().rdx;
    let hdc = helper::handler_create("hdc://paint");
    if ps_ptr != 0 {
        // PAINTSTRUCT: hdc(8) + fErase(4) + rcPaint(16) + ...
        emu.maps.write_qword(ps_ptr, hdc);
        emu.maps.write_dword(ps_ptr + 8, 0); // fErase = FALSE
        // rcPaint: left, top, right, bottom
        emu.maps.write_dword(ps_ptr + 12, 0);
        emu.maps.write_dword(ps_ptr + 16, 0);
        emu.maps.write_dword(ps_ptr + 20, 1920);
        emu.maps.write_dword(ps_ptr + 24, 1080);
    }
    log_red!(
        emu,
        "user32!BeginPaint hwnd:0x{:x} => hdc:0x{:x}",
        hwnd,
        hdc
    );
    emu.regs_mut().rax = hdc;
}

fn EndPaint(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EndPaint");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

fn InvalidateRect(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "user32!InvalidateRect arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.regs_mut().rax = 1;
}

fn FillRect(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "user32!FillRect arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.regs_mut().rax = 1;
}

fn DrawTextA(emu: &mut emu::Emu) {
    let text = emu.maps.read_string(emu.regs().rdx);
    log_red!(emu, "user32!DrawTextA '{}'", text);
    emu.regs_mut().rax = 20;
}

fn DrawTextW(emu: &mut emu::Emu) {
    let text = emu.maps.read_wide_string(emu.regs().rdx);
    log_red!(emu, "user32!DrawTextW '{}'", text);
    emu.regs_mut().rax = 20;
}
