use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().rax {
        constants::NR64_SOCKET => handle_syscall64_socket(emu),
        _ => return false,
    }

    true
}

pub(super) fn handle_syscall64_socket(emu: &mut emu::Emu) {
    let sock = helper::socket_create();
    let fam = emu.regs().rdi;
    let typ = emu.regs().rsi;
    let proto = emu.regs().rdx;

    log::trace!(
        "{}** {} syscall socketcall socket()  fam: {} type: {} proto: {} sock: {} {}",
        emu.colors.light_red,
        emu.pos,
        fam,
        typ,
        proto,
        sock,
        emu.colors.nc
    );
    emu.regs_mut().rax = sock;
}
