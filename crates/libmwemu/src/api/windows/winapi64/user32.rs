use crate::emu;
use crate::serialization;
use crate::winapi::helper;
use crate::winapi::winapi64;

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
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "CharLowerBuffW" => CharLowerBuffW(emu),
        "CharLowerBuffA" => CharLowerBuffA(emu),
        "CharUpperBuffW" => CharUpperBuffW(emu),
        "CharUpperBuffA" => CharUpperBuffA(emu),
        "FindWindowA" => FindWindowA(emu),
        "FindWindowW" => FindWindowW(emu),
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
        _ => {
            if emu.cfg.skip_unimplemented {
                log::warn!(
                    "banzai: skipping unimplemented API 0x{:x} {} at 0x{:x}",
                    addr,
                    api,
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
    let titleptr = emu.regs().rcx;
    let msgptr = emu.regs().rdx;
    let msg = emu.maps.read_string(msgptr);
    let title = emu.maps.read_string(titleptr);

    log_red!(emu, "user32!MessageBoxA {} {}", title, msg);

    emu.regs_mut().rax = 0;
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
    log_red!(emu, "user32!GetDesktopWindow");
    //emu.regs_mut().rax = 0x11223344; // current window handle
    emu.regs_mut().rax = 0; // no windows handler is more stealthy
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
    let ui_action = emu.regs().rcx;
    let ui_param = emu.regs().rdx;
    let pv_param = emu.regs().r8;
    let f_win_ini = emu.regs().r9;
    log_red!(
        emu,
        "** {} user32!SystemParametersInfoA {} {} {} {}",
        emu.pos,
        ui_action,
        ui_param,
        pv_param,
        f_win_ini
    );
    // TODO: write pvParam
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
        "** {} user32!LoadIconA {} {}",
        emu.pos,
        hinstance,
        lpiconname
    );
    // TODO: do not return null
    emu.regs_mut().rax = 0;
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
        "** {} user32!LoadCursorA {} {}",
        emu.pos,
        hinstance,
        lpcursorname
    );
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
ATOM RegisterClassA(
  [in] const WNDCLASSA *lpWndClass
);
*/
fn RegisterClassA(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "** {} user32!RegisterClassA {}", emu.pos, lpwndclass);
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
ATOM RegisterClassW(
  [in] const WNDCLASSW *lpWndClass
);
*/
fn RegisterClassW(emu: &mut emu::Emu) {
    let lpwndclass = emu.regs().rcx;
    log_red!(emu, "** {} user32!RegisterClassW {}", emu.pos, lpwndclass);
    // TODO: do not return null
    emu.regs_mut().rax = 0;
}

/*
HDC GetDC(
  [in] HWND hWnd
);
*/
fn GetDC(emu: &mut emu::Emu) {
    let hwnd = emu.regs().rcx;
    log_red!(emu, "** {} user32!GetDC {}", emu.pos, hwnd);
    // TODO: do something / do not return null
    emu.regs_mut().rax = 0;
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
    log_red!(emu, "** {} user32!ReleaseDC {} {}", emu.pos, hwnd, hdc);
    // TODO: do something
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
}

pub fn FindWindowW(emu: &mut emu::Emu) {
    let class_ptr = emu.regs().rcx;
    let window_ptr = emu.regs().rdx;

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

    log_red!(
        emu,
        "user32!wsprintfA buffer=0x{:x} fmt='{}'",
        buffer,
        format
    );

    emu.maps.write_string(buffer, &format);
    emu.regs_mut().rax = format.len() as u64;
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
    log_red!(emu, "user32!AdjustWindowRect");
    emu.regs_mut().rax = 1;
}

fn AdjustWindowRectEx(emu: &mut emu::Emu) {
    log_red!(emu, "user32!AdjustWindowRectEx");
    emu.regs_mut().rax = 1;
}

fn CreateWindowExA(emu: &mut emu::Emu) {
    let class = emu.maps.read_string(emu.regs().rdx);
    let title = emu.maps.read_string(emu.regs().r8);
    log_red!(
        emu,
        "user32!CreateWindowExA class:'{}' title:'{}'",
        class,
        title
    );
    emu.regs_mut().rax = 0x1337_0001;
}

fn CreateWindowExW(emu: &mut emu::Emu) {
    let class = emu.maps.read_wide_string(emu.regs().rdx);
    let title = emu.maps.read_wide_string(emu.regs().r8);
    log_red!(
        emu,
        "user32!CreateWindowExW class:'{}' title:'{}'",
        class,
        title
    );
    emu.regs_mut().rax = 0x1337_0001;
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
    log_red!(emu, "user32!DestroyWindow hwnd:0x{:x}", emu.regs().rcx);
    emu.regs_mut().rax = 1;
}

fn PostMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageA hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 1;
}

fn PostMessageW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!PostMessageW hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 1;
}

fn SendMessageA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SendMessageA hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
    );
    emu.regs_mut().rax = 0;
}

fn SendMessageW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SendMessageW hwnd:0x{:x} msg:0x{:x}",
        emu.regs().rcx,
        emu.regs().rdx
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
    log_red!(emu, "user32!GetClientRect");
    emu.regs_mut().rax = 1;
}

fn SetWindowPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowPos");
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
    log_red!(emu, "user32!GetWindowTextA");
    emu.regs_mut().rax = 0;
}

fn GetWindowTextW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowTextW");
    emu.regs_mut().rax = 0;
}

fn GetWindowLongA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowLongA idx:{}", emu.regs().rdx as i32);
    emu.regs_mut().rax = 0;
}

fn GetWindowLongW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetWindowLongW idx:{}", emu.regs().rdx as i32);
    emu.regs_mut().rax = 0;
}

fn SetWindowLongA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowLongA idx:{}", emu.regs().rdx as i32);
    emu.regs_mut().rax = 0;
}

fn SetWindowLongW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetWindowLongW idx:{}", emu.regs().rdx as i32);
    emu.regs_mut().rax = 0;
}

fn GetWindowLongPtrA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!GetWindowLongPtrA idx:{}",
        emu.regs().rdx as i32
    );
    emu.regs_mut().rax = 0;
}

fn GetWindowLongPtrW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!GetWindowLongPtrW idx:{}",
        emu.regs().rdx as i32
    );
    emu.regs_mut().rax = 0;
}

fn SetWindowLongPtrA(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SetWindowLongPtrA idx:{}",
        emu.regs().rdx as i32
    );
    emu.regs_mut().rax = 0;
}

fn SetWindowLongPtrW(emu: &mut emu::Emu) {
    log_red!(
        emu,
        "user32!SetWindowLongPtrW idx:{}",
        emu.regs().rdx as i32
    );
    emu.regs_mut().rax = 0;
}

fn PeekMessageA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!PeekMessageA");
    emu.regs_mut().rax = 0;
}

fn PeekMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!PeekMessageW");
    emu.regs_mut().rax = 0;
}

fn GetMessageA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetMessageA");
    emu.regs_mut().rax = 0;
}

fn GetMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetMessageW");
    emu.regs_mut().rax = 0;
}

fn TranslateMessage(emu: &mut emu::Emu) {
    log_red!(emu, "user32!TranslateMessage");
    emu.regs_mut().rax = 0;
}

fn DispatchMessageA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!DispatchMessageA");
    emu.regs_mut().rax = 0;
}

fn DispatchMessageW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!DispatchMessageW");
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
}

fn GetCursorPos(emu: &mut emu::Emu) {
    let pt = emu.regs().rcx;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    log_red!(emu, "user32!GetCursorPos");
    emu.regs_mut().rax = 1;
}

fn SetCursorPos(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetCursorPos");
    emu.regs_mut().rax = 1;
}

fn SetCursor(emu: &mut emu::Emu) {
    log_red!(emu, "user32!SetCursor");
    emu.regs_mut().rax = 0;
}

fn ShowCursor(emu: &mut emu::Emu) {
    log_red!(emu, "user32!ShowCursor");
    emu.regs_mut().rax = 0;
}

fn LoadStringA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!LoadStringA id:{}", emu.regs().rdx);
    emu.regs_mut().rax = 0;
}

fn LoadStringW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!LoadStringW id:{}", emu.regs().rdx);
    emu.regs_mut().rax = 0;
}

fn GetClassInfoExA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetClassInfoExA");
    emu.regs_mut().rax = 0;
}

fn GetClassInfoExW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetClassInfoExW");
    emu.regs_mut().rax = 0;
}

fn MapVirtualKeyA(emu: &mut emu::Emu) {
    log_red!(emu, "user32!MapVirtualKeyA");
    emu.regs_mut().rax = 0;
}

fn MapVirtualKeyW(emu: &mut emu::Emu) {
    log_red!(emu, "user32!MapVirtualKeyW");
    emu.regs_mut().rax = 0;
}

fn GetKeyboardLayout(emu: &mut emu::Emu) {
    log_red!(emu, "user32!GetKeyboardLayout");
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
    log_red!(emu, "user32!BeginPaint");
    emu.regs_mut().rax = 0x1337_0002;
}

fn EndPaint(emu: &mut emu::Emu) {
    log_red!(emu, "user32!EndPaint");
    emu.regs_mut().rax = 1;
}

fn InvalidateRect(emu: &mut emu::Emu) {
    log_red!(emu, "user32!InvalidateRect");
    emu.regs_mut().rax = 1;
}

fn FillRect(emu: &mut emu::Emu) {
    log_red!(emu, "user32!FillRect");
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
