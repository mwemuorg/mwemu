use crate::emu;

pub fn IsLFNDrive(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!IsLFNDrive arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn IsLFNDriveA(emu: &mut emu::Emu) {
    is_lfn_drive(emu, "IsLFNDriveA");
}

fn is_lfn_drive(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx as u32;
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn IsLFNDriveW(emu: &mut emu::Emu) {
    is_lfn_drive(emu, "IsLFNDriveW");
}
