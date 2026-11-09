use crate::emu;
use crate::winapi::helper;

pub fn CreateBoundaryDescriptorA(emu: &mut emu::Emu) {
    let Name = emu.regs().rcx as u32;
    let _Flags = emu.regs().rdx as u32;

    let Name = emu.maps.read_string(Name as u64);

    log_red!(emu, "kernel32!CreateBoundaryDescriptorA {}", Name);

    emu.regs_mut().rax = helper::handler_create("CreateBoundaryDescriptorA");
}
