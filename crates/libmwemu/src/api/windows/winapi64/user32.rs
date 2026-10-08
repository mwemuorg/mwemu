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
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_ref().unwrap(),
                    );
                }

                unimplemented!("atemmpt to call unimplemented API 0x{:x} {}", addr, api);
            }
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
    let nindex = emu.regs().rcx as usize;
    log_red!(emu, "user32!GetSystemMetrics nindex: {}", nindex);
    // TODO: do something
    emu.regs_mut().rax = 0;
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
