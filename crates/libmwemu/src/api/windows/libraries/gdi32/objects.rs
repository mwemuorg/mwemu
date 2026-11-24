use serde::{Deserialize, Serialize};

use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::winapi::common::{memory, strings};
use crate::winapi::policy::api_try;

const OBJ_PEN: u32 = 1;
const OBJ_BRUSH: u32 = 2;
const OBJ_DC: u32 = 3;
const OBJ_PAL: u32 = 5;
const OBJ_FONT: u32 = 6;
const OBJ_BITMAP: u32 = 7;
const OBJ_REGION: u32 = 8;
const OBJ_MEMDC: u32 = 10;

const BS_SOLID: u32 = 0;
const BS_NULL: u32 = 1;
const BS_HATCHED: u32 = 2;
const BS_PATTERN: u32 = 3;
const PS_NULL: u32 = 5;
const HS_DIAGCROSS: u32 = 5;

const WHITE_BRUSH: i32 = 0;
const NULL_BRUSH: i32 = 5;
const WHITE_PEN: i32 = 6;
const BLACK_PEN: i32 = 7;
const NULL_PEN: i32 = 8;
const OEM_FIXED_FONT: i32 = 10;
const SYSTEM_FONT: i32 = 13;
const DEFAULT_PALETTE: i32 = 15;
const SYSTEM_FIXED_FONT: i32 = 16;
const DEFAULT_GUI_FONT: i32 = 17;
const DC_BRUSH: i32 = 18;
const DC_PEN: i32 = 19;

const LOGFONT_HEADER: usize = 28;
const LF_FACESIZE: usize = 32;
const RGB_WHITE: u32 = 0x00ff_ffff;
const LOGPEN_SIZE: usize = 16;
const LOGBRUSH_HEADER: usize = 8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum GraphicObject {
    Brush {
        style: u32,
        color: u32,
        hatch: u64,
    },
    Pen {
        style: u32,
        width: i32,
        color: u32,
    },
    Bitmap {
        width: i32,
        height: i32,
        depth: u16,
    },
    Font {
        header: [u8; LOGFONT_HEADER],
        face: String,
    },
    Region([i32; 4]),
    Palette,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct GraphicHandle {
    pub(crate) object: GraphicObject,
    pub(crate) stock: Option<i32>,
}

const GDI_STOCK_OBJECT: u16 = 0x80;
const GDI_BRUSH_TYPE: u16 = 0x10;
const GDI_PEN_TYPE: u16 = 0x30;
const GDI_FONT_TYPE: u16 = 0x0a;
const GDI_PALETTE_TYPE: u16 = 0x08;

impl GraphicHandle {
    pub(crate) fn handle(&self, slot: usize) -> u64 {
        let Some(stock) = self.stock else {
            return slot as u64;
        };
        if stock < 0 || slot > u16::MAX as usize {
            return slot as u64;
        }
        let kind = match self.object {
            GraphicObject::Brush { .. } => GDI_BRUSH_TYPE,
            GraphicObject::Pen { .. } => GDI_PEN_TYPE,
            GraphicObject::Font { .. } => GDI_FONT_TYPE,
            GraphicObject::Palette => GDI_PALETTE_TYPE,
            _ => return slot as u64,
        };
        ((GDI_STOCK_OBJECT | kind) as u64) << u16::BITS | slot as u64
    }
}

pub(crate) fn stock_objects(emu: &mut Emu) -> Vec<(u64, u64)> {
    (WHITE_BRUSH..=DC_PEN)
        .filter_map(|index| {
            let handle = stock_object(emu, index);
            (handle != 0).then_some((index as u64, handle))
        })
        .collect()
}

impl GraphicObject {
    fn kind(&self) -> u32 {
        match self {
            Self::Brush { .. } => OBJ_BRUSH,
            Self::Pen { .. } => OBJ_PEN,
            Self::Bitmap { .. } => OBJ_BITMAP,
            Self::Font { .. } => OBJ_FONT,
            Self::Region(_) => OBJ_REGION,
            Self::Palette => OBJ_PAL,
        }
    }

    fn bytes(&self, is64: bool, wide: bool) -> Option<Vec<u8>> {
        let pointer = if is64 { 8 } else { 4 };
        let mut data = Vec::new();
        match self {
            Self::Brush {
                style,
                color,
                hatch,
            } => {
                data.extend(style.to_le_bytes());
                data.extend(color.to_le_bytes());
                data.extend(&hatch.to_le_bytes()[..pointer]);
            }
            Self::Pen {
                style,
                width,
                color,
            } => {
                data.extend(style.to_le_bytes());
                data.extend(width.to_le_bytes());
                data.extend(0i32.to_le_bytes());
                data.extend(color.to_le_bytes());
            }
            Self::Bitmap {
                width,
                height,
                depth,
            } => {
                let stride = bitmap_stride(*width, *depth)?;
                data.extend(0i32.to_le_bytes());
                data.extend(width.to_le_bytes());
                data.extend(height.to_le_bytes());
                data.extend(stride.to_le_bytes());
                data.extend(1u16.to_le_bytes());
                data.extend(depth.to_le_bytes());
                data.resize(if is64 { 32 } else { 24 }, 0);
            }
            Self::Font { header, face } => {
                data.extend(header);
                let unit = if wide { 2 } else { 1 };
                let mut text = strings::encode_text(face, wide);
                text.truncate((LF_FACESIZE - 1) * unit);
                data.extend(text);
                data.resize(LOGFONT_HEADER + LF_FACESIZE * unit, 0);
            }
            Self::Palette => data.extend(0u16.to_le_bytes()),
            Self::Region(_) => return None,
        }
        Some(data)
    }
}

fn bitmap_stride(width: i32, depth: u16) -> Option<i32> {
    let bits = (width as u32).checked_mul(depth.into())?;
    i32::try_from(bits.div_ceil(16).checked_mul(2)?).ok()
}

fn create(emu: &mut Emu, object: Option<GraphicObject>) -> u64 {
    object.map_or(0, |object| {
        emu.handle_management.insert_graphic(GraphicHandle {
            object,
            stock: None,
        })
    })
}

// The initial monochrome bitmap in a memory DC is not a public stock index.
const INITIAL_BITMAP: i32 = -1;

fn stock_object(emu: &mut Emu, index: i32) -> u64 {
    if let Some(handle) = emu.handle_management.stock_graphic(index) {
        return handle;
    }
    let object = match index {
        WHITE_BRUSH..=NULL_BRUSH | DC_BRUSH => {
            let color = match index {
                0 | DC_BRUSH => RGB_WHITE,
                1 => 0xc0c0c0,
                2 => 0x808080,
                3 => 0x404040,
                _ => 0,
            };
            GraphicObject::Brush {
                style: if index == NULL_BRUSH {
                    BS_NULL
                } else {
                    BS_SOLID
                },
                color,
                hatch: 0,
            }
        }
        WHITE_PEN..=NULL_PEN | DC_PEN => GraphicObject::Pen {
            style: if index == NULL_PEN { PS_NULL } else { 0 },
            width: 1,
            color: if index == WHITE_PEN { RGB_WHITE } else { 0 },
        },
        OEM_FIXED_FONT..=14 | SYSTEM_FIXED_FONT | DEFAULT_GUI_FONT => GraphicObject::Font {
            header: [0; LOGFONT_HEADER],
            face: if index == DEFAULT_GUI_FONT {
                "MS Shell Dlg"
            } else {
                "System"
            }
            .into(),
        },
        DEFAULT_PALETTE => GraphicObject::Palette,
        INITIAL_BITMAP => GraphicObject::Bitmap {
            width: 1,
            height: 1,
            depth: 1,
        },
        _ => return 0,
    };
    emu.handle_management.insert_graphic(GraphicHandle {
        object,
        stock: Some(index),
    })
}

fn slot(kind: u32) -> Option<(usize, i32)> {
    match kind {
        OBJ_PEN => Some((0, BLACK_PEN)),
        OBJ_BRUSH => Some((1, WHITE_BRUSH)),
        OBJ_FONT => Some((2, SYSTEM_FONT)),
        OBJ_BITMAP => Some((3, INITIAL_BITMAP)),
        OBJ_PAL => Some((4, DEFAULT_PALETTE)),
        _ => None,
    }
}

fn selected(emu: &mut Emu, dc: u64, kind: u32) -> Option<u64> {
    let (index, stock) = slot(kind)?;
    let dc = emu.handle_management.dc(dc)?;
    if kind == OBJ_BITMAP && !dc.memory {
        return None;
    }
    let selected = dc.colors.selected[index];
    Some(if selected == 0 {
        stock_object(emu, stock)
    } else {
        selected
    })
}

pub(super) fn current_font(emu: &mut Emu, dc: u64) -> Option<u64> {
    selected(emu, dc, OBJ_FONT)
}

pub fn GetObjectType(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let object = api_try!(emu, call.arg(emu, 0));
    let kind = if let Some(dc) = emu.handle_management.dc(object) {
        if dc.memory { OBJ_MEMDC } else { OBJ_DC }
    } else {
        emu.handle_management
            .graphic(object)
            .map_or(0, |entry| entry.object.kind())
    };
    log_red!(emu, "gdi32!GetObjectType object={object:#x} kind={kind}");
    call.set_return(emu, kind.into());
    call.cleanup_args(emu, 1);
}

pub fn GetStockObject(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let index = api_try!(emu, call.arg_i32(emu, 0));
    let result = if index < 0 {
        0
    } else {
        stock_object(emu, index)
    };
    log_red!(emu, "gdi32!GetStockObject index={index} object={result:#x}");
    call.set_return(emu, result);
    call.cleanup_args(emu, 1);
}

pub fn GetCurrentObject(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc = api_try!(emu, call.arg(emu, 0));
    let kind = api_try!(emu, call.arg_u32(emu, 1));
    let object = selected(emu, dc, kind).unwrap_or(0);
    log_red!(
        emu,
        "gdi32!GetCurrentObject dc={dc:#x} kind={kind} object={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 2);
}

pub fn SelectObject(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [dc, object] = api_try!(emu, call.args(emu));
    let kind = emu
        .handle_management
        .graphic(object)
        .map(|entry| entry.object.kind());
    let previous = match kind {
        Some(OBJ_REGION) => {
            let rect = emu
                .handle_management
                .graphic(object)
                .and_then(|entry| match entry.object {
                    GraphicObject::Region(rect) => Some(rect),
                    _ => None,
                });
            match (emu.handle_management.dc(dc), rect) {
                (Some(context), Some(rect)) => {
                    context.colors.clip = Some(rect);
                    if rect[0] >= rect[2] || rect[1] >= rect[3] {
                        1
                    } else {
                        2
                    }
                }
                _ => u32::MAX as u64,
            }
        }
        Some(OBJ_BITMAP) if emu.handle_management.graphic_in_other_dc(object, dc) => 0,
        Some(kind) if kind != OBJ_PAL => slot(kind)
            .and_then(|(index, _)| {
                let previous = selected(emu, dc, kind)?;
                emu.handle_management.dc(dc)?.colors.selected[index] = object;
                Some(previous)
            })
            .unwrap_or(0),
        _ => 0,
    };
    log_red!(
        emu,
        "gdi32!SelectObject dc={dc:#x} object={object:#x} previous={previous:#x}"
    );
    call.set_return(emu, previous);
    call.cleanup_args(emu, 2);
}

pub fn DeleteObject(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let object = api_try!(emu, call.arg(emu, 0));
    let removed = emu.handle_management.delete_graphic(object);
    log_red!(
        emu,
        "gdi32!DeleteObject object={object:#x} success={removed}"
    );
    call.set_return(emu, removed.into());
    call.cleanup_args(emu, 1);
}

pub fn CreateSolidBrush(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let color = api_try!(emu, call.arg_u32(emu, 0));
    let object = create(
        emu,
        Some(GraphicObject::Brush {
            style: BS_SOLID,
            color,
            hatch: 0,
        }),
    );
    log_red!(
        emu,
        "gdi32!CreateSolidBrush color={color:#x} brush={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 1);
}

pub fn CreateHatchBrush(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let hatch = api_try!(emu, call.arg_u32(emu, 0));
    let color = api_try!(emu, call.arg_u32(emu, 1));
    let object = create(
        emu,
        (hatch <= HS_DIAGCROSS).then_some(GraphicObject::Brush {
            style: BS_HATCHED,
            color,
            hatch: hatch.into(),
        }),
    );
    log_red!(
        emu,
        "gdi32!CreateHatchBrush hatch={hatch} color={color:#x} brush={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 2);
}

pub fn CreatePatternBrush(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let bitmap = api_try!(emu, call.arg(emu, 0));
    let valid = emu
        .handle_management
        .graphic(bitmap)
        .is_some_and(|entry| matches!(entry.object, GraphicObject::Bitmap { .. }));
    let object = create(
        emu,
        valid.then_some(GraphicObject::Brush {
            style: BS_PATTERN,
            color: 0,
            hatch: bitmap,
        }),
    );
    log_red!(
        emu,
        "gdi32!CreatePatternBrush bitmap={bitmap:#x} brush={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 1);
}

pub fn CreateBrushIndirect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let brush = (|| {
        let style = emu.maps.read_dword(source)?;
        let color = emu.maps.read_dword(source.checked_add(4)?)?;
        let hatch = memory::read_pointer(emu, source.checked_add(LOGBRUSH_HEADER as u64)?)?;
        match style {
            BS_SOLID | BS_NULL => Some(GraphicObject::Brush {
                style,
                color,
                hatch: 0,
            }),
            BS_HATCHED if hatch <= HS_DIAGCROSS.into() => Some(GraphicObject::Brush {
                style,
                color,
                hatch,
            }),
            BS_PATTERN
                if emu
                    .handle_management
                    .graphic(hatch)
                    .is_some_and(|e| matches!(e.object, GraphicObject::Bitmap { .. })) =>
            {
                Some(GraphicObject::Brush {
                    style,
                    color: 0,
                    hatch,
                })
            }
            _ => None,
        }
    })();
    let object = create(emu, brush);
    log_red!(
        emu,
        "gdi32!CreateBrushIndirect logbrush={source:#x} brush={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 1);
}

fn pen(style: u32, width: i32, color: u32) -> Option<GraphicObject> {
    if style > 6 {
        return None;
    }
    let width = width.checked_abs()?;
    let style = if width > 1 && (1..=4).contains(&style) {
        0
    } else {
        style
    };
    Some(GraphicObject::Pen {
        style,
        width,
        color,
    })
}

pub fn CreatePen(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let style = api_try!(emu, call.arg_u32(emu, 0));
    let width = api_try!(emu, call.arg_i32(emu, 1));
    let color = api_try!(emu, call.arg_u32(emu, 2));
    let object = create(emu, pen(style, width, color));
    log_red!(
        emu,
        "gdi32!CreatePen style={style} width={width} color={color:#x} pen={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 3);
}

pub fn CreatePenIndirect(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let source = api_try!(emu, call.arg(emu, 0));
    let description = memory::read_array::<LOGPEN_SIZE>(emu, source).and_then(|bytes| {
        let field = |offset: usize| {
            u32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ])
        };
        pen(field(0), field(4) as i32, field(12))
    });
    let object = create(emu, description);
    log_red!(
        emu,
        "gdi32!CreatePenIndirect logpen={source:#x} pen={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 1);
}

pub fn CreateCompatibleBitmap(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let dc = api_try!(emu, call.arg(emu, 0));
    let width = api_try!(emu, call.arg_i32(emu, 1));
    let height = api_try!(emu, call.arg_i32(emu, 2));
    let bitmap = (|| {
        let memory = emu.handle_management.dc(dc)?.memory;
        if width < 0 || height < 0 {
            return None;
        }
        if width == 0 || height == 0 {
            return Some(GraphicObject::Bitmap {
                width: 1,
                height: 1,
                depth: 1,
            });
        }
        let depth = if memory {
            let current = selected(emu, dc, OBJ_BITMAP)?;
            let GraphicObject::Bitmap { depth, .. } =
                emu.handle_management.graphic(current)?.object
            else {
                return None;
            };
            depth
        } else {
            32
        };
        bitmap_stride(width, depth)?.checked_mul(height)?;
        Some(GraphicObject::Bitmap {
            width,
            height,
            depth,
        })
    })();
    let object = create(emu, bitmap);
    log_red!(
        emu,
        "gdi32!CreateCompatibleBitmap dc={dc:#x} width={width} height={height} bitmap={object:#x}"
    );
    call.set_return(emu, object);
    call.cleanup_args(emu, 3);
}

fn rectangle(emu: &mut Emu, api: &str, indirect: bool) {
    let call = WinApi(Stdcall);
    let rect = if indirect {
        let source = api_try!(emu, call.arg(emu, 0));
        memory::read_array::<16>(emu, source).map(|bytes| {
            std::array::from_fn(|index| {
                let at = index * size_of::<i32>();
                i32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
            })
        })
    } else {
        Some(api_try!(emu, call.args::<4>(emu)).map(|coord| coord as i32))
    };
    let object = create(emu, rect.map(GraphicObject::Region));
    log_red!(emu, "gdi32!{api} bounds={rect:?} region={object:#x}");
    call.set_return(emu, object);
    call.cleanup_args(emu, if indirect { 1 } else { 4 });
}

pub fn CreateRectRgn(emu: &mut Emu) {
    rectangle(emu, "CreateRectRgn", false);
}
pub fn CreateRectRgnIndirect(emu: &mut Emu) {
    rectangle(emu, "CreateRectRgnIndirect", true);
}

fn create_font(emu: &mut Emu, api: &str, wide: bool, indirect: bool) {
    let call = WinApi(Stdcall);
    let font = if indirect {
        let source = api_try!(emu, call.arg(emu, 0));
        let unit = if wide { 2 } else { 1 };
        emu.maps
            .try_read_bytes(source, LOGFONT_HEADER + LF_FACESIZE * unit)
            .map(|bytes| {
                let mut header = [0; LOGFONT_HEADER];
                header.copy_from_slice(&bytes[..LOGFONT_HEADER]);
                let face = &bytes[LOGFONT_HEADER..];
                let face = if wide {
                    let units: Vec<_> = face
                        .chunks_exact(2)
                        .map(|pair| u16::from_le_bytes([pair[0], pair[1]]))
                        .take_while(|&c| c != 0)
                        .collect();
                    String::from_utf16_lossy(&units)
                } else {
                    let end = face.iter().position(|&b| b == 0).unwrap_or(face.len());
                    String::from_utf8_lossy(&face[..end]).into_owned()
                };
                GraphicObject::Font { header, face }
            })
    } else {
        let fields = api_try!(emu, call.args::<14>(emu));
        let face = if fields[13] == 0 {
            Some(String::new())
        } else {
            strings::read_text_checked(emu, fields[13], wide)
        };
        face.map(|face| {
            let mut header = [0; LOGFONT_HEADER];
            for (destination, field) in header[..20].chunks_exact_mut(4).zip(&fields[..5]) {
                destination.copy_from_slice(&(*field as u32).to_le_bytes());
            }
            for (destination, field) in header[20..].iter_mut().zip(&fields[5..13]) {
                *destination = *field as u8;
            }
            GraphicObject::Font { header, face }
        })
    };
    let object = create(emu, font);
    log_red!(emu, "gdi32!{api} font={object:#x}");
    call.set_return(emu, object);
    call.cleanup_args(emu, if indirect { 1 } else { 14 });
}

pub fn CreateFontA(emu: &mut Emu) {
    create_font(emu, "CreateFontA", false, false);
}
pub fn CreateFontW(emu: &mut Emu) {
    create_font(emu, "CreateFontW", true, false);
}
pub fn CreateFontIndirectA(emu: &mut Emu) {
    create_font(emu, "CreateFontIndirectA", false, true);
}
pub fn CreateFontIndirectW(emu: &mut Emu) {
    create_font(emu, "CreateFontIndirectW", true, true);
}

fn get_object(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let object = api_try!(emu, call.arg(emu, 0));
    let capacity = api_try!(emu, call.arg_i32(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let count = (|| {
        let entry = emu.handle_management.graphic(object)?;
        let mut bytes = entry.object.bytes(emu.cfg.arch.is_64bits(), wide)?;
        if output == 0 {
            return Some(bytes.len());
        }
        let capacity = usize::try_from(capacity).ok()?;
        if capacity < bytes.len() {
            if !matches!(entry.object, GraphicObject::Font { .. }) {
                return None;
            }
            bytes.truncate(capacity);
        }
        emu.maps.write_bytes(output, &bytes).then_some(bytes.len())
    })()
    .unwrap_or(0);
    log_red!(
        emu,
        "gdi32!{api} object={object:#x} capacity={capacity} output={output:#x} bytes={count}"
    );
    call.set_return(emu, count as u64);
    call.cleanup_args(emu, 3);
}

pub fn GetObjectA(emu: &mut Emu) {
    get_object(emu, "GetObjectA", false);
}
pub fn GetObjectW(emu: &mut Emu) {
    get_object(emu, "GetObjectW", true);
}

fn text_face(emu: &mut Emu, api: &str, wide: bool) {
    let call = WinApi(Stdcall);
    let dc = api_try!(emu, call.arg(emu, 0));
    let capacity = api_try!(emu, call.arg_i32(emu, 1));
    let output = api_try!(emu, call.arg(emu, 2));
    let count = (|| {
        let font = selected(emu, dc, OBJ_FONT)?;
        let GraphicObject::Font { face, .. } = &emu.handle_management.graphic(font)?.object else {
            return None;
        };
        let unit = if wide { 2 } else { 1 };
        let mut bytes = strings::encode_text(if face.is_empty() { "System" } else { face }, wide);
        bytes.resize(bytes.len() + unit, 0);
        if output == 0 {
            return Some(bytes.len() / unit);
        }
        if capacity <= 0 {
            return None;
        }
        bytes.truncate((capacity as usize).saturating_mul(unit));
        let terminator = bytes.len() - unit;
        bytes[terminator..].fill(0);
        emu.maps
            .write_bytes(output, &bytes)
            .then_some(bytes.len() / unit)
    })()
    .unwrap_or(0);
    log_red!(
        emu,
        "gdi32!{api} dc={dc:#x} capacity={capacity} output={output:#x} length={count}"
    );
    call.set_return(emu, count as u64);
    call.cleanup_args(emu, 3);
}

pub fn GetTextFaceA(emu: &mut Emu) {
    text_face(emu, "GetTextFaceA", false);
}
pub fn GetTextFaceW(emu: &mut Emu) {
    text_face(emu, "GetTextFaceW", true);
}
