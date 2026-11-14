use super::EINVAL;
use super::read_into;
use super::set_errno;
use crate::emu;

pub(super) fn fread(emu: &mut emu::Emu) {
    let buf = emu.regs().rcx;
    let size = emu.regs().rdx;
    let count = emu.regs().r8;
    let file = emu.regs().r9;
    let total = size.saturating_mul(count) as usize;
    let items = if total == 0 {
        Some(0)
    } else {
        read_into(emu, file, buf, total).map(|n| n as u64 / size)
    };
    log_red!(
        emu,
        "wincrt!fread buf:0x{:x} size:{} count:{} fp:0x{:x} = {:?}",
        buf,
        size,
        count,
        file,
        items
    );
    if items.is_none() {
        set_errno(emu, EINVAL);
    }
    emu.regs_mut().rax = items.unwrap_or(0);
}
