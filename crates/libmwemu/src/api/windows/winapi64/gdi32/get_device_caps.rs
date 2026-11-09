use crate::emu;
use crate::winapi::helper;

pub fn GetDeviceCaps(emu: &mut emu::Emu) {
    let hdc = emu.regs().rcx;
    let index = emu.regs().rdx as i32;
    // Windows GetDeviceCaps index constants (wingdi.h)
    let result: u64 = match index {
        0 => 1,            // DRIVERVERSION
        2 => 0,            // TECHNOLOGY (DT_RASDISPLAY)
        4 => 508,          // HORZSIZE (mm, ~20" for 1920px at 96dpi)
        6 => 285,          // VERTSIZE (mm, ~11.25" for 1080px at 96dpi)
        8 => 1920,         // HORZRES (pixels)
        10 => 1080,        // VERTRES (pixels)
        12 => 32,          // BITSPIXEL (bits per pixel)
        14 => 32,          // BITSPIXEL alias (some callers use this index)
        16 => 0xFFFF_FFFF, // NUMBRUSHES (-1 as DWORD = device-supported)
        18 => 0xFFFF_FFFF, // NUMPENS (-1 as DWORD = device-supported)
        20 => 0,           // NUMMARKERS
        22 => 0,           // NUMFONTS
        24 => 0,           // NUMCOLORS (0 for >8bpp)
        26 => 0,           // PDEVICESIZE
        28 => 0x1ff,       // CURVECAPS (all curve capabilities)
        30 => 0x1ff,       // LINECAPS (all line capabilities)
        32 => 0x1ff,       // POLYGONALCAPS (all polygon capabilities)
        34 => 0x7fff,      // TEXTCAPS (all text capabilities)
        36 => 1,           // CLIPCAPS (CP_RECTANGLE)
        38 => 0x7e99,      // RASTERCAPS
        40 => 1,           // ASPECTX (relative width of pixel)
        42 => 1,           // ASPECTY (relative height of pixel)
        44 => 1,           // ASPECTXY (diagonal aspect ratio)
        88 => 96,          // LOGPIXELSX (logical pixels per inch X)
        90 => 96,          // LOGPIXELSY (logical pixels per inch Y)
        104 => 256,        // SIZEPALETTE
        106 => 20,         // NUMRESERVED
        108 => 18,         // COLORRES (bits of color resolution)
        110 => 0,          // PHYSICALWIDTH (printer only)
        111 => 0,          // PHYSICALHEIGHT (printer only)
        112 => 0,          // PHYSICALOFFSETX (printer only)
        113 => 0,          // PHYSICALOFFSETY (printer only)
        114 => 0,          // SCALINGFACTORX
        115 => 0,          // SCALINGFACTORY
        116 => 60,         // VREFRESH (Hz)
        117 => 1080,       // DESKTOPVERTRES
        118 => 1920,       // DESKTOPHORZRES
        119 => 0,          // BLTALIGNMENT
        120 => 0x1ff,      // SHADEBLENDCAPS
        121 => 0x7f,       // COLORMGMTCAPS
        _ => 0,
    };
    log_red!(
        emu,
        "gdi32!GetDeviceCaps hdc:0x{:x} index:{} = {}",
        hdc,
        index,
        result
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
