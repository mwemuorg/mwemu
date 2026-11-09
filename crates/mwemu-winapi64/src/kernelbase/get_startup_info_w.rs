use crate::emu;

pub fn GetStartupInfoW(emu: &mut emu::Emu) {
    let info_ptr = emu.regs().rcx;
    if info_ptr != 0 {
        let cb = if true { 104u32 } else { 68u32 };
        emu.maps.write_dword(info_ptr, cb);
    }
    log_red!(emu, "kernelbase!GetStartupInfoW");
}
