use crate::emu;
use crate::winapi::helper;

pub fn EqualSid(emu: &mut emu::Emu) {
    let sid1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let sid2 = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let equal = if sid1 != 0 && sid2 != 0 {
        let count1 = emu.maps.read_byte(sid1 + 1).unwrap_or(0) as u64;
        let count2 = emu.maps.read_byte(sid2 + 1).unwrap_or(0) as u64;
        if count1 == count2 {
            let len = 8 + count1 * 4;
            let mut eq = true;
            for i in 0..len {
                let b1 = emu.maps.read_byte(sid1 + i).unwrap_or(0);
                let b2 = emu.maps.read_byte(sid2 + i).unwrap_or(0);
                if b1 != b2 {
                    eq = false;
                    break;
                }
            }
            eq
        } else {
            false
        }
    } else {
        false
    };
    log_red!(
        emu,
        "advapi32!EqualSid 0x{:x} 0x{:x} => {}",
        sid1,
        sid2,
        equal as u32
    );
    let h = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    for _ in 0..2 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = equal as u64;
}
