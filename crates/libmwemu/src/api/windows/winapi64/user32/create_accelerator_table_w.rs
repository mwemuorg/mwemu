use crate::emu;
use crate::winapi::helper;

pub fn CreateAcceleratorTableW(emu: &mut emu::Emu) {
    let accel_ptr = emu.regs().rcx;
    let num_entries = emu.regs().rdx as u32;
    log_red!(
        emu,
        "user32!CreateAcceleratorTableW accel:0x{:x} count:{}",
        accel_ptr,
        num_entries
    );
    let hndl = helper::handler_create("user32://CreateAcceleratorTableW");
    emu.regs_mut().rax = hndl;
}
