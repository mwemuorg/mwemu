use crate::emu;

pub fn BasepSetFileEncryptionCompression(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!BasepSetFileEncryptionCompression param1:0x{:x} param2:0x{:x} param3:0x{:x} param4:0x{:x}",
        param1,
        param2,
        param3,
        param4
    );
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
