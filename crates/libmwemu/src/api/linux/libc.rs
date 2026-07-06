#[path = "../abi.rs"]
mod abi;

use crate::emu::Emu;
use abi::ApiAbi;

pub fn gateway(symbol: &str, emu: &mut Emu) {
    match symbol {
        "__libc_start_main" => api_libc_start_main(emu),
        "__cxa_finalize" => api_cxa_finalize(emu),
        "__cxa_atexit" => api_cxa_atexit(emu),
        "__gmon_start__" => api_gmon_start(emu),
        "printf" => api_printf(emu),
        "fprintf" => api_fprintf(emu),
        "puts" => api_puts(emu),
        "putchar" => api_putchar(emu),
        "exit" => api_exit(emu),
        "_exit" => api_exit(emu),
        "abort" => api_abort(emu),
        "malloc" => api_malloc(emu),
        "calloc" => api_calloc(emu),
        "realloc" => api_realloc(emu),
        "free" => api_free(emu),
        "write" => api_write(emu),
        "read" => api_read(emu),
        "open" => api_open(emu),
        "close" => api_close(emu),
        "strlen" => api_strlen(emu),
        "memcpy" => api_memcpy(emu),
        "memset" => api_memset(emu),
        "mmap" => api_mmap(emu),
        "munmap" => api_munmap(emu),
        _ => {
            log::warn!("linuxapi libc: unimplemented API {}", symbol);
            todo!("Linux libc API: {}", symbol);
        }
    }
}

fn api_libc_start_main(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let main_fn = abi.arg(emu, 0);
    let argc = abi.arg(emu, 1);
    let argv = abi.arg(emu, 2);
    let envp = argv.wrapping_add((argc + 1) * 8);

    log::info!(
        "{}** {} Linux API __libc_start_main(main=0x{:x}, argc={}, argv=0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        main_fn,
        argc,
        argv,
        emu.colors.nc
    );

    let call_result = if emu.cfg.arch.is_aarch64() {
        emu.aarch64_call64(main_fn, &[argc, argv, envp])
    } else {
        emu.linux_call64(main_fn, &[argc, argv, envp])
    };

    match call_result {
        Ok(status) => {
            // Model glibc/musl startup minimally: after main returns, set the
            // exit status in the first argument register and terminate.
            if emu.cfg.arch.is_aarch64() {
                emu.regs_aarch64_mut().x[0] = status;
            } else {
                emu.regs_mut().rdi = status;
            }
            api_exit(emu);
        }
        Err(err) => {
            log::warn!(
                "linuxapi libc: __libc_start_main failed to run main: {}",
                err
            );
            emu.stop();
        }
    }
}

fn api_printf(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let fmt_addr = abi.arg(emu, 0);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} Linux API printf(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        emu.colors.nc
    );
    abi.set_ret(emu, fmt.len() as u64);
}

fn api_fprintf(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let _stream = abi.arg(emu, 0);
    let fmt_addr = abi.arg(emu, 1);
    let fmt = emu.maps.read_string(fmt_addr);
    log::info!(
        "{}** {} Linux API fprintf(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        fmt,
        emu.colors.nc
    );
    abi.set_ret(emu, fmt.len() as u64);
}

fn api_puts(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let s_addr = abi.arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    log::info!(
        "{}** {} Linux API puts(\"{}\") {}",
        emu.colors.light_red,
        emu.pos,
        s,
        emu.colors.nc
    );
    abi.set_ret(emu, 0);
}

fn api_putchar(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let c = abi.arg(emu, 0) as u8 as char;
    log::info!(
        "{}** {} Linux API putchar('{}') {}",
        emu.colors.light_red,
        emu.pos,
        c,
        emu.colors.nc
    );
    abi.set_ret(emu, c as u64);
}

fn api_exit(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let status = abi.arg(emu, 0);
    log::info!(
        "{}** {} Linux API exit({}) {}",
        emu.colors.light_red,
        emu.pos,
        status,
        emu.colors.nc
    );
    emu.stop();
}

fn api_abort(emu: &mut Emu) {
    log::info!(
        "{}** {} Linux API abort() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    emu.stop();
}

fn api_cxa_finalize(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let dso_handle = abi.arg(emu, 0);
    log::info!(
        "{}** {} Linux API __cxa_finalize(0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        dso_handle,
        emu.colors.nc
    );
    abi.set_ret(emu, 0);
}

fn api_cxa_atexit(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let func = abi.arg(emu, 0);
    let arg = abi.arg(emu, 1);
    let dso_handle = abi.arg(emu, 2);
    log::info!(
        "{}** {} Linux API __cxa_atexit(func=0x{:x}, arg=0x{:x}, dso=0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        func,
        arg,
        dso_handle,
        emu.colors.nc
    );
    abi.set_ret(emu, 0);
}

fn api_gmon_start(emu: &mut Emu) {
    log::info!(
        "{}** {} Linux API __gmon_start__() {}",
        emu.colors.light_red,
        emu.pos,
        emu.colors.nc
    );
    let abi = ApiAbi::from_emu(emu);
    abi.set_ret(emu, 0);
}

fn api_malloc(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let size = abi.arg(emu, 0);
    log::info!(
        "{}** {} Linux API malloc({}) {}",
        emu.colors.light_red,
        emu.pos,
        size,
        emu.colors.nc
    );
    todo!("malloc({})", size);
}

fn api_calloc(emu: &mut Emu) {
    todo!("calloc");
}
fn api_realloc(emu: &mut Emu) {
    todo!("realloc");
}

fn api_free(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let ptr = abi.arg(emu, 0);
    log::info!(
        "{}** {} Linux API free(0x{:x}) {}",
        emu.colors.light_red,
        emu.pos,
        ptr,
        emu.colors.nc
    );
    // no-op for now
}

fn api_write(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let fd = abi.arg(emu, 0);
    let buf = abi.arg(emu, 1);
    let count = abi.arg(emu, 2);
    let s = emu.maps.read_string(buf);
    log::info!(
        "{}** {} Linux API write(fd={}, \"{}\", {}) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        s,
        count,
        emu.colors.nc
    );
    abi.set_ret(emu, count);
}

fn api_read(emu: &mut Emu) {
    todo!("read");
}
fn api_open(emu: &mut Emu) {
    todo!("open");
}

fn api_close(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let fd = abi.arg(emu, 0);
    log::info!(
        "{}** {} Linux API close(fd={}) {}",
        emu.colors.light_red,
        emu.pos,
        fd,
        emu.colors.nc
    );
    abi.set_ret(emu, 0);
}

fn api_strlen(emu: &mut Emu) {
    let abi = ApiAbi::from_emu(emu);
    let s_addr = abi.arg(emu, 0);
    let s = emu.maps.read_string(s_addr);
    abi.set_ret(emu, s.len() as u64);
}

fn api_memcpy(emu: &mut Emu) {
    todo!("memcpy");
}
fn api_memset(emu: &mut Emu) {
    todo!("memset");
}
fn api_mmap(emu: &mut Emu) {
    todo!("mmap");
}
fn api_munmap(emu: &mut Emu) {
    todo!("munmap");
}
