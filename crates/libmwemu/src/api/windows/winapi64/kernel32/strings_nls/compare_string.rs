use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu;
use crate::winapi::policy::api_try;

pub fn CompareStringA(emu: &mut emu::Emu) {
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg(emu, 0));
    let flags = api_try!(emu, call.arg(emu, 1));
    let s1_ptr = api_try!(emu, call.arg(emu, 2));
    let _len1 = api_try!(emu, call.arg_i32(emu, 3));
    let s2_ptr = api_try!(emu, call.arg(emu, 4));
    let _len2 = api_try!(emu, call.arg_i32(emu, 5));
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
    call.set_return(emu, ret);
}

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
    let call = WinApi(Stdcall);
    let locale = api_try!(emu, call.arg(emu, 0));
    let dw_cmp_flags = api_try!(emu, call.arg(emu, 1));
    let lp_string1 = api_try!(emu, call.arg(emu, 2));
    let cch_count1 = api_try!(emu, call.arg_i32(emu, 3)); // ✅ Fixed: no multiplication

    let lp_string2 = api_try!(emu, call.arg(emu, 4));
    let cch_count2 = api_try!(emu, call.arg_i32(emu, 5));

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

    emu.set_last_error(0);
    call.set_return(emu, result as u64);
}
