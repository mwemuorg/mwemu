use crate::arch::Arch;
use crate::emu;
use crate::syscall::linux::constants::syscalls::*;
use crate::syscall::linux::io;
use crate::syscall::linux::unsupported;
use crate::winapi::helper;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() as u32 {
        3 => io::read(emu, Arch::X86),
        4 => io::write(emu, Arch::X86),
        5 if !emu.cfg.is_x64() => handle_syscall32_open(emu),
        6 => io::close(emu, Arch::X86),
        8 => unsupported(emu, "creat"),
        9 => unsupported(emu, "link"),
        10 => unsupported(emu, "unlink"),
        12 => unsupported(emu, "chdir"),
        14 => unsupported(emu, "mknod"),
        15 => unsupported(emu, "chmod"),
        16 => unsupported(emu, "lchown"),
        18 => unsupported(emu, "oldstat"),
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
