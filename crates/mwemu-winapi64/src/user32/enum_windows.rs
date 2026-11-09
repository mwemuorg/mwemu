use crate::emu;
use crate::windows::window_table;

pub(super) fn EnumWindows(emu: &mut emu::Emu) {
    let enum_func = emu.regs().rcx;
    let lparam = emu.regs().rdx;
    log_red!(
        emu,
        "user32!EnumWindows lpEnumFunc: 0x{:x} lParam: 0x{:x}",
        enum_func,
        lparam
    );

    // BOOL CALLBACK EnumWindowsProc(HWND, LPARAM); FALSE stops the walk and
    // makes EnumWindows return FALSE too.
    for hwnd in window_table::top_level_windows() {
        match emu.call64(enum_func, &[hwnd, lparam]) {
            Ok(keep_going) if keep_going as u32 != 0 => {}
            Ok(_) => {
                emu.regs_mut().rax = 0;
                return;
            }
            Err(e) => {
                log::warn!(
                    "EnumWindows callback 0x{:x} failed: {}",
                    enum_func,
                    e.message
                );
                emu.regs_mut().rax = 0;
                return;
            }
        }
    }
    emu.regs_mut().rax = 1;
}
