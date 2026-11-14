use crate::emu;

pub fn CryptXmlGetTransforms(emu: &mut emu::Emu) {
    let pp_transforms = emu.regs().rcx;
    log_red!(
        emu,
        "crypt32!CryptXmlGetTransforms ppTransforms:0x{:x}",
        pp_transforms
    );
    if pp_transforms != 0 {
        emu.maps.write_qword(pp_transforms, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
