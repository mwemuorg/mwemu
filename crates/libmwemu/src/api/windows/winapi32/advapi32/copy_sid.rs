use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CopySid(emu: &mut emu::Emu) {
    let dest_len = WinApiAbi::arg(emu, 0);
    let dest_sid = WinApiAbi::arg(emu, 1);
    let src_sid = WinApiAbi::arg(emu, 2);
    if src_sid != 0 && dest_sid != 0 {
        let sub_count = emu.maps.read_byte(src_sid + 1).unwrap_or(1) as u64;
        let sid_len = 8 + sub_count * 4;
        let copy_len = std::cmp::min(sid_len, dest_len);
        for i in 0..copy_len {
            let b = emu.maps.read_byte(src_sid + i).unwrap_or(0);
            emu.maps.write_byte(dest_sid + i, b);
        }
    }
    log_red!(
        emu,
        "advapi32!CopySid dst:0x{:x} src:0x{:x} len:{}",
        dest_sid,
        src_sid,
        dest_len
    );
    for _ in 0..3 {
        emu.stack_pop32(false);
    }
    emu.regs_mut().rax = 1;
}
