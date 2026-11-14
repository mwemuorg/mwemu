use crate::emu;

pub fn GetProcessVersion(emu: &mut emu::Emu) {
    let process_id = emu.regs().rcx;
    let _ = emu.maps.is_mapped(process_id);
    let version = 0x000A_0000u64;
    log_red!(
        emu,
        "kernelbase!GetProcessVersion pid:{} => 0x{:x}",
        process_id,
        version
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = version;
}
