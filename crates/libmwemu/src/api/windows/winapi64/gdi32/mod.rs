use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi64;

mod add_font_resource_ex;
pub use add_font_resource_ex::{AddFontResourceExA, AddFontResourceExW};

mod bit_blt;
pub use bit_blt::BitBlt;
mod combine_rgn;
pub use combine_rgn::CombineRgn;
mod create_brush_indirect;
pub use create_brush_indirect::CreateBrushIndirect;
mod create_compatible_bitmap;
pub use create_compatible_bitmap::CreateCompatibleBitmap;
mod create_compatible_dc;
pub use create_compatible_dc::CreateCompatibleDC;
mod create_dc;
pub use create_dc::{CreateDCA, CreateDCW};

mod create_dib_section;
pub use create_dib_section::CreateDIBSection;
mod create_font;
pub use create_font::{CreateFontA, CreateFontW};
mod create_font_indirect;

pub use create_font_indirect::{CreateFontIndirectA, CreateFontIndirectW};

mod create_hatch_brush;
pub use create_hatch_brush::CreateHatchBrush;
mod create_pattern_brush;
pub use create_pattern_brush::CreatePatternBrush;
mod create_pen;
pub use create_pen::CreatePen;
mod create_pen_indirect;
pub use create_pen_indirect::CreatePenIndirect;
mod create_rect_rgn;
pub use create_rect_rgn::CreateRectRgn;
mod create_rect_rgn_indirect;
pub use create_rect_rgn_indirect::CreateRectRgnIndirect;
mod create_solid_brush;
pub use create_solid_brush::CreateSolidBrush;
mod delete_dc;
pub use delete_dc::DeleteDC;
mod delete_object;
pub use delete_object::DeleteObject;
mod end_doc;
pub use end_doc::EndDoc;
mod end_page;
pub use end_page::EndPage;
mod enum_font_families_ex;
pub use enum_font_families_ex::{EnumFontFamiliesExA, EnumFontFamiliesExW};

mod exclude_clip_rect;
pub use exclude_clip_rect::ExcludeClipRect;
mod ext_text_out;
pub use ext_text_out::{ExtTextOutA, ExtTextOutW};

mod gdi_arc;
pub use gdi_arc::GdiArc;
mod gdi_chord;
pub use gdi_chord::GdiChord;
mod gdi_ellipse;
pub use gdi_ellipse::GdiEllipse;
mod gdi_fill_rect;
pub use gdi_fill_rect::GdiFillRect;
mod gdi_pie;
pub use gdi_pie::GdiPie;
mod gdi_poly_bezier;
pub use gdi_poly_bezier::GdiPolyBezier;
mod gdi_polygon;
pub use gdi_polygon::GdiPolygon;
mod gdi_polyline;
pub use gdi_polyline::GdiPolyline;
mod gdi_rectangle;
pub use gdi_rectangle::GdiRectangle;
mod get_bk_color;
pub use get_bk_color::GetBkColor;
mod get_char_abc_widths;
pub use get_char_abc_widths::{GetCharABCWidthsA, GetCharABCWidthsW};

mod get_char_width32;
pub use get_char_width32::{GetCharWidth32A, GetCharWidth32W};

mod get_clip_box;
pub use get_clip_box::GetClipBox;
mod get_current_object;
pub use get_current_object::GetCurrentObject;
mod get_dc;
pub use get_dc::GetDC;
mod get_dib_color_table;
pub use get_dib_color_table::GetDIBColorTable;
mod get_di_bits;
pub use get_di_bits::GetDIBits;
mod get_device_caps;
pub use get_device_caps::GetDeviceCaps;
mod get_layout;
pub use get_layout::GetLayout;
mod get_map_mode;
pub use get_map_mode::GetMapMode;
mod get_nearest_color;
pub use get_nearest_color::GetNearestColor;
mod get_object;
pub use get_object::{GetObjectA, GetObjectW};
mod get_object_type;

pub use get_object_type::GetObjectType;
mod get_pixel_gdi;
pub use get_pixel_gdi::GetPixelGdi;
mod get_poly_fill_mode;
pub use get_poly_fill_mode::GetPolyFillMode;
mod get_rop2;
pub use get_rop2::GetROP2;
mod get_stock_object;
pub use get_stock_object::GetStockObject;
mod get_stretch_blt_mode_gdi;
pub use get_stretch_blt_mode_gdi::GetStretchBltModeGdi;
mod get_system_palette_entries;
pub use get_system_palette_entries::GetSystemPaletteEntries;
mod get_text_align;
pub use get_text_align::GetTextAlign;
mod get_text_color;
pub use get_text_color::GetTextColor;
mod get_text_extent_point32;
pub use get_text_extent_point32::{GetTextExtentPoint32A, GetTextExtentPoint32W};

mod get_text_face;
pub use get_text_face::{GetTextFaceA, GetTextFaceW};

mod get_text_metrics;
pub use get_text_metrics::{GetTextMetricsA, GetTextMetricsW};

mod get_world_transform;
pub use get_world_transform::GetWorldTransform;
mod intersect_clip_rect;
pub use intersect_clip_rect::IntersectClipRect;
mod line_to;
pub use line_to::LineTo;
mod move_to_ex;
pub use move_to_ex::MoveToEx;
mod pat_blt;
pub use pat_blt::PatBlt;
mod release_dc;
pub use release_dc::ReleaseDC;
mod remove_font_resource_ex;
pub use remove_font_resource_ex::{RemoveFontResourceExA, RemoveFontResourceExW};

mod restore_dc;
pub use restore_dc::RestoreDC;
mod save_dc;
pub use save_dc::SaveDC;
mod select_clip_rgn;
pub use select_clip_rgn::SelectClipRgn;
mod select_object;
pub use select_object::SelectObject;
mod set_bk_color;
pub use set_bk_color::SetBkColor;
mod set_bk_mode;
pub use set_bk_mode::SetBkMode;
mod set_dc_brush_color;
pub use set_dc_brush_color::SetDCBrushColor;
mod set_dc_pen_color;
pub use set_dc_pen_color::SetDCPenColor;
mod set_di_bits;
pub use set_di_bits::SetDIBits;
mod set_layout;
pub use set_layout::SetLayout;
mod set_map_mode;
pub use set_map_mode::SetMapMode;
mod set_pixel;
pub use set_pixel::SetPixel;
mod set_poly_fill_mode;
pub use set_poly_fill_mode::SetPolyFillMode;
mod set_rop2;
pub use set_rop2::SetROP2;
mod set_stretch_blt_mode;
pub use set_stretch_blt_mode::SetStretchBltMode;
mod set_text_align;
pub use set_text_align::SetTextAlign;
mod set_text_color;
pub use set_text_color::SetTextColor;
mod set_viewport_org_ex;
pub use set_viewport_org_ex::SetViewportOrgEx;
mod set_window_org_ex;
pub use set_window_org_ex::SetWindowOrgEx;
mod set_world_transform;
pub use set_world_transform::SetWorldTransform;
mod start_doc;
pub use start_doc::StartDoc;
mod start_page;
pub use start_page::StartPage;
mod stretch_blt;
pub use stretch_blt::StretchBlt;
mod text_out;
pub use text_out::{TextOutA, TextOutW};

pub fn gateway(addr: u64, emu: &mut emu::Emu) -> String {
    let api = winapi64::kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        "BitBlt" => BitBlt(emu),
        "CreateCompatibleBitmap" => CreateCompatibleBitmap(emu),
        "CreateCompatibleDC" => CreateCompatibleDC(emu),
        "CreateDIBSection" => CreateDIBSection(emu),
        "CreateFontA" => CreateFontA(emu),
        "CreateFontIndirectA" => CreateFontIndirectA(emu),
        "CreateFontIndirectW" => CreateFontIndirectW(emu),
        "CreateFontW" => CreateFontW(emu),
        "CreatePen" => CreatePen(emu),
        "CreateSolidBrush" => CreateSolidBrush(emu),
        "DeleteDC" => DeleteDC(emu),
        "DeleteObject" => DeleteObject(emu),
        "Ellipse" => GdiEllipse(emu),
        "ExtTextOutA" => ExtTextOutA(emu),
        "ExtTextOutW" => ExtTextOutW(emu),
        "GetBkColor" => GetBkColor(emu),
        "GetDeviceCaps" => GetDeviceCaps(emu),
        "GetObjectA" => GetObjectA(emu),
        "GetObjectW" => GetObjectW(emu),
        "GetPixel" => GetPixelGdi(emu),
        "GetStockObject" => GetStockObject(emu),
        "GetTextExtentPoint32A" => GetTextExtentPoint32A(emu),
        "GetTextExtentPoint32W" => GetTextExtentPoint32W(emu),
        "GetTextMetricsA" => GetTextMetricsA(emu),
        "GetTextMetricsW" => GetTextMetricsW(emu),
        "LineTo" => LineTo(emu),
        "MoveToEx" => MoveToEx(emu),
        "Rectangle" => GdiRectangle(emu),
        "RestoreDC" => RestoreDC(emu),
        "SaveDC" => SaveDC(emu),
        "SelectObject" => SelectObject(emu),
        "SetBkColor" => SetBkColor(emu),
        "SetBkMode" => SetBkMode(emu),
        "SetPixel" => SetPixel(emu),
        "SetStretchBltMode" => SetStretchBltMode(emu),
        "SetTextColor" => SetTextColor(emu),
        "StretchBlt" => StretchBlt(emu),
        "TextOutA" => TextOutA(emu),
        "TextOutW" => TextOutW(emu),
        "AddFontResourceExA" => AddFontResourceExA(emu),
        "AddFontResourceExW" => AddFontResourceExW(emu),
        "Arc" => GdiArc(emu),
        "Chord" => GdiChord(emu),
        "CombineRgn" => CombineRgn(emu),
        "CreateBrushIndirect" => CreateBrushIndirect(emu),
        "CreateDCA" => CreateDCA(emu),
        "CreateDCW" => CreateDCW(emu),
        "CreateHatchBrush" => CreateHatchBrush(emu),
        "CreatePatternBrush" => CreatePatternBrush(emu),
        "CreatePenIndirect" => CreatePenIndirect(emu),
        "CreateRectRgn" => CreateRectRgn(emu),
        "CreateRectRgnIndirect" => CreateRectRgnIndirect(emu),
        "EndDoc" => EndDoc(emu),
        "EndPage" => EndPage(emu),
        "EnumFontFamiliesExA" => EnumFontFamiliesExA(emu),
        "EnumFontFamiliesExW" => EnumFontFamiliesExW(emu),
        "ExcludeClipRect" => ExcludeClipRect(emu),
        "FillRect" => GdiFillRect(emu),
        "GetCharABCWidthsA" => GetCharABCWidthsA(emu),
        "GetCharABCWidthsW" => GetCharABCWidthsW(emu),
        "GetCharWidth32A" => GetCharWidth32A(emu),
        "GetCharWidth32W" => GetCharWidth32W(emu),
        "GetClipBox" => GetClipBox(emu),
        "GetCurrentObject" => GetCurrentObject(emu),
        "GetDC" => GetDC(emu),
        "GetDIBColorTable" => GetDIBColorTable(emu),
        "GetDIBits" => GetDIBits(emu),
        "GetLayout" => GetLayout(emu),
        "GetMapMode" => GetMapMode(emu),
        "GetNearestColor" => GetNearestColor(emu),
        "GetObjectType" => GetObjectType(emu),
        "GetPolyFillMode" => GetPolyFillMode(emu),
        "GetROP2" => GetROP2(emu),
        "GetStretchBltMode" => GetStretchBltModeGdi(emu),
        "GetSystemPaletteEntries" => GetSystemPaletteEntries(emu),
        "GetTextAlign" => GetTextAlign(emu),
        "GetTextColor" => GetTextColor(emu),
        "GetTextFaceA" => GetTextFaceA(emu),
        "GetTextFaceW" => GetTextFaceW(emu),
        "GetWorldTransform" => GetWorldTransform(emu),
        "IntersectClipRect" => IntersectClipRect(emu),
        "PatBlt" => PatBlt(emu),
        "Pie" => GdiPie(emu),
        "PolyBezier" => GdiPolyBezier(emu),
        "Polygon" => GdiPolygon(emu),
        "Polyline" => GdiPolyline(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "RemoveFontResourceExA" => RemoveFontResourceExA(emu),
        "RemoveFontResourceExW" => RemoveFontResourceExW(emu),
        "SelectClipRgn" => SelectClipRgn(emu),
        "SetDCBrushColor" => SetDCBrushColor(emu),
        "SetDCPenColor" => SetDCPenColor(emu),
        "SetDIBits" => SetDIBits(emu),
        "SetLayout" => SetLayout(emu),
        "SetMapMode" => SetMapMode(emu),
        "SetPolyFillMode" => SetPolyFillMode(emu),
        "SetROP2" => SetROP2(emu),
        "SetTextAlign" => SetTextAlign(emu),
        "SetViewportOrgEx" => SetViewportOrgEx(emu),
        "SetWindowOrgEx" => SetWindowOrgEx(emu),
        "SetWorldTransform" => SetWorldTransform(emu),
        "StartDoc" | "StartDocA" | "StartDocW" => StartDoc(emu),
        "StartPage" => StartPage(emu),
        _ => return fallback::unimplemented(addr, api, emu),
    }
    String::new()
}

// === DC operations ===

// === Bitmap ===

// === Object ===

// === Font ===

// === Text ===

// === Drawing ===

// === Pen/Brush ===

// === Region ===

// === Map/Viewport ===

// === ROP/mode ===

// === Transform ===

// === Printer ===

// === Palette ===

// === DC Colors ===

// === Layout ===
