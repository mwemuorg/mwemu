use crate::emu::Emu;

impl Emu {
    /// Enable banzai mode, in this mode on the windows api of 32bits, if the called api is not
    /// implemented will try to fix the stack (because knows the number of params of every api) and
    /// will continue the emulation.
    pub fn enable_banzai(&mut self) {
        self.cfg.skip_unimplemented = true;
        self.maps.set_banzai(true);
    }

    /// Disable the banzai mode, if the emualted code call an unimplemented 32bits winapis, the
    /// emulation will stop.
    pub fn disable_banzai(&mut self) {
        self.cfg.skip_unimplemented = false;
        self.maps.set_banzai(false);
    }

    /// Add windows 32bits apis to the banzai mode, with this info mwemu will know how to continue
    /// the emulating inf this api is found and is not implemented.
    pub fn banzai_add(&mut self, name: &str, nparams: i32) {
        self.banzai.add(name, nparams);
    }
}
