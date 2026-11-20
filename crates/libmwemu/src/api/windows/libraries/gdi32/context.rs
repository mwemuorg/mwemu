use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::system::HORZRES;

use super::objects::GraphicObject;
use super::{Attribute, CLR_INVALID, attribute};

const DRIVERVERSION: u32 = 0;
const TECHNOLOGY: u32 = 2;
const HORZSIZE: u32 = 4;
const VERTSIZE: u32 = 6;
const VERTRES: u32 = 10;
const BITSPIXEL: u32 = 12;
const PLANES: u32 = 14;
const NUMBRUSHES: u32 = 16;
const NUMPENS: u32 = 18;
const NUMCOLORS: u32 = 24;
const ASPECTX: u32 = 40;
const ASPECTY: u32 = 42;
const ASPECTXY: u32 = 44;
const LOGPIXELSX: u32 = 88;
const LOGPIXELSY: u32 = 90;
const COLORRES: u32 = 108;
const VREFRESH: u32 = 116;
const DESKTOPVERTRES: u32 = 117;
const DESKTOPHORZRES: u32 = 118;

const VIRTUAL_WIDTH: i32 = 1920;
const VIRTUAL_HEIGHT: i32 = 1080;
const BITS_PER_PIXEL: u32 = 32;
const DT_RASDISPLAY: u32 = 1;
const NULLREGION: u32 = 1;
const SIMPLEREGION: u32 = 2;
const GDI_ERROR: u32 = 0;
const IDENTITY_TRANSFORM: [f32; 6] = [1., 0., 0., 1., 0., 0.];

pub fn SetDCBrushColor(emu: &mut Emu) {
    attribute(emu, "SetDCBrushColor", Attribute::Brush, true);
}
pub fn SetDCPenColor(emu: &mut Emu) {
    attribute(emu, "SetDCPenColor", Attribute::Pen, true);
}
pub fn GetDCBrushColor(emu: &mut Emu) {
    attribute(emu, "GetDCBrushColor", Attribute::Brush, false);
}
pub fn GetDCPenColor(emu: &mut Emu) {
    attribute(emu, "GetDCPenColor", Attribute::Pen, false);
}
pub fn SetStretchBltMode(emu: &mut Emu) {
    attribute(emu, "SetStretchBltMode", Attribute::Stretch, true);
}
pub fn GetStretchBltMode(emu: &mut Emu) {
    attribute(emu, "GetStretchBltMode", Attribute::Stretch, false);
}

pub fn GetDeviceCaps(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc = api_try!(emu, call.arg(emu, 0));
    let capability = api_try!(emu, call.arg_u32(emu, 1));
    let result = if emu.handle_management.dc(dc).is_none() {
        0
    } else {
        match capability {
            // The virtual display has one true-color plane and square pixels.
            DRIVERVERSION => 1,
            TECHNOLOGY => DT_RASDISPLAY,
            HORZSIZE => 508,
            VERTSIZE => 285,
            HORZRES | DESKTOPHORZRES => VIRTUAL_WIDTH as u32,
            VERTRES | DESKTOPVERTRES => VIRTUAL_HEIGHT as u32,
            BITSPIXEL | COLORRES => BITS_PER_PIXEL,
            PLANES | ASPECTX | ASPECTY | ASPECTXY => 1,
            NUMBRUSHES | NUMPENS | NUMCOLORS => u32::MAX,
            LOGPIXELSX | LOGPIXELSY => 96,
            VREFRESH => 60,
            _ => 0,
        }
    };
    log_red!(
        emu,
        "gdi32!GetDeviceCaps dc={dc:#x} capability={capability} result={result}"
    );
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 2);
}

pub fn GetNearestColor(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc = api_try!(emu, call.arg(emu, 0));
    let color = api_try!(emu, call.arg_u32(emu, 1));
    let nearest = if emu.handle_management.dc(dc).is_none() {
        CLR_INVALID
    } else if color >> 24 == 0 {
        color
    } else {
        policy::unsupported(emu, "gdi32", "GetNearestColor", emu.pc());
        return;
    };
    log_red!(
        emu,
        "gdi32!GetNearestColor dc={dc:#x} color={color:#x} nearest={nearest:#x}"
    );
    call.set_return(emu, nearest.into());
    call.cleanup_args(emu, 2);
}

fn write_rect(emu: &mut Emu, addr: u64, rect: [i32; 4]) -> bool {
    let bytes: Vec<_> = rect.into_iter().flat_map(i32::to_le_bytes).collect();
    emu.maps.write_bytes(addr, &bytes)
}

fn intersect(a: [i32; 4], b: [i32; 4]) -> [i32; 4] {
    let rect = [
        a[0].max(b[0]),
        a[1].max(b[1]),
        a[2].min(b[2]),
        a[3].min(b[3]),
    ];
    if rect[0] >= rect[2] || rect[1] >= rect[3] {
        [0; 4]
    } else {
        rect
    }
}

fn region_type(rect: [i32; 4]) -> u32 {
    if rect[0] >= rect[2] || rect[1] >= rect[3] {
        NULLREGION
    } else {
        SIMPLEREGION
    }
}

fn clip_box(emu: &mut Emu, dc: u64) -> Option<[i32; 4]> {
    let dc_state = emu.handle_management.dc(dc)?;
    let clip = dc_state.colors.clip;
    let bounds = if dc_state.memory {
        let selected = dc_state.colors.selected[3];
        if selected == 0 {
            [0, 0, 1, 1]
        } else {
            let GraphicObject::Bitmap { width, height, .. } =
                emu.handle_management.graphic(selected)?.object
            else {
                return None;
            };
            [0, 0, width, height]
        }
    } else {
        [0, 0, VIRTUAL_WIDTH, VIRTUAL_HEIGHT]
    };
    Some(clip.map_or(bounds, |rect| intersect(bounds, rect)))
}

pub fn GetClipBox(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dc, output] = api_try!(emu, call.args(emu));
    let result = clip_box(emu, dc)
        .filter(|rect| write_rect(emu, output, *rect))
        .map_or(GDI_ERROR, region_type);
    log_red!(
        emu,
        "gdi32!GetClipBox dc={dc:#x} bounds={output:#x} region_type={result}"
    );
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 2);
}

pub fn SelectClipRgn(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dc, region] = api_try!(emu, call.args(emu));
    let result = (|| {
        let clip = if region == 0 {
            None
        } else {
            let GraphicObject::Region(rect) = emu.handle_management.graphic(region)?.object else {
                return None;
            };
            Some(rect)
        };
        emu.handle_management.dc(dc)?.colors.clip = clip;
        clip_box(emu, dc).map(region_type)
    })()
    .unwrap_or(GDI_ERROR);
    log_red!(
        emu,
        "gdi32!SelectClipRgn dc={dc:#x} region={region:#x} region_type={result}"
    );
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 2);
}

fn clip_rect(emu: &mut Emu, api: &str, exclude: bool) {
    let call = WinApi(Stdcall);
    let [dc, left, top, right, bottom] = api_try!(emu, call.args(emu));
    let rect = [left as i32, top as i32, right as i32, bottom as i32];
    let result = if let Some(current) = clip_box(emu, dc) {
        let overlap = intersect(current, rect);
        let next = if !exclude {
            overlap
        } else if region_type(overlap) == NULLREGION {
            current
        } else if overlap == current {
            [0; 4]
        } else {
            policy::unsupported(emu, "gdi32", api, emu.pc());
            return;
        };
        if let Some(context) = emu.handle_management.dc(dc) {
            context.colors.clip = Some(next);
        }
        region_type(next)
    } else {
        GDI_ERROR
    };
    log_red!(
        emu,
        "gdi32!{api} dc={dc:#x} bounds={rect:?} region_type={result}"
    );
    call.set_return(emu, result.into());
    call.cleanup_args(emu, 5);
}

pub fn IntersectClipRect(emu: &mut Emu) {
    clip_rect(emu, "IntersectClipRect", false);
}
pub fn ExcludeClipRect(emu: &mut Emu) {
    clip_rect(emu, "ExcludeClipRect", true);
}

pub fn GetWorldTransform(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dc, output] = api_try!(emu, call.args(emu));
    let bytes: Vec<_> = IDENTITY_TRANSFORM
        .into_iter()
        .flat_map(f32::to_le_bytes)
        .collect();
    let success = emu.handle_management.dc(dc).is_some() && emu.maps.write_bytes(output, &bytes);
    log_red!(
        emu,
        "gdi32!GetWorldTransform dc={dc:#x} transform={output:#x} success={success}"
    );
    call.set_return(emu, success.into());
    call.cleanup_args(emu, 2);
}

pub fn SetWorldTransform(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dc, transform] = api_try!(emu, call.args(emu));
    // The default GM_COMPATIBLE graphics mode rejects SetWorldTransform.
    log_red!(
        emu,
        "gdi32!SetWorldTransform dc={dc:#x} transform={transform:#x}"
    );
    call.set_return(emu, 0);
    call.cleanup_args(emu, 2);
}
