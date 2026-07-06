use crate::emu;

/*
int CompareStringW(
    [in] LCID                              Locale,
    [in] DWORD                             dwCmpFlags,
    [in] _In_NLS_string_(cchCount1)PCNZWCH lpString1,
    [in] int                               cchCount1,
    [in] _In_NLS_string_(cchCount2)PCNZWCH lpString2,
    [in] int                               cchCount2
);
*/

pub fn CompareStringW(emu: &mut emu::Emu) {
    let locale = emu.regs().rcx;
    let dw_cmp_flags = emu.regs().rdx;
    let lp_string1 = emu.regs().r8;
    let cch_count1 = emu.regs().r9 as i32; // ✅ Fixed: no multiplication

    // Get stack parameters
    let lp_string2_addr = emu.regs().rsp + 0x20;
    let cch_count2_addr = emu.regs().rsp + 0x28;

    let lp_string2 = emu.maps.read_qword(lp_string2_addr).unwrap_or(0);
    let cch_count2 = emu.maps.read_dword(cch_count2_addr).unwrap_or(0) as i32; // ✅ Fixed: no multiplication

    log_red!(
        emu,
        "kernel32!CompareStringW locale: 0x{:x} flags: 0x{:x} str1: 0x{:x} len1: {} str2: 0x{:x} len2: {}",
        locale,
        dw_cmp_flags,
        lp_string1,
        cch_count1,
        lp_string2,
        cch_count2
    );

    // Read the strings - handle null/empty cases
    let s1 = if lp_string1 == 0 {
        String::new()
    } else if cch_count1 == -1 {
        emu.maps.read_wide_string(lp_string1)
    } else if cch_count1 == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string_n(lp_string1, cch_count1 as usize)
    };

    let s2 = if lp_string2 == 0 {
        String::new()
    } else if cch_count2 == -1 {
        emu.maps.read_wide_string(lp_string2)
    } else if cch_count2 == 0 {
        String::new()
    } else {
        emu.maps.read_wide_string_n(lp_string2, cch_count2 as usize)
    };

    // Perform comparison based on flags
    let result = if (dw_cmp_flags & 0x00000001) != 0 {
        // Case-insensitive comparison (NORM_IGNORECASE)
        let s1_lower = s1.to_lowercase();
        let s2_lower = s2.to_lowercase();
        match s1_lower.cmp(&s2_lower) {
            std::cmp::Ordering::Less => 1,    // CSTR_LESS_THAN
            std::cmp::Ordering::Equal => 2,   // CSTR_EQUAL
            std::cmp::Ordering::Greater => 3, // CSTR_GREATER_THAN
        }
    } else {
        // Case-sensitive comparison
        match s1.cmp(&s2) {
            std::cmp::Ordering::Less => 1,    // CSTR_LESS_THAN
            std::cmp::Ordering::Equal => 2,   // CSTR_EQUAL
            std::cmp::Ordering::Greater => 3, // CSTR_GREATER_THAN
        }
    };

    log::trace!("\t\t'{}' == '{}'  ={}", s1, s2, result);

    emu.regs_mut().rax = result as u64;
}
