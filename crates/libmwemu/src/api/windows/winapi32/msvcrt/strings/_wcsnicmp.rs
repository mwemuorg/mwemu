use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn _wcsnicmp(emu: &mut emu::Emu) {
    let s1_ptr = WinApiAbi::arg(emu, 0);
    let s2_ptr = WinApiAbi::arg(emu, 1);
    let n = WinApiAbi::arg(emu, 2) as usize;
    let s1 = emu.maps.read_wide_string(s1_ptr).to_ascii_lowercase();
    let s2 = emu.maps.read_wide_string(s2_ptr).to_ascii_lowercase();
    let result = s1.get(..n).unwrap_or(&s1).cmp(s2.get(..n).unwrap_or(&s2)) as i32;
    log_red!(
        emu,
        "msvcrt!_wcsnicmp '{}' '{}' {} => {}",
        s1,
        s2,
        n,
        result
    );
    emu.regs_mut().rax = result as u64;
}
