use crate::emu;
use crate::winapi::helper;

pub fn GetSecurityDescriptorLength(emu: &mut emu::Emu) {
    let sd_ptr = emu.regs().rcx;
    // Minimum SD size: Rev(1)+Sbz1(1)+Control(2)+Owner(8)+Group(8)+Sacl(8)+Dacl(8) = 36
    let mut length = 36u64;
    if sd_ptr != 0 {
        let control = emu.maps.read_word(sd_ptr + 2).unwrap_or(0);
        if (control & 0x0004) != 0 {
            // SE_DACL_PRESENT
            let dacl = emu.maps.read_qword(sd_ptr + 28).unwrap_or(0);
            if dacl != 0 {
                let dacl_size = emu.maps.read_word(dacl + 2).unwrap_or(8) as u64;
                length += dacl_size;
            }
        }
        if (control & 0x0010) != 0 {
            // SE_SACL_PRESENT
            let sacl = emu.maps.read_qword(sd_ptr + 20).unwrap_or(0);
            if sacl != 0 {
                let sacl_size = emu.maps.read_word(sacl + 2).unwrap_or(8) as u64;
                length += sacl_size;
            }
        }
    }
    log_red!(
        emu,
        "kernelbase!GetSecurityDescriptorLength sd:0x{:x} => {}",
        sd_ptr,
        length
    );
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = length;
}
