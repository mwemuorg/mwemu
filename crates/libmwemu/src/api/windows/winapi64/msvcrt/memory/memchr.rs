use crate::emu;

pub fn memchr(emu: &mut emu::Emu) {
    let ptr = emu.regs().rcx;
    let val = emu.regs().rdx as u8;
    let n = emu.regs().r8 as usize;
    let mut found: u64 = 0;
    for i in 0..n {
        if emu.maps.read_byte(ptr + i as u64).unwrap_or(0) == val {
            found = ptr + i as u64;
            break;
        }
    }
    log_red!(
        emu,
        "msvcrt!memchr ptr:0x{:x} val:0x{:x} n:{} => 0x{:x}",
        ptr,
        val,
        n,
        found
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = found;
}
