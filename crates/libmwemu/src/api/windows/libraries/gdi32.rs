use crate::api::dispatch_api;
use crate::winapi::libraries::unsupported::gdi32 as unsupported;
mod context;
mod objects;
mod text;

pub use context::{
    ExcludeClipRect, GetClipBox, GetDCBrushColor, GetDCPenColor, GetDeviceCaps, GetNearestColor,
    GetStretchBltMode, GetWorldTransform, IntersectClipRect, SelectClipRgn, SetDCBrushColor,
    SetDCPenColor, SetStretchBltMode, SetWorldTransform,
};
pub use objects::{
    CreateBrushIndirect, CreateCompatibleBitmap, CreateFontA, CreateFontIndirectA,
    CreateFontIndirectW, CreateFontW, CreateHatchBrush, CreatePatternBrush, CreatePen,
    CreatePenIndirect, CreateRectRgn, CreateRectRgnIndirect, CreateSolidBrush, DeleteObject,
    GetCurrentObject, GetObjectA, GetObjectType, GetObjectW, GetStockObject, GetTextFaceA,
    GetTextFaceW, SelectObject,
};
pub(crate) use objects::{GraphicHandle, stock_objects};
pub use text::{
    GetCharABCWidthsA, GetCharABCWidthsW, GetCharWidth32A, GetCharWidth32W, GetTextExtentPoint32A,
    GetTextExtentPoint32W, GetTextMetricsA, GetTextMetricsW,
};

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::trace;
use crate::winapi::{
    common::{
        memory::{read_pointer, write_pointer},
        strings,
    },
    helper,
    policy::{self, api_try},
};
use crate::windows::constants::errors::ERROR_INVALID_HANDLE;
use serde::{Deserialize, Serialize};

const MM_TEXT: u32 = 1;
const MM_ANISOTROPIC: u32 = 8;
const ALTERNATE: u32 = 1;
const WINDING: u32 = 2;
const R2_BLACK: u32 = 1;
const R2_COPYPEN: u32 = 13;
const R2_WHITE: u32 = 16;
const TA_MASK: u32 = 0x11f;
const LAYOUT_MASK: u32 = 0xf;

const CLR_INVALID: u32 = u32::MAX;
const RGB_WHITE: u32 = 0x00ff_ffff;
const PAINTSTRUCT64_BYTES: u64 = 72;
const PAINTSTRUCT32_BYTES: u64 = 64;
const TRANSPARENT: u32 = 1;
const OPAQUE: u32 = 2;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Attributes {
    background: u32,
    text: u32,
    mode: u32,
    mapping: u32,
    fill: u32,
    raster_op: u32,
    alignment: u32,
    layout: u32,
    position: [i32; 2],
    viewport: [i32; 2],
    origin: [i32; 2],
    selected: [u64; 5],
    clip: Option<[i32; 4]>,
    brush: u32,
    pen: u32,
    stretch: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct DeviceContext {
    pub(crate) window: Option<u64>,
    memory: bool,
    colors: Attributes,
    saved: Vec<Attributes>,
}

impl DeviceContext {
    fn new(window: Option<u64>, memory: bool) -> Self {
        Self {
            window,
            memory,
            colors: Attributes {
                background: RGB_WHITE,
                text: 0,
                mode: OPAQUE,
                mapping: MM_TEXT,
                fill: ALTERNATE,
                raster_op: R2_COPYPEN,
                alignment: 0,
                layout: 0,
                position: [0; 2],
                viewport: [0; 2],
                origin: [0; 2],
                selected: [0; 5],
                clip: None,
                brush: RGB_WHITE,
                pen: 0,
                stretch: 1,
            },
            saved: Vec::new(),
        }
    }
}

impl DeviceContext {
    pub(crate) fn holds_graphic(&self, object: u64) -> bool {
        self.colors.selected.contains(&object)
            || self
                .saved
                .iter()
                .any(|state| state.selected.contains(&object))
    }
}

fn window_dc(emu: &mut Emu, window: u64) -> u64 {
    if window == 0
        || emu.guest_windows.contains(window)
        || helper::handler_get_uri(window) == "desktop_window"
    {
        emu.handle_management
            .insert_dc(DeviceContext::new(Some(window), false))
    } else {
        0
    }
}

fn acquire_window_dc(emu: &mut Emu, name: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let window = api_try!(emu, call.arg(emu, 0));
    if extended {
        let region = api_try!(emu, call.arg(emu, 1));
        let flags = api_try!(emu, call.arg_u32(emu, 2));
        if region != 0 || flags != 0 {
            policy::unsupported(emu, "user32", name, emu.pc());
            return;
        }
    }
    let dc = window_dc(emu, window);
    log_red!(emu, "user32!{name} window={window:#x}");
    call.set_return(emu, dc);
    call.cleanup_args(emu, if extended { 3 } else { 1 });
}

pub fn GetDC(emu: &mut Emu) {
    acquire_window_dc(emu, "GetDC", false);
}

pub fn GetWindowDC(emu: &mut Emu) {
    acquire_window_dc(emu, "GetWindowDC", false);
}

pub fn GetDCEx(emu: &mut Emu) {
    acquire_window_dc(emu, "GetDCEx", true);
}

pub fn BeginPaint(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let window = api_try!(emu, call.arg(emu, 0));
    let out = api_try!(emu, call.arg(emu, 1));
    let size = if emu.cfg.arch.is_64bits() {
        PAINTSTRUCT64_BYTES
    } else {
        PAINTSTRUCT32_BYTES
    };
    let dc = if window != 0 && emu.maps.validate_write_range(out, size) {
        window_dc(emu, window)
    } else {
        0
    };
    if dc != 0 {
        emu.maps.fill_range(out, 0, size as usize);
        write_pointer(emu, out, dc);
    }
    log_red!(emu, "user32!BeginPaint window={window:#x}");
    call.set_return(emu, dc);
    call.cleanup_args(emu, 2);
}

pub fn EndPaint(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let window = api_try!(emu, call.arg(emu, 0));
    let ps = api_try!(emu, call.arg(emu, 1));
    let ok = read_pointer(emu, ps)
        .is_some_and(|dc_handle| emu.handle_management.release_dc(dc_handle, Some(window)));
    log_red!(emu, "user32!EndPaint window={window:#x}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 2);
}

pub fn CreateDCA(emu: &mut Emu) {
    create_display_dc(emu, "CreateDCA", false);
}

pub fn CreateDCW(emu: &mut Emu) {
    create_display_dc(emu, "CreateDCW", true);
}

fn create_display_dc(emu: &mut Emu, name: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let driver = api_try!(emu, call.arg(emu, 0));
    let device = api_try!(emu, call.arg(emu, 1));
    let mode = api_try!(emu, call.arg(emu, 3));
    let display = |p| p != 0 && strings::read_text(emu, p, wide).eq_ignore_ascii_case("DISPLAY");
    if mode != 0 || !display(driver) && !display(device) {
        policy::unsupported(emu, "gdi32", name, emu.pc());
        return;
    }
    let dc = emu
        .handle_management
        .insert_dc(DeviceContext::new(None, false));
    log_red!(emu, "gdi32!{name} display");
    call.set_return(emu, dc);
    call.cleanup_args(emu, 4);
}

pub fn ReleaseDC(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [window, dc] = api_try!(emu, call.args(emu));
    let released = emu.handle_management.release_dc(dc, Some(window));
    log_red!(emu, "user32!ReleaseDC dc={dc:#x}");
    call.set_return(emu, released.into());
    call.cleanup_args(emu, 2);
}

pub fn CreateCompatibleDC(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let dc = if source == 0 || emu.handle_management.dc(source).is_some() {
        emu.handle_management
            .insert_dc(DeviceContext::new(None, true))
    } else {
        0
    };
    log_red!(emu, "gdi32!CreateCompatibleDC source={source:#x}");
    call.set_return(emu, dc);
    call.cleanup_args(emu, 1);
}

pub fn DeleteDC(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc_handle = api_try!(emu, call.arg(emu, 0));
    let released = emu.handle_management.release_dc(dc_handle, None);
    log_red!(emu, "gdi32!DeleteDC dc={dc_handle:#x}");
    call.set_return(emu, released.into());
    call.cleanup_args(emu, 1);
}

#[derive(Clone, Copy)]
enum Attribute {
    Brush,
    Pen,
    Stretch,
    Background,
    Text,
    Mode,
    Mapping,
    Fill,
    RasterOp,
    Alignment,
    Layout,
}

impl Attribute {
    fn accepts(self, value: u32) -> bool {
        match self {
            Self::Mode => matches!(value, TRANSPARENT | OPAQUE),
            Self::Mapping => (MM_TEXT..=MM_ANISOTROPIC).contains(&value),
            Self::Fill => matches!(value, ALTERNATE | WINDING),
            Self::RasterOp => (R2_BLACK..=R2_WHITE).contains(&value),
            Self::Alignment => value & !TA_MASK == 0,
            Self::Layout => value & !LAYOUT_MASK == 0,
            Self::Stretch => (1..=4).contains(&value),
            Self::Brush | Self::Pen | Self::Text | Self::Background => true,
        }
    }

    fn failure(self) -> u32 {
        match self {
            Self::Brush
            | Self::Pen
            | Self::Text
            | Self::Background
            | Self::Alignment
            | Self::Layout => CLR_INVALID,
            _ => 0,
        }
    }
}

fn attribute(emu: &mut Emu, api: &str, attribute: Attribute, setter: bool) {
    let call = WinApi(Stdcall);
    let dc_handle = api_try!(emu, call.arg(emu, 0));
    let value = if setter {
        Some(api_try!(emu, call.arg_u32(emu, 1)))
    } else {
        None
    };
    let result = if let Some(dc) = emu.handle_management.dc(dc_handle) {
        let slot = match attribute {
            Attribute::Mapping => &mut dc.colors.mapping,
            Attribute::Fill => &mut dc.colors.fill,
            Attribute::RasterOp => &mut dc.colors.raster_op,
            Attribute::Alignment => &mut dc.colors.alignment,
            Attribute::Layout => &mut dc.colors.layout,
            Attribute::Brush => &mut dc.colors.brush,
            Attribute::Pen => &mut dc.colors.pen,
            Attribute::Stretch => &mut dc.colors.stretch,
            Attribute::Mode => &mut dc.colors.mode,
            Attribute::Text => &mut dc.colors.text,
            Attribute::Background => &mut dc.colors.background,
        };
        match value {
            Some(v) if !attribute.accepts(v) => attribute.failure(),
            Some(v) => std::mem::replace(slot, v),
            None => *slot,
        }
    } else {
        attribute.failure()
    };
    log_red!(emu, "gdi32!{api} dc={dc_handle:#x}");
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 1 + usize::from(setter));
}

pub fn SaveDC(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc_handle = api_try!(emu, call.arg(emu, 0));
    let result = emu.handle_management.dc(dc_handle).map_or(0, |dc| {
        dc.saved.push(dc.colors);
        dc.saved.len()
    });
    log_red!(emu, "gdi32!SaveDC dc={dc_handle:#x}");
    call.set_return(emu, result as u64);
    call.cleanup_args(emu, 1);
}

pub fn RestoreDC(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc_handle = api_try!(emu, call.arg(emu, 0));
    let level = api_try!(emu, call.arg_i32(emu, 1));
    let ok = emu.handle_management.dc(dc_handle).is_some_and(|dc| {
        let idx = if level < 0 {
            dc.saved.len().checked_sub(level.unsigned_abs() as usize)
        } else {
            (level as usize).checked_sub(1)
        };
        let Some((i, colors)) = idx.and_then(|i| dc.saved.get(i).copied().map(|c| (i, c))) else {
            return false;
        };
        dc.colors = colors;
        dc.saved.truncate(i);
        true
    });
    log_red!(emu, "gdi32!RestoreDC dc={dc_handle:#x} level={level}");
    call.set_return(emu, ok.into());
    call.cleanup_args(emu, 2);
}

pub fn GetBkColor(emu: &mut Emu) {
    attribute(emu, "GetBkColor", Attribute::Background, false);
}

pub fn GetBkMode(emu: &mut Emu) {
    attribute(emu, "GetBkMode", Attribute::Mode, false);
}

pub fn GetTextColor(emu: &mut Emu) {
    attribute(emu, "GetTextColor", Attribute::Text, false);
}

pub fn SetBkColor(emu: &mut Emu) {
    attribute(emu, "SetBkColor", Attribute::Background, true);
}

pub fn SetBkMode(emu: &mut Emu) {
    attribute(emu, "SetBkMode", Attribute::Mode, true);
}

pub fn SetTextColor(emu: &mut Emu) {
    attribute(emu, "SetTextColor", Attribute::Text, true);
}

fn raster(emu: &mut Emu, api: &str, words: usize, failure: u32) {
    let call = WinApi(Stdcall);
    let dc = api_try!(emu, call.arg(emu, 0));
    if emu.handle_management.dc(dc).is_none() {
        trace::export(emu, call, "gdi32", api);
        emu.set_last_error(ERROR_INVALID_HANDLE);
        call.set_return(emu, failure.into());
        call.cleanup_args(emu, words);
        return;
    }
    // Device contexts track drawing attributes. Rendering requires a pixel surface.
    policy::unsupported(emu, "gdi32", api, emu.pc());
}

pub fn BitBlt(emu: &mut Emu) {
    raster(emu, "BitBlt", 9, 0);
}

pub fn TextOutA(emu: &mut Emu) {
    raster(emu, "TextOutA", 5, 0);
}

pub fn TextOutW(emu: &mut Emu) {
    raster(emu, "TextOutW", 5, 0);
}

pub fn GetPixelGdi(emu: &mut Emu) {
    raster(emu, "GetPixel", 3, CLR_INVALID);
}

pub fn SetPixel(emu: &mut Emu) {
    raster(emu, "SetPixel", 4, CLR_INVALID);
}

pub fn SetMapMode(emu: &mut Emu) {
    attribute(emu, "SetMapMode", Attribute::Mapping, true);
}

pub fn GetMapMode(emu: &mut Emu) {
    attribute(emu, "GetMapMode", Attribute::Mapping, false);
}

pub fn SetPolyFillMode(emu: &mut Emu) {
    attribute(emu, "SetPolyFillMode", Attribute::Fill, true);
}

pub fn GetPolyFillMode(emu: &mut Emu) {
    attribute(emu, "GetPolyFillMode", Attribute::Fill, false);
}

pub fn SetROP2(emu: &mut Emu) {
    attribute(emu, "SetROP2", Attribute::RasterOp, true);
}

pub fn GetROP2(emu: &mut Emu) {
    attribute(emu, "GetROP2", Attribute::RasterOp, false);
}

pub fn SetTextAlign(emu: &mut Emu) {
    attribute(emu, "SetTextAlign", Attribute::Alignment, true);
}

pub fn GetTextAlign(emu: &mut Emu) {
    attribute(emu, "GetTextAlign", Attribute::Alignment, false);
}

pub fn SetLayout(emu: &mut Emu) {
    attribute(emu, "SetLayout", Attribute::Layout, true);
}

pub fn GetLayout(emu: &mut Emu) {
    attribute(emu, "GetLayout", Attribute::Layout, false);
}

enum Origin {
    Pen,
    Window,
    Viewport,
}

fn set_origin(emu: &mut Emu, api: &str, which: Origin) {
    let call = WinApi(Stdcall);
    let dc_handle = api_try!(emu, call.arg(emu, 0));
    let x = api_try!(emu, call.arg_i32(emu, 1));
    let y = api_try!(emu, call.arg_i32(emu, 2));
    let previous = api_try!(emu, call.arg(emu, 3));
    log_red!(
        emu,
        "gdi32!{api} dc={dc_handle:#x} x={x} y={y} previous={previous:#x}"
    );
    let success = emu.handle_management.dc(dc_handle).is_some_and(|dc| {
        let position = match which {
            Origin::Pen => &mut dc.colors.position,
            Origin::Window => &mut dc.colors.origin,
            Origin::Viewport => &mut dc.colors.viewport,
        };
        if previous != 0 {
            let [old_x, old_y] = *position;
            if !emu
                .maps
                .validate_write_range(previous, size_of::<[i32; 2]>() as u64)
            {
                return false;
            }
            emu.maps.write_dword(previous, old_x as u32);
            emu.maps
                .write_dword(previous + size_of::<i32>() as u64, old_y as u32);
        }
        *position = [x, y];
        true
    });
    call.set_return(emu, success.into());
    call.cleanup_args(emu, 4);
}

pub fn MoveToEx(emu: &mut Emu) {
    set_origin(emu, "MoveToEx", Origin::Pen);
}
pub fn SetWindowOrgEx(emu: &mut Emu) {
    set_origin(emu, "SetWindowOrgEx", Origin::Window);
}
pub fn SetViewportOrgEx(emu: &mut Emu) {
    set_origin(emu, "SetViewportOrgEx", Origin::Viewport);
}

pub(crate) fn dispatch(api: &str, emu: &mut Emu) -> bool {
    dispatch_api!(api, {
        "AddFontResourceExA" => unsupported::AddFontResourceExA(emu),
        "AddFontResourceExW" => unsupported::AddFontResourceExW(emu),
        "Arc" => unsupported::Arc(emu),
        "BitBlt" => BitBlt(emu),
        "Chord" => unsupported::Chord(emu),
        "CombineRgn" => unsupported::CombineRgn(emu),
        "CreateBrushIndirect" => CreateBrushIndirect(emu),
        "CreateCompatibleBitmap" => CreateCompatibleBitmap(emu),
        "CreateCompatibleDC" => CreateCompatibleDC(emu),
        "CreateDCA" => CreateDCA(emu),
        "CreateDCW" => CreateDCW(emu),
        "CreateDIBSection" => unsupported::CreateDIBSection(emu),
        "CreateFontA" => CreateFontA(emu),
        "CreateFontIndirectA" => CreateFontIndirectA(emu),
        "CreateFontIndirectW" => CreateFontIndirectW(emu),
        "CreateFontW" => CreateFontW(emu),
        "CreateHatchBrush" => CreateHatchBrush(emu),
        "CreatePatternBrush" => CreatePatternBrush(emu),
        "CreatePen" => CreatePen(emu),
        "CreatePenIndirect" => CreatePenIndirect(emu),
        "CreateRectRgn" => CreateRectRgn(emu),
        "CreateRectRgnIndirect" => CreateRectRgnIndirect(emu),
        "CreateSolidBrush" => CreateSolidBrush(emu),
        "DeleteDC" => DeleteDC(emu),
        "DeleteObject" => DeleteObject(emu),
        "Ellipse" => unsupported::Ellipse(emu),
        "EndDoc" => unsupported::EndDoc(emu),
        "EndPage" => unsupported::EndPage(emu),
        "EnumFontFamiliesExA" => unsupported::EnumFontFamiliesExA(emu),
        "EnumFontFamiliesExW" => unsupported::EnumFontFamiliesExW(emu),
        "ExcludeClipRect" => ExcludeClipRect(emu),
        "ExtTextOutA" => unsupported::ExtTextOutA(emu),
        "ExtTextOutW" => unsupported::ExtTextOutW(emu),
        "GetBkColor" => GetBkColor(emu),
        "GetBkMode" => GetBkMode(emu),
        "GetCharABCWidthsA" => GetCharABCWidthsA(emu),
        "GetCharABCWidthsW" => GetCharABCWidthsW(emu),
        "GetCharWidth32A" => GetCharWidth32A(emu),
        "GetCharWidth32W" => GetCharWidth32W(emu),
        "GetClipBox" => GetClipBox(emu),
        "GetCurrentObject" => GetCurrentObject(emu),
        "GetDCBrushColor" => GetDCBrushColor(emu),
        "GetDCPenColor" => GetDCPenColor(emu),
        "GetDIBColorTable" => unsupported::GetDIBColorTable(emu),
        "GetDIBits" => unsupported::GetDIBits(emu),
        "GetDeviceCaps" => GetDeviceCaps(emu),
        "GetLayout" => GetLayout(emu),
        "GetMapMode" => GetMapMode(emu),
        "GetNearestColor" => GetNearestColor(emu),
        "GetObjectA" => GetObjectA(emu),
        "GetObjectType" => GetObjectType(emu),
        "GetObjectW" => GetObjectW(emu),
        "GetPixel" => GetPixelGdi(emu),
        "GetPolyFillMode" => GetPolyFillMode(emu),
        "GetROP2" => GetROP2(emu),
        "GetStockObject" => GetStockObject(emu),
        "GetStretchBltMode" => GetStretchBltMode(emu),
        "GetSystemPaletteEntries" => unsupported::GetSystemPaletteEntries(emu),
        "GetTextAlign" => GetTextAlign(emu),
        "GetTextColor" => GetTextColor(emu),
        "GetTextExtentPoint32A" => GetTextExtentPoint32A(emu),
        "GetTextExtentPoint32W" => GetTextExtentPoint32W(emu),
        "GetTextFaceA" => GetTextFaceA(emu),
        "GetTextFaceW" => GetTextFaceW(emu),
        "GetTextMetricsA" => GetTextMetricsA(emu),
        "GetTextMetricsW" => GetTextMetricsW(emu),
        "GetWorldTransform" => GetWorldTransform(emu),
        "IntersectClipRect" => IntersectClipRect(emu),
        "LineTo" => unsupported::LineTo(emu),
        "MoveToEx" => MoveToEx(emu),
        "PatBlt" => unsupported::PatBlt(emu),
        "Pie" => unsupported::Pie(emu),
        "PolyBezier" => unsupported::PolyBezier(emu),
        "Polygon" => unsupported::Polygon(emu),
        "Polyline" => unsupported::Polyline(emu),
        "Rectangle" => unsupported::Rectangle(emu),
        "RemoveFontResourceExA" => unsupported::RemoveFontResourceExA(emu),
        "RemoveFontResourceExW" => unsupported::RemoveFontResourceExW(emu),
        "RestoreDC" => RestoreDC(emu),
        "SaveDC" => SaveDC(emu),
        "SelectClipRgn" => SelectClipRgn(emu),
        "SelectObject" => SelectObject(emu),
        "SetBkColor" => SetBkColor(emu),
        "SetBkMode" => SetBkMode(emu),
        "SetDCBrushColor" => SetDCBrushColor(emu),
        "SetDCPenColor" => SetDCPenColor(emu),
        "SetDIBits" => unsupported::SetDIBits(emu),
        "SetLayout" => SetLayout(emu),
        "SetMapMode" => SetMapMode(emu),
        "SetPixel" => SetPixel(emu),
        "SetPolyFillMode" => SetPolyFillMode(emu),
        "SetROP2" => SetROP2(emu),
        "SetStretchBltMode" => SetStretchBltMode(emu),
        "SetTextAlign" => SetTextAlign(emu),
        "SetTextColor" => SetTextColor(emu),
        "SetViewportOrgEx" => SetViewportOrgEx(emu),
        "SetWindowOrgEx" => SetWindowOrgEx(emu),
        "SetWorldTransform" => SetWorldTransform(emu),
        "StartDocA" => unsupported::StartDocA(emu),
        "StartDocW" => unsupported::StartDocW(emu),
        "StartPage" => unsupported::StartPage(emu),
        "StretchBlt" => unsupported::StretchBlt(emu),
        "TextOutA" => TextOutA(emu),
        "TextOutW" => TextOutW(emu),
    },
        handled(()) => true,
        fallback => false,
    )
}
