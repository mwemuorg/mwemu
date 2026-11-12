use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn CreateBoundaryDescriptorA(emu: &mut emu::Emu) {
    let Name = WinApiAbi::arg(emu, 0) as u32;
    let _Flags = WinApiAbi::arg(emu, 1) as u32;

    let Name = emu.maps.read_string(Name as u64);

    log_red!(emu, "kernel32!CreateBoundaryDescriptorA {}", Name);

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}

pub fn CreateBoundaryDescriptorW(emu: &mut emu::Emu) {
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
    log_red!(emu, "kernel32!CreateBoundaryDescriptorW");
    emu.regs_mut().rax = helper::handler_create("CreateBoundaryDescriptorW");
}
