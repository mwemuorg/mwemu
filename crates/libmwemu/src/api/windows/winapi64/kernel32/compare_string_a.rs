use crate::emu;

pub fn CompareStringA(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let s1_ptr = emu.regs().r8;
    let _len1 = emu.regs().r9 as i32;
    let s2_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _len2 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0) as i32;
    let str1 = emu.maps.read_string(s1_ptr);
    let str2 = if s2_ptr != 0 {
        emu.maps.read_string(s2_ptr)
    } else {
        String::new()
    };
    let ignore_case = flags & 1 != 0;
    let result = if ignore_case {
        str1.to_lowercase().cmp(&str2.to_lowercase())
    } else {
        str1.cmp(&str2)
    };
    let ret = match result {
        std::cmp::Ordering::Less => 1u64,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    };
    log_red!(
        emu,
        "kernel32!CompareStringA locale:{} flags:0x{:x} '{}' '{}' => {}",
        locale,
        flags,
        str1,
        str2,
        ret
    );
    emu.regs_mut().rax = ret;
}
