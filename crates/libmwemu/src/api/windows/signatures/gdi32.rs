use super::{Parameter, ReturnType, Signature};

pub(super) fn lookup(name: &str) -> Option<Signature> {
    match name {
        "BitBlt" => {
            signature!(Win32Bool, 9, 9; Pointer(hdc), Dword(x), Dword(y), Dword(cx), Dword(cy), Pointer(hdc_src), Dword(x1), Dword(y1), Dword(rop))
        }
        "CombineRgn" => {
            signature!(Generic, 4, 4; Pointer(hrgn_dst), Pointer(hrgn_src1), Pointer(hrgn_src2), Dword(i_mode))
        }
        "CreateBrushIndirect" => signature!(Generic, 1, 1; Pointer(plbrush)),
        "CreateDIBSection" => {
            signature!(Generic, 6, 6; Pointer(hdc), Pointer(pbmi), Dword(usage), Pointer(ppv_bits), Pointer(h_section), Dword(offset))
        }
        "CreateFontA" => {
            signature!(Generic, 14, 14; Dword(c_height), Dword(c_width), Dword(c_escapement), Dword(c_orientation), Dword(c_weight), Dword(b_italic), Dword(b_underline), Dword(b_strike_out), Dword(i_char_set), Dword(i_out_precision), Dword(i_clip_precision), Dword(i_quality), Dword(i_pitch_and_family), AnsiString(psz_face_name))
        }
        "CreateFontIndirectA" | "CreateFontIndirectW" => signature!(Generic, 1, 1; Pointer(lplf)),
        "CreateFontW" => {
            signature!(Generic, 14, 14; Dword(c_height), Dword(c_width), Dword(c_escapement), Dword(c_orientation), Dword(c_weight), Dword(b_italic), Dword(b_underline), Dword(b_strike_out), Dword(i_char_set), Dword(i_out_precision), Dword(i_clip_precision), Dword(i_quality), Dword(i_pitch_and_family), WideString(psz_face_name))
        }
        "CreatePatternBrush" => signature!(Generic, 1, 1; Pointer(hbm)),
        "CreatePenIndirect" => signature!(Generic, 1, 1; Pointer(plpen)),
        "CreateRectRgn" => signature!(Generic, 4, 4; Dword(x1), Dword(y1), Dword(x2), Dword(y2)),
        "CreateRectRgnIndirect" => signature!(Generic, 1, 1; Pointer(lprect)),
        "EndDoc" | "EndPage" | "GetLayout" | "GetMapMode" | "GetPolyFillMode" | "GetROP2"
        | "GetStretchBltMode" | "GetTextAlign" | "StartPage" => {
            signature!(Generic, 1, 1; Pointer(hdc))
        }
        "EnumFontFamiliesExA" | "EnumFontFamiliesExW" => {
            signature!(Generic, 5, 5; Pointer(hdc), Pointer(lp_logfont), Pointer(lp_proc), Pointer(l_param), Dword(dw_flags))
        }
        "ExcludeClipRect" | "IntersectClipRect" => {
            signature!(Generic, 5, 5; Pointer(hdc), Dword(left), Dword(top), Dword(right), Dword(bottom))
        }
        "ExtTextOutA" | "ExtTextOutW" => {
            signature!(Generic, 8, 8; Pointer(hdc), Dword(x), Dword(y), Dword(options), Pointer(lprect), Pointer(lp_string), Dword(c), Pointer(lp_dx))
        }
        "GetDIBColorTable" => {
            signature!(Generic, 4, 4; Pointer(hdc), Dword(i_start), Dword(c_entries), Pointer(prgbq))
        }
        "GetDIBits" => {
            signature!(Generic, 7, 7; Pointer(hdc), Pointer(hbm), Dword(start), Dword(c_lines), Pointer(lpv_bits), Pointer(lpbmi), Dword(usage))
        }
        "GetSystemPaletteEntries" => {
            signature!(Generic, 4, 4; Pointer(hdc), Dword(i_start), Dword(c_entries), Pointer(p_pal_entries))
        }
        "TextOutA" | "TextOutW" => {
            signature!(Win32Bool,5,5; Pointer(hdc), Dword(x), Dword(y), Pointer(text), Dword(count))
        }
        "GetPixel" => signature!(Generic,3,3; Pointer(hdc), Dword(x), Dword(y)),
        "LineTo" => signature!(Win32Bool, 3, 3; Pointer(hdc), Dword(x), Dword(y)),
        "MoveToEx" => signature!(Generic, 4, 4; Pointer(hdc), Dword(x), Dword(y), Pointer(lppt)),
        "PatBlt" => {
            signature!(Generic, 6, 6; Pointer(hdc), Dword(x), Dword(y), Dword(w), Dword(h), Dword(rop))
        }
        "RemoveFontResourceExA" => {
            signature!(Generic, 3, 3; AnsiString(name), Dword(fl), Pointer(pdv))
        }
        "RemoveFontResourceExW" => {
            signature!(Generic, 3, 3; WideString(name), Dword(fl), Pointer(pdv))
        }
        "SelectClipRgn" => signature!(Generic, 2, 2; Pointer(hdc), Pointer(hrgn)),
        "SetDIBits" => {
            signature!(Generic, 7, 7; Pointer(hdc), Pointer(hbm), Dword(start), Dword(c_lines), Pointer(lp_bits), Pointer(lpbmi), Dword(color_use))
        }
        "SetPixel" => signature!(Generic, 4, 4; Pointer(hdc), Dword(x), Dword(y), Dword(color)),
        "StretchBlt" => {
            signature!(Generic, 11, 11; Pointer(hdc_dest), Dword(x_dest), Dword(y_dest), Dword(w_dest), Dword(h_dest), Pointer(hdc_src), Dword(x_src), Dword(y_src), Dword(w_src), Dword(h_src), Dword(rop))
        }
        "AddFontResourceExA" => {
            signature!(Generic, 3, 3; AnsiString(name), Dword(flags), Pointer(reserved))
        }
        "AddFontResourceExW" => {
            signature!(Generic, 3, 3; WideString(name), Dword(flags), Pointer(reserved))
        }
        "StartDocA" => signature!(Generic, 2, 2; Pointer(dc), Pointer(doc_info)),
        "StartDocW" => signature!(Generic, 2, 2; Pointer(dc), Pointer(doc_info)),
        "Polygon" => signature!(Win32Bool, 3, 3; Pointer(dc), Pointer(points), Dword(count)),
        "SelectObject" => signature!(Generic, 2, 2; Pointer(dc), Pointer(object)),
        "Arc" => {
            signature!(Win32Bool, 9, 9; Pointer(dc), Dword(left), Dword(top), Dword(right), Dword(bottom), Dword(start_x), Dword(start_y), Dword(end_x), Dword(end_y))
        }
        "Chord" => {
            signature!(Win32Bool, 9, 9; Pointer(dc), Dword(left), Dword(top), Dword(right), Dword(bottom), Dword(start_x), Dword(start_y), Dword(end_x), Dword(end_y))
        }
        "Pie" => {
            signature!(Win32Bool, 9, 9; Pointer(dc), Dword(left), Dword(top), Dword(right), Dword(bottom), Dword(start_x), Dword(start_y), Dword(end_x), Dword(end_y))
        }
        "Ellipse" => {
            signature!(Win32Bool, 5, 5; Pointer(dc), Dword(left), Dword(top), Dword(right), Dword(bottom))
        }
        "Rectangle" => {
            signature!(Win32Bool, 5, 5; Pointer(dc), Dword(left), Dword(top), Dword(right), Dword(bottom))
        }
        "Polyline" => signature!(Win32Bool, 3, 3; Pointer(dc), Pointer(points), Dword(count)),
        "PolyBezier" => signature!(Win32Bool, 3, 3; Pointer(dc), Pointer(points), Dword(count)),
        "GetCharWidth32A" => {
            signature!(Win32Bool, 4, 4; Pointer(dc), Dword(first_char), Dword(last_char), Pointer(widths))
        }
        "GetCharWidth32W" => {
            signature!(Win32Bool, 4, 4; Pointer(dc), Dword(first_char), Dword(last_char), Pointer(widths))
        }
        "GetCharABCWidthsA" => {
            signature!(Win32Bool, 4, 4; Pointer(dc), Dword(first_char), Dword(last_char), Pointer(widths))
        }
        "GetCharABCWidthsW" => {
            signature!(Win32Bool, 4, 4; Pointer(dc), Dword(first_char), Dword(last_char), Pointer(widths))
        }
        "GetTextExtentPoint32A" => {
            signature!(Win32Bool, 4, 4; Pointer(dc), Pointer(text), Dword(count), Pointer(size))
        }
        "GetTextExtentPoint32W" => {
            signature!(Win32Bool, 4, 4; Pointer(dc), Pointer(text), Dword(count), Pointer(size))
        }
        "GetTextMetricsA" => signature!(Win32Bool, 2, 2; Pointer(dc), Pointer(metrics)),
        "GetTextMetricsW" => signature!(Win32Bool, 2, 2; Pointer(dc), Pointer(metrics)),
        _ => None,
    }
}
