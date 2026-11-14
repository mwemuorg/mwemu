use crate::emu;
use crate::winapi::helper;

pub fn CreateAcceleratorTableA(emu: &mut emu::Emu) {
    create_accelerator_table(emu, "CreateAcceleratorTableA");
}

fn create_accelerator_table(emu: &mut emu::Emu, api: &str) {
    let accel_ptr = emu.regs().rcx;
    let num_entries = emu.regs().rdx as u32;
    log_red!(
        emu,
        "user32!{api} accel:0x{:x} count:{}",
        accel_ptr,
        num_entries
    );
    let hndl = helper::handler_create(&format!("user32://{api}"));
    emu.regs_mut().rax = hndl;
}

pub fn CreateAcceleratorTableW(emu: &mut emu::Emu) {
    create_accelerator_table(emu, "CreateAcceleratorTableW");
}
