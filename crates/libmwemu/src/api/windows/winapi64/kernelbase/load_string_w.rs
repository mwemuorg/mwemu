use crate::emu;
use crate::winapi::helper;

pub fn LoadStringW(emu: &mut emu::Emu) {
    let hinstance = emu.regs().rcx;
    let id = emu.regs().rdx as u32;
    let buffer_ptr = emu.regs().r8;
    let buffer_len = emu.regs().r9 as u32;
    log_red!(
        emu,
        "kernelbase!LoadStringW inst:0x{:x} id:{} buf:0x{:x} len:{}",
        hinstance,
        id,
        buffer_ptr,
        buffer_len
    );
    let handle = helper::handler_create("LoadStringW");
    let out_ptr = emu.regs().r9;
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    if buffer_ptr != 0 && buffer_len > 0 {
        let fake = format!("String_{}", id);
        emu.maps.write_wide_string(buffer_ptr, &fake);
        emu.regs_mut().rax = fake.len() as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
}
