use super::rectangles::Rect;
use super::window_state::{DESKTOP_WINDOW, SCREEN_HEIGHT, SCREEN_WIDTH, WORK_HEIGHT};
use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::err::MwemuError;
use crate::winapi::common::{memory::read_array, strings};
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::ERROR_NOACCESS;

const MONITOR: u64 = 0x0001_0001;
const MONITOR_DEFAULTTONULL: u32 = 0;
const MONITOR_DEFAULTTOPRIMARY: u32 = 1;
const MONITOR_DEFAULTTONEAREST: u32 = 2;
const MONITORINFOF_PRIMARY: u32 = 1;
const MONITORINFO_BYTES: u32 = 40;
const CCHDEVICENAME: u32 = 32;
const SPI_GETWORKAREA: u32 = 0x30;
const WS_BORDER: u32 = 0x0080_0000;
const WS_DLGFRAME: u32 = 0x0040_0000;
const WS_THICKFRAME: u32 = 0x0004_0000;
const WS_EX_CLIENTEDGE: u32 = 0x200;
const WS_EX_WINDOWEDGE: u32 = 0x100;
const CAPTION_HEIGHT: i32 = 23;
const MENU_HEIGHT: i32 = 19;

fn get_monitor(emu: &mut Emu, api: &str, point: bool) {
    let call = WinApi(Stdcall);
    let (intersects, flags) = if point {
        let coords = api_try!(emu, call.arg_u64(emu, 0));
        let x = coords as i32;
        let y = (coords >> 32) as i32;
        let flags = api_try!(
            emu,
            call.arg_u32(emu, if emu.cfg.arch.is_64bits() { 1 } else { 2 })
        );
        log_red!(emu, "user32!{api} x={x} y={y} flags={flags:#x}");
        (
            (0..SCREEN_WIDTH).contains(&x) && (0..SCREEN_HEIGHT).contains(&y),
            flags,
        )
    } else {
        let window = api_try!(emu, call.arg(emu, 0));
        let flags = api_try!(emu, call.arg_u32(emu, 1));
        log_red!(emu, "user32!{api} hwnd={window:#x} flags={flags:#x}");
        let intersects = window == DESKTOP_WINDOW
            || emu.guest_windows.get(window).is_some_and(|w| {
                let [x, y, width, height] = w.bounds;
                x < SCREEN_WIDTH
                    && y < SCREEN_HEIGHT
                    && i64::from(x) + i64::from(width) > 0
                    && i64::from(y) + i64::from(height) > 0
            });
        (intersects, flags)
    };
    let value = match flags {
        MONITOR_DEFAULTTONULL if !intersects => 0,
        MONITOR_DEFAULTTONULL | MONITOR_DEFAULTTOPRIMARY | MONITOR_DEFAULTTONEAREST => MONITOR,
        _ => 0,
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, if point { 3 } else { 2 });
}

fn monitor_info(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [monitor, output] = api_try!(emu, call.args(emu));
    let full = MONITORINFO_BYTES + CCHDEVICENAME * if wide { 2 } else { 1 };
    let size = emu.maps.read_dword(output);
    let ok = if monitor != MONITOR || output == 0 {
        false
    } else if let Some(size) = size.filter(|&size| size == MONITORINFO_BYTES || size == full) {
        let mut data = Vec::with_capacity(size as usize);
        data.extend(size.to_le_bytes());
        for v in [
            0,
            0,
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            0,
            0,
            SCREEN_WIDTH,
            WORK_HEIGHT,
        ] {
            data.extend(v.to_le_bytes());
        }
        data.extend(MONITORINFOF_PRIMARY.to_le_bytes());
        if size == full {
            data.extend(strings::encode_text(r"\\.\DISPLAY1", wide));
            data.resize(size as usize, 0);
        }
        emu.maps.validate_write_range(output, size.into()) && emu.maps.write_bytes(output, &data)
    } else {
        false
    };
    log_red!(
        emu,
        "user32!{api} monitor={monitor:#x} monitor_info={output:#x} success={ok}"
    );
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 2);
}

fn adjust(emu: &mut Emu, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let [rect, style, menu] = api_try!(emu, call.args(emu));
    let extra = if extended {
        api_try!(emu, call.arg_u32(emu, 3))
    } else {
        0
    };
    let style = style as u32;
    let frame = if style & WS_THICKFRAME != 0 {
        8
    } else if style & WS_DLGFRAME != 0 {
        3
    } else {
        i32::from(style & WS_BORDER != 0)
    };
    let border = frame
        + i32::from(extra & WS_EX_CLIENTEDGE != 0) * 2
        + i32::from(extra & WS_EX_WINDOWEDGE != 0);
    let top = border
        + if style & (WS_BORDER | WS_DLGFRAME) == WS_BORDER | WS_DLGFRAME {
            CAPTION_HEIGHT
        } else {
            0
        }
        + if menu != 0 { MENU_HEIGHT } else { 0 };
    let ok = read_array::<16>(emu, rect)
        .filter(|_| rect != 0)
        .is_some_and(|raw| {
            let (words, _) = raw.as_chunks::<4>();
            let [left, y, right, bottom] = std::array::from_fn(|i| i32::from_le_bytes(words[i]));
            let result = [
                left.wrapping_sub(border),
                y.wrapping_sub(top),
                right.wrapping_add(border),
                bottom.wrapping_add(border),
            ]
            .map(i32::to_le_bytes);
            emu.maps.validate_write_range(rect, 16)
                && emu.maps.write_bytes(rect, result.as_flattened())
        });
    log_red!(
        emu,
        "user32!{api} rect={rect:#x} style={style:#x} menu={menu} extended_style={extra:#x}"
    );
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 3 + usize::from(extended));
}

fn parameters(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [action, parameter, output, flags] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!{api} action={action:#x} parameter={parameter:#x} output={output:#x} flags={flags:#x}"
    );
    if action as u32 != SPI_GETWORKAREA {
        policy::unsupported(emu, "user32", api, emu.pc());
        return;
    }
    let values = [0, 0, SCREEN_WIDTH, WORK_HEIGHT].map(i32::to_le_bytes);
    let ok = output != 0
        && emu.maps.validate_write_range(output, 16)
        && emu.maps.write_bytes(output, values.as_flattened());
    if !ok {
        emu.set_last_error(ERROR_NOACCESS);
    }
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 4);
}

fn enumerate_primary_monitor(
    emu: &mut Emu,
    clip: u64,
    callback: u64,
    data: u64,
) -> Result<bool, MwemuError> {
    if callback == 0 {
        return Ok(false);
    }
    let mut bounds = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    if clip != 0 {
        let Some(clip) = Rect::read(emu, clip) else {
            return Ok(false);
        };
        bounds = bounds.intersection(clip);
        if bounds.empty() {
            return Ok(true);
        }
    }

    // Keep the RECT in this invocation's guest stack frame while the callback runs.
    let sp = emu.sp();
    let Some(rect) = sp.checked_sub(Rect::BYTE_LEN as u64) else {
        return Ok(false);
    };
    if !bounds.write(emu, rect) {
        return Ok(false);
    }
    emu.set_sp(rect);
    let args = [MONITOR, 0, rect, data];
    let result = if emu.cfg.arch.is_64bits() {
        emu.call64(callback, &args)? as u32
    } else {
        emu.call32(callback, &args.map(|arg| arg as u32))?
    };
    emu.set_sp(sp);
    Ok(result != 0)
}

pub fn EnumDisplayMonitors(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dc, clip, callback, data] = api_try!(emu, call.args(emu));
    log_red!(
        emu,
        "user32!EnumDisplayMonitors dc={dc:#x} clip={clip:#x} callback={callback:#x} data={data:#x}"
    );
    if dc != 0 {
        policy::unsupported(emu, "user32", "EnumDisplayMonitors", emu.pc());
        return;
    }
    match enumerate_primary_monitor(emu, clip, callback, data) {
        Ok(completed) => {
            call.set_return(emu, completed.into());
            call.cleanup_args(emu, 4);
        }
        Err(error) => {
            log::warn!("EnumDisplayMonitors callback {callback:#x}: {error}");
            emu.stop();
        }
    }
}

pub fn MonitorFromPoint(emu: &mut Emu) {
    get_monitor(emu, "MonitorFromPoint", true);
}

pub fn MonitorFromWindow(emu: &mut Emu) {
    get_monitor(emu, "MonitorFromWindow", false);
}

pub fn AdjustWindowRect(emu: &mut Emu) {
    adjust(emu, "AdjustWindowRect", false);
}

pub fn AdjustWindowRectEx(emu: &mut Emu) {
    adjust(emu, "AdjustWindowRectEx", true);
}

pub fn GetMonitorInfoA(emu: &mut Emu) {
    monitor_info(emu, "GetMonitorInfoA", false);
}

pub fn GetMonitorInfoW(emu: &mut Emu) {
    monitor_info(emu, "GetMonitorInfoW", true);
}

pub fn SystemParametersInfoA(emu: &mut Emu) {
    parameters(emu, "SystemParametersInfoA");
}

pub fn SystemParametersInfoW(emu: &mut Emu) {
    parameters(emu, "SystemParametersInfoW");
}

pub fn GetCaretBlinkTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    const DEFAULT_CARET_BLINK_MS: u64 = 530;
    log_red!(emu, "user32!GetCaretBlinkTime");
    call.set_return(emu, DEFAULT_CARET_BLINK_MS);
}

pub fn GetDoubleClickTime(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    const DEFAULT_DOUBLE_CLICK_MS: u64 = 500;
    log_red!(emu, "user32!GetDoubleClickTime");
    call.set_return(emu, DEFAULT_DOUBLE_CLICK_MS);
}

pub fn GetKeyboardType(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let kind = api_try!(emu, call.arg_i32(emu, 0));
    let value = match kind {
        0 => 4,
        2 => 12,
        _ => 0,
    };
    log_red!(
        emu,
        "user32!GetKeyboardType information={kind} value={value}"
    );
    call.set_return(emu, value);
    call.cleanup_args(emu, 1);
}

pub fn GetKeyboardLayout(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    const US_KEYBOARD_LAYOUT: u64 = 0x0409_0409;
    let thread = api_try!(emu, call.arg_u32(emu, 0));
    log_red!(
        emu,
        "user32!GetKeyboardLayout thread={thread} layout={US_KEYBOARD_LAYOUT:#x}"
    );
    call.set_return(emu, US_KEYBOARD_LAYOUT);
    call.cleanup_args(emu, 1);
}

const SM_CXSCREEN: i32 = 0;
const SM_CYSCREEN: i32 = 1;
const SM_CXVSCROLL: i32 = 2;
const SM_CYHSCROLL: i32 = 3;
const SM_CYCAPTION: i32 = 4;
const SM_CXBORDER: i32 = 5;
const SM_CYBORDER: i32 = 6;
const SM_CXDLGFRAME: i32 = 7;
const SM_CYDLGFRAME: i32 = 8;
const SM_CYVTHUMB: i32 = 9;
const SM_CXHTHUMB: i32 = 10;
const SM_CXICON: i32 = 11;
const SM_CYICON: i32 = 12;
const SM_CXCURSOR: i32 = 13;
const SM_CYCURSOR: i32 = 14;
const SM_CYMENU: i32 = 15;
const SM_CXFULLSCREEN: i32 = 16;
const SM_CYFULLSCREEN: i32 = 17;
const SM_CYVSCROLL: i32 = 20;
const SM_CXHSCROLL: i32 = 21;
const SM_CXMIN: i32 = 28;
const SM_CYMIN: i32 = 29;
const SM_CXSIZE: i32 = 30;
const SM_CYSIZE: i32 = 31;
const SM_CXFRAME: i32 = 32;
const SM_CYFRAME: i32 = 33;
const SM_CXMINTRACK: i32 = 34;
const SM_CYMINTRACK: i32 = 35;
const SM_CXDOUBLECLK: i32 = 36;
const SM_CYDOUBLECLK: i32 = 37;
const SM_CXICONSPACING: i32 = 38;
const SM_CYICONSPACING: i32 = 39;
const SM_CMOUSEBUTTONS: i32 = 43;
const SM_CYMINSPACING: i32 = 48;
const SM_CXSMICON: i32 = 49;
const SM_CYSMICON: i32 = 50;
const SM_CYSMCAPTION: i32 = 51;
const SM_CXSMSIZE: i32 = 52;
const SM_CYSMSIZE: i32 = 53;
const SM_CXMENUSIZE: i32 = 54;
const SM_CXMAXIMIZED: i32 = 61;
const SM_CYMAXIMIZED: i32 = 62;
const SM_NETWORK: i32 = 63;
const SM_XVIRTUALSCREEN: i32 = 76;
const SM_YVIRTUALSCREEN: i32 = 77;
const SM_CXVIRTUALSCREEN: i32 = 78;
const SM_CYVIRTUALSCREEN: i32 = 79;
const SM_CMONITORS: i32 = 80;

pub fn GetSystemMetrics(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg_i32(emu, 0));
    let result: u64 = match index {
        SM_CXSCREEN => 1920,
        SM_CYSCREEN => 1080,
        SM_CXVSCROLL => 20,
        SM_CYHSCROLL => 20,
        SM_CYCAPTION => 23,
        SM_CXBORDER => 1,
        SM_CYBORDER => 1,
        SM_CXDLGFRAME => 4,
        SM_CYDLGFRAME => 4,
        SM_CYVTHUMB => 20,
        SM_CXHTHUMB => 20,
        SM_CXICON => 32,
        SM_CYICON => 32,
        SM_CXCURSOR => 32,
        SM_CYCURSOR => 32,
        SM_CYMENU => 20,
        SM_CXFULLSCREEN => 1920,
        SM_CYFULLSCREEN => 1017,
        SM_CYVSCROLL => 17,
        SM_CXHSCROLL => 17,
        SM_CXMIN => 20,
        SM_CYMIN => 20,
        SM_CXSIZE => 22,
        SM_CYSIZE => 22,
        SM_CXFRAME => 8,
        SM_CYFRAME => 8,
        SM_CXMINTRACK => 112,
        SM_CYMINTRACK => 27,
        SM_CXDOUBLECLK => 4,
        SM_CYDOUBLECLK => 4,
        SM_CXICONSPACING => 75,
        SM_CYICONSPACING => 75,
        SM_CMOUSEBUTTONS => 1,
        SM_CYMINSPACING => 16,
        SM_CXSMICON => 16,
        SM_CYSMICON => 16,
        SM_CYSMCAPTION => 16,
        SM_CXSMSIZE => 16,
        SM_CYSMSIZE => 18,
        SM_CXMENUSIZE => 18,
        SM_CXMAXIMIZED => 1920,
        SM_CYMAXIMIZED => 1080,
        SM_NETWORK => 0,
        SM_XVIRTUALSCREEN => 0,
        SM_YVIRTUALSCREEN => 0,
        SM_CXVIRTUALSCREEN => 1920,
        SM_CYVIRTUALSCREEN => 1080,
        SM_CMONITORS => 1,
        _ => 0,
    };
    log_red!(emu, "user32!GetSystemMetrics index={index} value={result}");
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn GetSysColor(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg_i32(emu, 0));
    const COLOR_SCROLLBAR: i32 = 0;
    const COLOR_BACKGROUND: i32 = 1;
    const COLOR_ACTIVECAPTION: i32 = 2;
    const COLOR_MENU: i32 = 4;
    const COLOR_WINDOW: i32 = 5;
    const COLOR_WINDOWTEXT: i32 = 8;
    const COLOR_BTNFACE: i32 = 15;
    let color = match index {
        COLOR_SCROLLBAR | COLOR_MENU | COLOR_BTNFACE => 0xd4d0c8,
        COLOR_BACKGROUND => 0x004e98,
        COLOR_ACTIVECAPTION => 0x0a246a,
        COLOR_WINDOW => 0xffffff,
        COLOR_WINDOWTEXT => 0,
        _ => {
            policy::unsupported(emu, "user32", "GetSysColor", emu.pc());
            return;
        }
    };
    log_red!(emu, "user32!GetSysColor index={index} color={color:#x}");
    call.set_return(emu, color);
    call.cleanup_args(emu, 1);
}
