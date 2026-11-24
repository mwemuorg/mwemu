use crate::api::abi::unix::LinuxSyscall;
use crate::arch::Arch;
use crate::emu::Emu;
use crate::syscall::linux::constants::errors::{EFAULT, EINVAL, ESRCH};
use iced_x86::Register;
use serde::{Deserialize, Serialize};

const FIRST_TLS_ENTRY: u32 = 6;
const TLS_ENTRIES: usize = 3;
const SELECTOR_INDEX_SHIFT: u32 = 3;
const SELECTOR_LDT: u16 = 4;
const DESC_32BIT: u32 = 1;
const DESC_EXPAND_DOWN: u32 = 1 << 1;
const DESC_EXECUTABLE: u32 = 1 << 2;
const DESC_READ_ONLY: u32 = 1 << 3;
const DESC_PAGE_LIMIT: u32 = 1 << 4;
const DESC_NOT_PRESENT: u32 = 1 << 5;
const DESC_FLAGS: u32 = 0x7f;
const LIMIT_MASK: u32 = 0x000f_ffff;
const PAGE_SHIFT: u32 = 12;

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
struct Descriptor {
    base: u32,
    limit: u32,
    flags: u32,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub(crate) struct Tls32 {
    entries: [Option<Descriptor>; TLS_ENTRIES],
    fs: u16,
    gs: u16,
}

impl Tls32 {
    pub(crate) fn selector(&self, segment: Register) -> u16 {
        match segment {
            Register::FS => self.fs,
            Register::GS => self.gs,
            _ => unreachable!(),
        }
    }

    fn entry(&self, selector: u16) -> Option<&Descriptor> {
        let index = ((selector as u32) >> SELECTOR_INDEX_SHIFT).checked_sub(FIRST_TLS_ENTRY)?;
        self.entries.get(index as usize)?.as_ref()
    }

    pub(crate) fn select(&mut self, segment: Register, selector: u16) -> bool {
        if selector > 3 && (selector & SELECTOR_LDT != 0 || self.entry(selector).is_none()) {
            return false;
        }
        match segment {
            Register::FS => self.fs = selector,
            Register::GS => self.gs = selector,
            _ => return false,
        }
        true
    }

    pub(crate) fn address(
        &self,
        segment: Register,
        offset: u32,
        bytes: usize,
        write: bool,
    ) -> Option<u64> {
        let entry = self.entry(self.selector(segment))?;
        let end = offset.checked_add(bytes.saturating_sub(1) as u32)?;
        let limit = if entry.flags & DESC_PAGE_LIMIT != 0 {
            (entry.limit << PAGE_SHIFT) | ((1 << PAGE_SHIFT) - 1)
        } else {
            entry.limit
        };
        let in_range = if entry.flags & DESC_EXPAND_DOWN != 0 {
            offset > limit
        } else {
            end <= limit
        };
        (in_range && (!write || entry.flags & DESC_READ_ONLY == 0))
            .then_some(entry.base.wrapping_add(offset) as u64)
    }
}

pub(super) fn thread_area(emu: &mut Emu, get: bool) {
    let call = LinuxSyscall;
    let [address, ..] = call.args_for(emu, Arch::X86);
    let result = access_descriptor(emu, address, get);
    call.set_return(emu, result.unwrap_or_else(|errno| errno));
}

fn access_descriptor(emu: &mut Emu, address: u64, get: bool) -> Result<u64, u64> {
    let read = |offset| emu.maps.read_dword(address + offset).ok_or(EFAULT);
    let entry = read(0)?;
    if get {
        let index = entry.checked_sub(FIRST_TLS_ENTRY).ok_or(EINVAL)?;
        let descriptor = emu
            .current_thread()
            .linux_tls32
            .entries
            .get(index as usize)
            .ok_or(EINVAL)?
            .unwrap_or(Descriptor {
                base: 0,
                limit: 0,
                flags: DESC_READ_ONLY | DESC_NOT_PRESENT,
            });
        let mut bytes = [0u8; 16];
        for (output, value) in bytes.chunks_exact_mut(4).zip([
            entry,
            descriptor.base,
            descriptor.limit,
            descriptor.flags,
        ]) {
            output.copy_from_slice(&value.to_le_bytes());
        }
        return emu
            .maps
            .write_bytes(address, &bytes)
            .then_some(0)
            .ok_or(EFAULT);
    }
    let base = read(4)?;
    let limit = read(8)?;
    let flags = read(12)? & DESC_FLAGS;
    let clear =
        base == 0 && limit == 0 && (flags == 0 || flags == DESC_READ_ONLY | DESC_NOT_PRESENT);
    if !clear && (flags & DESC_32BIT == 0 || flags & (DESC_EXECUTABLE | DESC_NOT_PRESENT) != 0) {
        return Err(EINVAL);
    }
    let entries = &emu.current_thread().linux_tls32.entries;
    let index = if entry == u32::MAX {
        let index = entries.iter().position(Option::is_none).ok_or(ESRCH)?;
        if !emu
            .maps
            .write_dword(address, FIRST_TLS_ENTRY + index as u32)
        {
            return Err(EFAULT);
        }
        index
    } else {
        let index = entry.checked_sub(FIRST_TLS_ENTRY).ok_or(EINVAL)? as usize;
        if index >= TLS_ENTRIES {
            return Err(EINVAL);
        }
        index
    };
    emu.current_thread_mut().linux_tls32.entries[index] = (!clear).then_some(Descriptor {
        base,
        limit: limit & LIMIT_MASK,
        flags,
    });
    Ok(0)
}
