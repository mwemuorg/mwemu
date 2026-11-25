use std::cell::Cell;

use super::Emu;

// API-shim entry-point cache. Resolved lazily when execution first enters
// the loader-DLL VA range; thereafter every per-instruction lookup is a
// single thread-local Cell read with no locks or hashing. Each group of
// symbols is re-resolved once its host module appears in the maps — the
// shim table may first be queried during LdrInitializeThunk before
// kernelbase or user32 are loaded.
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
    pub(in crate::emu::execution) fn invalidate_shim_table(&self) {
        SHIM_TABLE.with(|c| c.set(None));
    }

    #[inline]
    pub(super) fn shim_table(&mut self) -> ShimTable {
        let cached = SHIM_TABLE.with(|c| c.get());
        if let Some(t) = cached {
            let need_kb = t.lla == 0 && self.maps.get_map_by_name("kernelbase.pe").is_some();
            let need_mba = t.mba == 0 && self.maps.get_map_by_name("user32.pe").is_some();
            if !need_kb && !need_mba {
                return t;
            }
            let resolve = crate::winapi::winapi64::kernel32::resolve_api_name_in_module;
            let new = ShimTable {
                lla: if need_kb {
                    resolve(self, "kernelbase.dll", "LoadLibraryA")
                } else {
                    t.lla
                },
                lpa: if need_kb {
                    resolve(self, "kernelbase.dll", "GetProcAddress")
                } else {
                    t.lpa
                },
                lpa2: if need_kb {
                    resolve(self, "kernelbase.dll", "GetProcAddressForCaller")
                } else {
                    t.lpa2
                },
                mba: if need_mba {
                    resolve(self, "user32.dll", "MessageBoxA")
                } else {
                    t.mba
                },
            };
            SHIM_TABLE.with(|c| c.set(Some(new)));
            return new;
        }
        let resolve = crate::winapi::winapi64::kernel32::resolve_api_name_in_module;
        let t = ShimTable {
            lla: resolve(self, "kernelbase.dll", "LoadLibraryA"),
            lpa: resolve(self, "kernelbase.dll", "GetProcAddress"),
            lpa2: resolve(self, "kernelbase.dll", "GetProcAddressForCaller"),
            mba: resolve(self, "user32.dll", "MessageBoxA"),
        };
        SHIM_TABLE.with(|c| c.set(Some(t)));
        t
    }
}
