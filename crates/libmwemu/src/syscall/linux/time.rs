use crate::arch::Arch;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api::abi::unix::LinuxSyscall;
use crate::emu::Emu;
use crate::syscall::linux::constants::errors::{EFAULT, EINVAL, EOPNOTSUPP, EOVERFLOW};

const CLOCK_REALTIME: u32 = 0;
const CLOCK_MONOTONIC: u32 = 1;
const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
const CLOCK_MONOTONIC_RAW: u32 = 4;
const CLOCK_REALTIME_COARSE: u32 = 5;
const CLOCK_MONOTONIC_COARSE: u32 = 6;
const CLOCK_BOOTTIME: u32 = 7;

pub(super) enum Timespec {
    Time32,
    Time64,
}

fn realtime(emu: &Emu) -> Result<Duration, u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|t| t + emu.processes.clock_offset)
        .map_err(|_| EOVERFLOW)
}

fn stamp(emu: &Emu, id: u32) -> Result<Duration, u64> {
    match id {
        CLOCK_REALTIME | CLOCK_REALTIME_COARSE => realtime(emu),
        CLOCK_MONOTONIC | CLOCK_MONOTONIC_COARSE | CLOCK_MONOTONIC_RAW | CLOCK_BOOTTIME => {
            Ok(emu.now.elapsed() + emu.processes.clock_offset)
        }
        CLOCK_PROCESS_CPUTIME_ID | CLOCK_THREAD_CPUTIME_ID => Err(EOPNOTSUPP),
        _ => Err(EINVAL),
    }
}

fn write_pair(
    emu: &mut Emu,
    addr: u64,
    seconds: u64,
    fraction: u32,
    layout: Timespec,
) -> Result<(), u64> {
    let written = match layout {
        Timespec::Time32 => {
            let seconds = i32::try_from(seconds).map_err(|_| EOVERFLOW)?;
            let mut data = [0; 8];
            data[..4].copy_from_slice(&seconds.to_le_bytes());
            data[4..].copy_from_slice(&fraction.to_le_bytes());
            emu.maps.validate_write_range(addr, 8) && emu.maps.write_bytes(addr, &data)
        }
        Timespec::Time64 => {
            let mut data = [0; 16];
            data[..8].copy_from_slice(&seconds.to_le_bytes());
            data[8..].copy_from_slice(&u64::from(fraction).to_le_bytes());
            emu.maps.validate_write_range(addr, 16) && emu.maps.write_bytes(addr, &data)
        }
    };
    if written { Ok(()) } else { Err(EFAULT) }
}

pub(super) fn clock_gettime(emu: &mut Emu, arch: Arch, layout: Timespec) {
    let call = LinuxSyscall;
    let [id, address, ..] = call.args_for(emu, arch);
    let result = stamp(emu, id as u32)
        .and_then(|t| write_pair(emu, address, t.as_secs(), t.subsec_nanos(), layout));
    log::trace!(
        "{} syscall clock_gettime({id}, {address:#x}): {result:?}",
        emu.pos
    );
    call.set_return(emu, result.err().unwrap_or(0));
}

pub(super) fn gettimeofday(emu: &mut Emu, arch: Arch, layout: Timespec) {
    let call = LinuxSyscall;
    let [tv, tz, ..] = call.args_for(emu, arch);
    let value = realtime(emu).and_then(|t| {
        if tv != 0 {
            write_pair(emu, tv, t.as_secs(), t.subsec_micros(), layout)?;
        }
        if tz != 0 && !(emu.maps.validate_write_range(tz, 8) && emu.maps.write_qword(tz, 0)) {
            return Err(EFAULT);
        }
        Ok(())
    });
    log::trace!(
        "{} syscall gettimeofday({tv:#x}, {tz:#x}): {value:?}",
        emu.pos
    );
    call.set_return(emu, value.err().unwrap_or(0));
}
