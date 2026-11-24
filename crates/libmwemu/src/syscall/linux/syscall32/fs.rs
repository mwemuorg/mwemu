use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::io;
use crate::syscall::linux::unsupported;
use crate::winapi::helper;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() {
        NR32_READ => io::read(emu, Arch::X86),
        NR32_WRITE => io::write(emu, Arch::X86),
        NR32_OPEN if !emu.cfg.is_x64() => handle_syscall32_open(emu),
        NR32_CLOSE => io::close(emu, Arch::X86),
        NR32_CREAT => unsupported(emu, "creat"),
        NR32_LINK => unsupported(emu, "link"),
        NR32_UNLINK => unsupported(emu, "unlink"),
        NR32_CHDIR => unsupported(emu, "chdir"),
        NR32_MKNOD => unsupported(emu, "mknod"),
        NR32_CHMOD => unsupported(emu, "chmod"),
        NR32_LCHOWN => unsupported(emu, "lchown"),
        NR32_OLDSTAT => unsupported(emu, "oldstat"),
        NR32_FCNTL | NR32_FCNTL64 => io::fcntl(emu, Arch::X86),
        NR32_WRITEV => io::writev(emu, Arch::X86),
        NR32_PIPE => io::pipe(emu, Arch::X86, false),
        NR32_PIPE2 => io::pipe(emu, Arch::X86, true),
        NR32_EVENTFD => io::eventfd(emu, Arch::X86, false),
        NR32_EVENTFD2 => io::eventfd(emu, Arch::X86, true),
        _ => return false,
    }

    true
}

fn handle_syscall32_open(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().get_ebx());
    let fd = helper::socket_create();
    emu.regs_mut().rax = fd;
    log::trace!(
        "{}** {} syscall open() file: {} fd:{} {}",
        emu.colors.light_red,
        emu.pos,
        file_path,
        fd,
        emu.colors.nc
    );
}
