use crate::emu;

pub fn SetTapePosition(emu: &mut emu::Emu) {
    let _hDevice = emu.regs().rcx as u32;
    let _dwPositionMethod = emu.regs().rdx as u32;
    let _dwPartition = emu.regs().r8 as u32;
    let _dwOffsetLow = emu.regs().r9 as u32;
    let _dwOffsetHigh = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _bImmediate = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!SetTapePosition");
    let val_ptr = emu.regs().rdx;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
