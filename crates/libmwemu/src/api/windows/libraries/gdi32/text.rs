use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::policy::{self, api_try};

use super::objects::{self, GraphicObject};

// Deterministic metrics for the virtual system font. Arbitrary guest fonts need a font engine.
const CELL_WIDTH: u32 = 8;
const CELL_HEIGHT: u32 = 16;
const TEXTMETRICA_BYTES: usize = 56;
const TEXTMETRICW_BYTES: usize = 60;

fn system_font(emu: &mut Emu, dc: u64) -> Option<bool> {
    let current = objects::current_font(emu, dc)?;
    let GraphicObject::Font { face, .. } = &emu.handle_management.graphic(current)?.object else {
        return None;
    };
    Some(face.is_empty() || face.eq_ignore_ascii_case("System"))
}

fn metrics(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [dc, output] = api_try!(emu, call.args(emu));
    let supported = system_font(emu, dc);
    if supported == Some(false) {
        policy::unsupported(emu, "gdi32", api, emu.pc());
        return;
    }
    let mut data = vec![
        0;
        if wide {
            TEXTMETRICW_BYTES
        } else {
            TEXTMETRICA_BYTES
        }
    ];
    let fields = [
        CELL_HEIGHT,
        13,
        3,
        0,
        0,
        CELL_WIDTH,
        CELL_WIDTH,
        400,
        0,
        96,
        96,
    ];
    for (dest, value) in data.chunks_exact_mut(4).zip(fields) {
        dest.copy_from_slice(&value.to_le_bytes());
    }
    let characters = [0x20u16, 0xff, b'?' as u16, 0x20];
    let at = fields.len() * 4;
    if wide {
        for (dest, c) in data[at..].chunks_exact_mut(2).zip(characters) {
            dest.copy_from_slice(&c.to_le_bytes());
        }
    } else {
        data[at..at + 4].copy_from_slice(&characters.map(|c| c as u8));
    }
    let success = supported.is_some() && emu.maps.write_bytes(output, &data);
    log_red!(
        emu,
        "gdi32!{api} dc={dc:#x} metrics={output:#x} success={success}"
    );
    call.set_return(emu, success.into());
    call.cleanup_args(emu, 2);
}

pub fn GetTextMetricsA(emu: &mut Emu) {
    metrics(emu, "GetTextMetricsA", false);
}
pub fn GetTextMetricsW(emu: &mut Emu) {
    metrics(emu, "GetTextMetricsW", true);
}

fn extent(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let [dc, text, count, output] = api_try!(emu, call.args(emu));
    let count = count as i32;
    let supported = system_font(emu, dc);
    if supported == Some(false) {
        policy::unsupported(emu, "gdi32", api, emu.pc());
        return;
    }
    let width = u32::try_from(count)
        .ok()
        .and_then(|n| n.checked_mul(CELL_WIDTH));
    let success = supported.is_some()
        && width.is_some_and(|width| {
            let unit: usize = if wide { 2 } else { 1 };
            if !usize::try_from(count)
                .ok()
                .and_then(|n| n.checked_mul(unit))
                .is_some_and(|size| size == 0 || emu.maps.try_read_bytes(text, size).is_some())
            {
                return false;
            }
            let mut size = [0; 8];
            size[..4].copy_from_slice(&width.to_le_bytes());
            size[4..].copy_from_slice(&CELL_HEIGHT.to_le_bytes());
            emu.maps.write_bytes(output, &size)
        });
    log_red!(
        emu,
        "gdi32!{api} dc={dc:#x} text={text:#x} count={count} size={output:#x} success={success}"
    );
    call.set_return(emu, success.into());
    call.cleanup_args(emu, 4);
}

pub fn GetTextExtentPoint32A(emu: &mut Emu) {
    extent(emu, "GetTextExtentPoint32A", false);
}
pub fn GetTextExtentPoint32W(emu: &mut Emu) {
    extent(emu, "GetTextExtentPoint32W", true);
}

fn widths(emu: &mut Emu, api: &str, abc: bool) {
    let call = WinApi(Stdcall);
    let [dc, first, last, output] = api_try!(emu, call.args(emu));
    let first = first as u32;
    let last = last as u32;
    let supported = system_font(emu, dc);
    if supported == Some(false) {
        policy::unsupported(emu, "gdi32", api, emu.pc());
        return;
    }
    // GetCharABCWidths only supports TrueType fonts, not the virtual raster font.
    let success = !abc && supported.is_some() && last <= u16::MAX.into() && first <= last && {
        let count = (last - first + 1) as usize;
        let width = CELL_WIDTH.to_le_bytes();
        emu.maps
            .validate_write_range(output, (count * width.len()) as u64)
            && emu.maps.write_bytes(output, &width.repeat(count))
    };
    log_red!(
        emu,
        "gdi32!{api} dc={dc:#x} first_character={first} last_character={last} widths={output:#x} success={success}"
    );
    call.set_return(emu, success.into());
    call.cleanup_args(emu, 4);
}

pub fn GetCharWidth32A(emu: &mut Emu) {
    widths(emu, "GetCharWidth32A", false);
}
pub fn GetCharWidth32W(emu: &mut Emu) {
    widths(emu, "GetCharWidth32W", false);
}
pub fn GetCharABCWidthsA(emu: &mut Emu) {
    widths(emu, "GetCharABCWidthsA", true);
}
pub fn GetCharABCWidthsW(emu: &mut Emu) {
    widths(emu, "GetCharABCWidthsW", true);
}
