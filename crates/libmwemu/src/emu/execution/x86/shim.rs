use std::cell::Cell;

use super::Emu;

// API-shim entry-point cache. Resolved ONCE on the first time execution
// enters the loader-DLL VA range; thereafter every per-instruction
// lookup is a single thread-local Cell read with no locks or hashing.
// `0xFFFF_FFFF_FFFF_FFFF` is the "tried-and-resolved" sentinel for
// symbols whose module is loaded but the symbol itself is missing — we
// still want to skip the export-walk on every subsequent instruction.
thread_local! {
    static SHIM_TABLE: Cell<Option<ShimTable>> = const { Cell::new(None) };
}

#[derive(Clone, Copy, Default)]
pub(super) struct ShimTable {
    pub lla: u64,
    pub lpa: u64,
    pub lpa2: u64,
    pub mba: u64,
}

impl Emu {
    #[inline]
    pub(super) fn shim_table(&mut self) -> ShimTable {
        let cached = SHIM_TABLE.with(|c| c.get());
        // kernelbase symbols are always present once kernelbase is loaded
        // (which happens during early `--ssdt` setup). user32 is loaded
        // on-demand by the LoadLibraryA shim, so mba may resolve to 0 on
        // the first pass — refresh it lazily once user32 appears.
        if let Some(t) = cached {
            if t.mba != 0 || self.maps.get_map_by_name("user32.pe").is_none() {
                return t;
            }
            // user32 is loaded now but mba was 0 — re-resolve just mba.
            let mba = crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "user32.dll",
                "MessageBoxA",
            );
            let new = ShimTable { mba, ..t };
            SHIM_TABLE.with(|c| c.set(Some(new)));
            if self.cfg.verbose >= 1 {
                log::trace!("shim table mba resolved: 0x{:x}", mba);
            }
            return new;
        }
        let t = ShimTable {
            lla: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "kernelbase.dll",
                "LoadLibraryA",
            ),
            lpa: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "kernelbase.dll",
                "GetProcAddress",
            ),
            lpa2: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "kernelbase.dll",
                "GetProcAddressForCaller",
            ),
            mba: crate::winapi::winapi64::kernel32::resolve_api_name_in_module(
                self,
                "user32.dll",
                "MessageBoxA",
            ),
        };
        SHIM_TABLE.with(|c| c.set(Some(t)));
        if self.cfg.verbose >= 1 {
            log::trace!(
                "shim table resolved: LLA=0x{:x} GPA=0x{:x} GPA-FC=0x{:x} MBA=0x{:x}",
                t.lla,
                t.lpa,
                t.lpa2,
                t.mba,
            );
        }
        t
    }
}
