//! The contract every binary format implements, and the helpers they share.

use std::path::Path;

use crate::emu::Emu;
use crate::err::MwemuError;
use crate::windows::constants::memory::CFG_DEFAULT_BASE;

/// What a format loader is given: the bytes read from disk and the path they
/// came from.
pub(crate) struct LoadInput<'a> {
    /// Path of the file being loaded.
    pub(crate) path: &'a str,
    /// Contents of that file.
    pub(crate) bytes: &'a [u8],
}

/// One binary format.
///
/// `load` maps the image into the guest and leaves the CPU where the OS
/// would have started it. Everything specific to a format — the emulator
/// shape it needs, the image mapping, the entry point fix-ups — belongs to
/// the implementation, not to the dispatcher.
pub(crate) trait FormatLoader {
    fn load(&self, emu: &mut Emu, input: &LoadInput<'_>) -> Result<(), MwemuError>;
}

/// Apply the `--entry-point` override: start at the address the operator
/// asked for instead of the image's own entry point.
pub(crate) fn apply_entry_point_override(emu: &mut Emu) {
    if emu.cfg.entry_point != CFG_DEFAULT_BASE {
        emu.regs_mut().rip = emu.cfg.entry_point;
    }
}

/// Enter a linked kernel object at its init function, with the kernel's halt
/// pad as the return address — what `insmod`/`kextload` would return to.
pub(crate) fn enter_kernel_init(emu: &mut Emu) {
    let retpad = emu.kernel.as_ref().map(|k| k.layout.retpad()).unwrap_or(0);
    if let Some(init) = emu.kernel.as_ref().and_then(|k| k.module.init) {
        emu.stack_push64(retpad);
        emu.set_pc(init);
    }
}

/// Point `cfg.maps_folder` at the system-DLL folder of `platform`/`arch`.
///
/// The CLI picks a folder before it knows what the input is, so an already
/// chosen folder is kept — except for macOS, where `replace_windows` also
/// swaps out a folder that still points at the Windows DLLs.
pub(crate) fn set_maps_folder(emu: &mut Emu, platform: &str, arch: &str, replace_windows: bool) {
    let current = emu.cfg.maps_folder.as_str();
    let replace = if replace_windows {
        current.is_empty() || current.contains("windows")
    } else {
        current.is_empty()
    };
    if !replace {
        return;
    }
    for candidate in [
        format!("maps/{platform}/{arch}"),
        format!("../../maps/{platform}/{arch}"),
    ] {
        if Path::new(&candidate).exists() {
            emu.cfg.maps_folder = format!("{candidate}/");
            return;
        }
    }
}
