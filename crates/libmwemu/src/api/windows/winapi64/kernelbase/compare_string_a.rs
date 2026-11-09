use crate::emu;

pub fn CompareStringA(emu: &mut emu::Emu) {
    let _locale = emu.regs().rcx;
    let _flags = emu.regs().rdx;
    let str1_ptr = emu.regs().r8;
    let _len1 = emu.regs().r9 as i32;
    let str2_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let s1 = if str1_ptr != 0 {
        emu.maps.read_string(str1_ptr)
    } else {
        String::new()
    };
    let s2 = if str2_ptr != 0 {
        emu.maps.read_string(str2_ptr)
    } else {
        String::new()
    };
    let result = match s1.to_lowercase().cmp(&s2.to_lowercase()) {
        std::cmp::Ordering::Less => 1u64,
        std::cmp::Ordering::Equal => 2,
        std::cmp::Ordering::Greater => 3,
    };
    log_red!(
        emu,
        "kernelbase!CompareStringA '{}' vs '{}' => {}",
        s1,
        s2,
        result
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
}
