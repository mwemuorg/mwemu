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
        _ => {
            if !emu.cfg.skip_unimplemented {
                if emu.cfg.dump_on_exit && emu.cfg.dump_filename.is_some() {
                    serialization::Serialization::dump(
                        emu,
                        emu.cfg.dump_filename.as_deref().unwrap_or("mwemu_dump.bin"),
                    );
                }

                log::error!("attempt to call unimplemented API 0x{:x} {}", addr, api);
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
    let index = arg(emu, 0);
    log_red!(emu, "user32!GetSystemMetrics nIndex: {}", index);
    emu.regs_mut().rax = 0;
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
