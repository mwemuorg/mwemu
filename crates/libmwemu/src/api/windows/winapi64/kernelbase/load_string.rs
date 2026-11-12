use crate::emu;

pub fn LoadStringA(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(
        emu,
        "kernelbase!LoadStringA '{}' p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        s,
        p1,
        p2,
        p3
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

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

    if buffer_ptr != 0 && buffer_len > 0 {
        let fake = format!("String_{}", id);
        emu.maps.write_wide_string(buffer_ptr, &fake);
        emu.regs_mut().rax = fake.len() as u64;
    } else {
        emu.regs_mut().rax = 0;
    }
}
