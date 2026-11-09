use crate::api::windows::common::fallback;
use crate::emu;
use crate::winapi::winapi32::kernel32;

mod add_font_resource_ex_a;
pub use add_font_resource_ex_a::AddFontResourceExA;
mod add_font_resource_ex_w;
pub use add_font_resource_ex_w::AddFontResourceExW;
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
mod create_dca;
pub use create_dca::CreateDCA;
mod create_dcw;
pub use create_dcw::CreateDCW;
mod create_dib_section;
pub use create_dib_section::CreateDIBSection;
mod create_font_a;
pub use create_font_a::CreateFontA;
mod create_font_indirect_a;
pub use create_font_indirect_a::CreateFontIndirectA;
mod create_font_indirect_w;
pub use create_font_indirect_w::CreateFontIndirectW;
mod create_font_w;
pub use create_font_w::CreateFontW;
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
mod enum_font_families_ex_a;
pub use enum_font_families_ex_a::EnumFontFamiliesExA;
mod enum_font_families_ex_w;
pub use enum_font_families_ex_w::EnumFontFamiliesExW;
mod exclude_clip_rect;
pub use exclude_clip_rect::ExcludeClipRect;
mod ext_text_out_a;
pub use ext_text_out_a::ExtTextOutA;
mod ext_text_out_w;
pub use ext_text_out_w::ExtTextOutW;
mod fill_rect;
pub use fill_rect::FillRect;
mod gdi_arc;
pub use gdi_arc::GdiArc;
mod gdi_chord;
pub use gdi_chord::GdiChord;
mod gdi_ellipse;
pub use gdi_ellipse::GdiEllipse;
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
mod get_char_abc_widths_a;
pub use get_char_abc_widths_a::GetCharABCWidthsA;
mod get_char_abc_widths_w;
pub use get_char_abc_widths_w::GetCharABCWidthsW;
mod get_char_width32_a;
pub use get_char_width32_a::GetCharWidth32A;
mod get_char_width32_w;
pub use get_char_width32_w::GetCharWidth32W;
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
mod get_object_a;
pub use get_object_a::GetObjectA;
mod get_object_type;
pub use get_object_type::GetObjectType;
mod get_object_w;
pub use get_object_w::GetObjectW;
mod get_pixel_gdi;
pub use get_pixel_gdi::GetPixelGdi;
mod get_poly_fill_mode;
pub use get_poly_fill_mode::GetPolyFillMode;
mod get_rop2;
pub use get_rop2::GetROP2;
mod get_stock_object;
pub use get_stock_object::GetStockObject;
mod get_stretch_blt_mode;
pub use get_stretch_blt_mode::GetStretchBltMode;
mod get_system_palette_entries;
pub use get_system_palette_entries::GetSystemPaletteEntries;
mod get_text_align;
pub use get_text_align::GetTextAlign;
mod get_text_color;
pub use get_text_color::GetTextColor;
mod get_text_extent_point32_a;
pub use get_text_extent_point32_a::GetTextExtentPoint32A;
mod get_text_extent_point32_w;
pub use get_text_extent_point32_w::GetTextExtentPoint32W;
mod get_text_face_a;
pub use get_text_face_a::GetTextFaceA;
mod get_text_face_w;
pub use get_text_face_w::GetTextFaceW;
mod get_text_metrics_a;
pub use get_text_metrics_a::GetTextMetricsA;
mod get_text_metrics_w;
pub use get_text_metrics_w::GetTextMetricsW;
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
mod remove_font_resource_ex_a;
pub use remove_font_resource_ex_a::RemoveFontResourceExA;
mod remove_font_resource_ex_w;
pub use remove_font_resource_ex_w::RemoveFontResourceExW;
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
mod text_out_a;
pub use text_out_a::TextOutA;
mod text_out_w;
pub use text_out_w::TextOutW;

pub fn gateway(addr: u32, emu: &mut emu::Emu) -> String {
    let api = kernel32::guess_api_name(emu, addr);
    let api = api.split("!").last().unwrap_or(&api);
    match api {
        // DC operations
        "CreateDCA" => CreateDCA(emu),
        "CreateDCW" => CreateDCW(emu),
        "CreateCompatibleDC" => CreateCompatibleDC(emu),
        "DeleteDC" => DeleteDC(emu),
        "GetDC" => GetDC(emu),
        "ReleaseDC" => ReleaseDC(emu),
        "SaveDC" => SaveDC(emu),
        "RestoreDC" => RestoreDC(emu),

        // Bitmap
        "CreateCompatibleBitmap" => CreateCompatibleBitmap(emu),
        "CreateDIBSection" => CreateDIBSection(emu),
        "BitBlt" => BitBlt(emu),
        "StretchBlt" => StretchBlt(emu),
        "PatBlt" => PatBlt(emu),
        "SetDIBits" => SetDIBits(emu),
        "GetDIBits" => GetDIBits(emu),
        "GetDIBColorTable" => GetDIBColorTable(emu),
        "SetBkMode" => SetBkMode(emu),
        "SetBkColor" => SetBkColor(emu),

        // Object
        "SelectObject" => SelectObject(emu),
        "DeleteObject" => DeleteObject(emu),
        "GetObjectA" => GetObjectA(emu),
        "GetObjectW" => GetObjectW(emu),
        "GetObjectType" => GetObjectType(emu),
        "GetStockObject" => GetStockObject(emu),
        "GetCurrentObject" => GetCurrentObject(emu),
        "GetDeviceCaps" => GetDeviceCaps(emu),

        // Font
        "CreateFontIndirectA" => CreateFontIndirectA(emu),
        "CreateFontIndirectW" => CreateFontIndirectW(emu),
        "CreateFontA" => CreateFontA(emu),
        "CreateFontW" => CreateFontW(emu),
        "GetTextMetricsA" => GetTextMetricsA(emu),
        "GetTextMetricsW" => GetTextMetricsW(emu),
        "GetTextExtentPoint32A" => GetTextExtentPoint32A(emu),
        "GetTextExtentPoint32W" => GetTextExtentPoint32W(emu),
        "GetTextFaceA" => GetTextFaceA(emu),
        "GetTextFaceW" => GetTextFaceW(emu),
        "AddFontResourceExA" => AddFontResourceExA(emu),
        "AddFontResourceExW" => AddFontResourceExW(emu),
        "RemoveFontResourceExA" => RemoveFontResourceExA(emu),
        "RemoveFontResourceExW" => RemoveFontResourceExW(emu),
        "GetCharWidth32A" => GetCharWidth32A(emu),
        "GetCharWidth32W" => GetCharWidth32W(emu),
        "GetCharABCWidthsA" => GetCharABCWidthsA(emu),
        "GetCharABCWidthsW" => GetCharABCWidthsW(emu),
        "EnumFontFamiliesExA" => EnumFontFamiliesExA(emu),
        "EnumFontFamiliesExW" => EnumFontFamiliesExW(emu),

        // Text
        "TextOutA" => TextOutA(emu),
        "TextOutW" => TextOutW(emu),
        "ExtTextOutA" => ExtTextOutA(emu),
        "ExtTextOutW" => ExtTextOutW(emu),
        "SetTextColor" => SetTextColor(emu),
        "GetTextColor" => GetTextColor(emu),
        "SetTextAlign" => SetTextAlign(emu),
        "GetTextAlign" => GetTextAlign(emu),

        // Drawing
        "Rectangle" => GdiRectangle(emu),
        "Ellipse" => GdiEllipse(emu),
        "MoveToEx" => MoveToEx(emu),
        "LineTo" => LineTo(emu),
        "SetPixel" => SetPixel(emu),
        "GetPixel" => GetPixelGdi(emu),
        "FillRect" => FillRect(emu),
        "Polygon" => GdiPolygon(emu),
        "Polyline" => GdiPolyline(emu),
        "PolyBezier" => GdiPolyBezier(emu),
        "Arc" => GdiArc(emu),
        "Pie" => GdiPie(emu),
        "Chord" => GdiChord(emu),

        // Region
        "CreateRectRgn" => CreateRectRgn(emu),
        "CreateRectRgnIndirect" => CreateRectRgnIndirect(emu),
        "GetClipBox" => GetClipBox(emu),
        "CombineRgn" => CombineRgn(emu),
        "SelectClipRgn" => SelectClipRgn(emu),
        "IntersectClipRect" => IntersectClipRect(emu),
        "ExcludeClipRect" => ExcludeClipRect(emu),

        // Pen/Brush
        "CreatePen" => CreatePen(emu),
        "CreatePenIndirect" => CreatePenIndirect(emu),
        "CreateSolidBrush" => CreateSolidBrush(emu),
        "CreateBrushIndirect" => CreateBrushIndirect(emu),
        "CreatePatternBrush" => CreatePatternBrush(emu),
        "CreateHatchBrush" => CreateHatchBrush(emu),

        // Map/viewport
        "SetMapMode" => SetMapMode(emu),
        "GetMapMode" => GetMapMode(emu),
        "SetWindowOrgEx" => SetWindowOrgEx(emu),
        "SetViewportOrgEx" => SetViewportOrgEx(emu),

        // ROP/mode
        "SetROP2" => SetROP2(emu),
        "GetROP2" => GetROP2(emu),
        "SetStretchBltMode" => SetStretchBltMode(emu),
        "GetStretchBltMode" => GetStretchBltMode(emu),
        "SetPolyFillMode" => SetPolyFillMode(emu),
        "GetPolyFillMode" => GetPolyFillMode(emu),

        // Transform
        "SetWorldTransform" => SetWorldTransform(emu),
        "GetWorldTransform" => GetWorldTransform(emu),

        // Printer
        "StartDoc" | "StartDocA" | "StartDocW" => StartDoc(emu),
        "EndDoc" => EndDoc(emu),
        "StartPage" => StartPage(emu),
        "EndPage" => EndPage(emu),

        // Palette
        "GetSystemPaletteEntries" => GetSystemPaletteEntries(emu),
        "GetNearestColor" => GetNearestColor(emu),

        // DC colors
        "SetDCBrushColor" => SetDCBrushColor(emu),
        "SetDCPenColor" => SetDCPenColor(emu),

        // Layout
        "GetLayout" => GetLayout(emu),
        "SetLayout" => SetLayout(emu),

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

// === Region ===

// === Pen/Brush ===

// === Map/Viewport ===

// === ROP/mode ===

// === Transform ===

// === Printer ===

// === Palette ===

// === DC colors ===

// === Layout ===
