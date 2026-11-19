use crate::emu::Emu;

impl Emu {
    /// Enable instruction/memory Banzai flags and the x86 cleanup fallback.
    /// API mode controls simulation; subsystem ABI checks retain precedence.
    pub fn enable_banzai(&mut self) {
        self.cfg.skip_unimplemented = true;
        self.maps.set_banzai(true);
    }

    /// Clear Banzai flags without changing the API mode.
    pub fn disable_banzai(&mut self) {
        self.cfg.skip_unimplemented = false;
        self.maps.set_banzai(false);
    }

    /// Set fallback x86 callee cleanup in DWORD slots, excluding the return address.
    pub fn banzai_add(&mut self, name: &str, nparams: i32) {
        self.banzai.add(name, nparams);
    }
}
