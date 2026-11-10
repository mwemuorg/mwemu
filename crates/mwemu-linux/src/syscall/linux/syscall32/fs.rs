use crate::emu;
use crate::winapi::helper;

pub(super) fn dispatch(emu: &mut emu::Emu) -> bool {
    match emu.regs().get_eax() as u32 {
        3 => handle_syscall32_read(emu),
        4 => handle_syscall32_write(emu),
        5 => handle_syscall32_open(emu),
        6 => handle_syscall32_close(emu),
        8 => super::trace_syscall32(emu, "creat"),
        9 => super::trace_syscall32(emu, "link"),
        10 => super::trace_syscall32(emu, "unlink"),
        12 => handle_syscall32_chdir(emu),
        14 => super::trace_syscall32(emu, "mknod"),
        15 => handle_syscall32_chmod(emu),
        16 => super::trace_syscall32(emu, "lchown"),
        18 => super::trace_syscall32(emu, "oldstat"),
        _ => return false,
    }

    true
}

fn handle_syscall32_read(emu: &mut emu::Emu) {
    let fd = emu.regs().rbx;
    let buff = emu.regs().rcx;
    let sz = emu.regs().rdx;
    emu.regs_mut().rax = buff;
    log::trace!(
        "{}** {} syscall read() fd: {} buf: 0x{:x} sz: {} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buff,
        sz,
        emu.colors.nc
    );
}

fn handle_syscall32_write(emu: &mut emu::Emu) {
    let fd = emu.regs().rbx;
    let buff = emu.regs().rcx;
    let sz = emu.regs().rdx;
    emu.regs_mut().rax = sz;
    log::trace!(
        "{}** {} syscall write() fd: {} buf: 0x{:x} sz: {} {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        buff,
        sz,
        emu.colors.nc
    );
}

fn handle_syscall32_open(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().rbx);
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

fn handle_syscall32_close(emu: &mut emu::Emu) {
    let fd = emu.regs().rbx;
    super::trace_syscall32(emu, &format!("close() fd: {}", fd));
    helper::socket_close(fd);
}

fn handle_syscall32_chdir(emu: &mut emu::Emu) {
    let path = emu.maps.read_string(emu.regs().rbx);
    log::trace!(
        "{}** {} syscall chdir() path: {} {}",
        emu.colors.light_red,
        emu.pos,
        path,
        emu.colors.nc
    );
}

fn handle_syscall32_chmod(emu: &mut emu::Emu) {
    let file_path = emu.maps.read_string(emu.regs().rbx);
    let perm = emu.regs().rcx;
    log::trace!(
        "{}** {} syscall chmod() file: {} perm: {} {}",
        emu.colors.light_red,
        emu.pos,
        file_path,
        perm,
        emu.colors.nc
    );
}
