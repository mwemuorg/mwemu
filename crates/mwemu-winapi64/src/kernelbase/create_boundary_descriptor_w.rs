use crate::emu;
use crate::winapi::helper;

pub fn CreateBoundaryDescriptorW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let name = emu.maps.read_wide_string(arg1);
    let hndl = helper::handler_create(&format!("kb_{}", name));
    log_red!(
        emu,
        "kernelbase!CreateBoundaryDescriptorW '{}' => 0x{:x}",
        name,
        hndl
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
