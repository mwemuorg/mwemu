use crate::emu;

pub(super) fn InternetErrorDlg(emu: &mut emu::Emu) {
    let err = emu.regs().rcx;

    log_red!(emu, "wininet!InternetErrorDlg err: {}", err);

    emu.regs_mut().rax = 0;
}
