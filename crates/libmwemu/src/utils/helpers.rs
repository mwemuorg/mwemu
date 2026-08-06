use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

static COLOR_ENABLED: AtomicBool = AtomicBool::new(true);

// TODO: remove the code when 'likely' and 'unlikely' are stable
#[inline(always)]
#[cold]
fn cold_path() {}

#[inline(always)]
pub(crate) fn likely(b: bool) -> bool {
    if b {
        true
    } else {
        cold_path();
        false
    }
}

#[inline(always)]
pub(crate) fn unlikely(b: bool) -> bool {
    if b {
        cold_path();
        true
    } else {
        false
    }
}

// This array is for the get_operand_size in order to make it query faster
//TODO: OpKind::Immediate8to64 could be 8
pub const OP_KIND_BIT_WIDTH: [u8; 25] = [
    255, // 0  Register
    16,  // 1  NearBranch16
    32,  // 2  NearBranch32
    64,  // 3  NearBranch64
    16,  // 4  FarBranch16
    32,  // 5  FarBranch32
    8,   // 6  Immediate8
    8,   // 7  Immediate8_2nd
    16,  // 8  Immediate16
    32,  // 9  Immediate32
    64,  // 10 Immediate64
    16,  // 11 Immediate8to16
    32,  // 12 Immediate8to32
    64,  // 13 Immediate8to64
    64,  // 14 Immediate32to64
    16,  // 15 MemorySegSI
    32,  // 16 MemorySegESI
    64,  // 17 MemorySegRSI
    16,  // 18 MemorySegDI
    32,  // 19 MemorySegEDI
    64,  // 20 MemorySegRDI
    16,  // 21 MemoryESDI
    32,  // 22 MemoryESEDI
    64,  // 23 MemoryESRDI
    255, // 24 Memory
];

pub fn disable_color() {
    COLOR_ENABLED.store(false, Ordering::Relaxed);
}

pub fn enable_color() {
    COLOR_ENABLED.store(true, Ordering::Relaxed);
}

#[inline]
pub fn color_enabled() -> bool {
    COLOR_ENABLED.load(Ordering::Relaxed)
}

#[macro_export]
macro_rules! color {
    ("Black") => {
        if $crate::color_enabled() {
            "\x1b[0;30m"
        } else {
            ""
        }
    };
    ("Red") => {
        if $crate::color_enabled() {
            "\x1b[0;31m"
        } else {
            ""
        }
    };
    ("Green") => {
        if $crate::color_enabled() {
            "\x1b[0;32m"
        } else {
            ""
        }
    };
    ("Orange") => {
        if $crate::color_enabled() {
            "\x1b[0;33m"
        } else {
            ""
        }
    };
    ("Blue") => {
        if $crate::color_enabled() {
            "\x1b[0;34m"
        } else {
            ""
        }
    };
    ("Purple") => {
        if $crate::color_enabled() {
            "\x1b[0;35m"
        } else {
            ""
        }
    };
    ("Cyan") => {
        if $crate::color_enabled() {
            "\x1b[0;36m"
        } else {
            ""
        }
    };
    ("LightGray") => {
        if $crate::color_enabled() {
            "\x1b[0;37m"
        } else {
            ""
        }
    };
    ("DarkGray") => {
        if $crate::color_enabled() {
            "\x1b[1;30m"
        } else {
            ""
        }
    };
    ("LightRed") => {
        if $crate::color_enabled() {
            "\x1b[1;31m"
        } else {
            ""
        }
    };
    ("LightGreen") => {
        if $crate::color_enabled() {
            "\x1b[1;32m"
        } else {
            ""
        }
    };
    ("Yellow") => {
        if $crate::color_enabled() {
            "\x1b[1;33m"
        } else {
            ""
        }
    };
    ("LightBlue") => {
        if $crate::color_enabled() {
            "\x1b[1;34m"
        } else {
            ""
        }
    };
    ("LightPurple") => {
        if $crate::color_enabled() {
            "\x1b[1;35m"
        } else {
            ""
        }
    };
    ("LightCyan") => {
        if $crate::color_enabled() {
            "\x1b[1;36m"
        } else {
            ""
        }
    };
    ("White") => {
        if $crate::color_enabled() {
            "\x1b[1;37m"
        } else {
            ""
        }
    };
    ("nc") => {
        if $crate::color_enabled() {
            "\x1b[0m"
        } else {
            ""
        }
    };
    ("ClearScreen") => {
        if $crate::color_enabled() { "\x1bc" } else { "" }
    };
    ($unknown:tt) => {
        compile_error!(concat!(
            "Unknown color name: '",
            $unknown,
            "'. Valid options are: \
            Black, Red, Green, Orange, Blue, Purple, Cyan, LightGray, \
            DarkGray, LightRed, LightGreen, Yellow, LightBlue, \
            LightPurple, LightCyan, White, nc, ClearScreen"
        ))
    };
}

pub fn filename_no_ext(path: &str) -> Option<String> {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}
