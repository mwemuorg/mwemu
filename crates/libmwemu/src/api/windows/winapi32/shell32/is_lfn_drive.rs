use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn IsLFNDrive(emu: &mut emu::Emu) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    log_red!(
        emu,
        "shell32!IsLFNDrive arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    emu.regs_mut().rax = 1;
}

pub fn IsLFNDriveA(emu: &mut emu::Emu) {
    is_lfn_drive(emu, "IsLFNDriveA");
}

fn is_lfn_drive(emu: &mut emu::Emu, api: &str) {
    let arg1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let arg2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    log_red!(emu, "shell32!{api} arg1:0x{:x} arg2:0x{:x}", arg1, arg2);
    emu.set_last_error(0);
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    emu.regs_mut().rax = 1;

    stdcall_ret(emu, 1);
}

pub fn IsLFNDriveW(emu: &mut emu::Emu) {
    is_lfn_drive(emu, "IsLFNDriveW");
}
