use crate::emu;

pub fn IsEqualGUID(emu: &mut emu::Emu) {
    let guid1 = emu.regs().rcx;
    let guid2 = emu.regs().rdx;
    let equal = if guid1 != 0 && guid2 != 0 {
        let mut eq = true;
        for i in 0..4u64 {
            let a = emu.maps.read_dword(guid1 + i * 4).unwrap_or(0);
            let b = emu.maps.read_dword(guid2 + i * 4).unwrap_or(0);
            if a != b {
                eq = false;
                break;
            }
        }
        eq
    } else {
        false
    };
    log_red!(
        emu,
        "ole32!IsEqualGUID 0x{:x} 0x{:x} => {}",
        guid1,
        guid2,
        equal
    );
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    emu.regs_mut().rax = equal as u64;
}
