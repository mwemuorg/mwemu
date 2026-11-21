use crate::api::abi::windows::{CallingConvention::Cdecl, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::exception::types::{ExceptionType, MemoryAccess};
use crate::winapi::common::crt::{Runtime, record_errno};
use crate::winapi::common::memory::read_array;
use crate::winapi::policy::api_try;
use crate::windows::constants::crt::{_TRUNCATE, EINVAL, ERANGE, STRUNCATE};

#[derive(Clone, Copy)]
enum Operation {
    Copy,
    Append,
    CopyCounted,
}

type Fault = (u64, MemoryAccess);

pub(super) fn read(emu: &Emu, address: u64, width: usize) -> Result<u16, Fault> {
    let unit = if width == 1 {
        emu.maps.read_byte(address).map(u16::from)
    } else {
        read_array(emu, address).map(u16::from_le_bytes)
    };
    unit.ok_or((address, MemoryAccess::Read))
}

pub(super) fn write(emu: &mut Emu, address: u64, value: u16, width: usize) -> Result<(), Fault> {
    if emu.maps.write_bytes(address, &value.to_le_bytes()[..width]) {
        Ok(())
    } else {
        Err((address, MemoryAccess::Write))
    }
}

struct Destination {
    start: u64,
    capacity: u64,
    width: usize,
}

impl Destination {
    fn address(&self, index: u64) -> Result<u64, Fault> {
        index
            .checked_mul(self.width as u64)
            .and_then(|n| self.start.checked_add(n))
            .ok_or((self.start, MemoryAccess::Write))
    }

    fn clear_with(&self, emu: &mut Emu, error: i32) -> Result<i32, Fault> {
        write(emu, self.start, 0, self.width)?;
        Ok(error)
    }

    fn copy(
        &self,
        emu: &mut Emu,
        mut source: u64,
        operation: Operation,
        count: Option<u64>,
    ) -> Result<i32, Fault> {
        if self.start == 0 && self.capacity == 0 && count == Some(0) {
            return Ok(0);
        }
        if self.start == 0 || self.capacity == 0 {
            return Ok(EINVAL);
        }
        if count == Some(0) {
            return self.clear_with(emu, 0);
        }
        if source == 0 {
            return self.clear_with(emu, EINVAL);
        }
        let mut index = 0;
        if matches!(operation, Operation::Append) {
            while index < self.capacity && read(emu, self.address(index)?, self.width)? != 0 {
                index += 1;
            }
            if index == self.capacity {
                return self.clear_with(emu, EINVAL);
            }
        }
        let truncate = count == Some(_TRUNCATE);
        let mut copied = 0;
        while index < self.capacity {
            let out = self.address(index)?;
            let unit = if Some(copied) == count {
                0
            } else {
                read(emu, source, self.width)?
            };
            if truncate && index == self.capacity - 1 && unit != 0 {
                write(emu, out, 0, self.width)?;
                return Ok(STRUNCATE);
            }
            write(emu, out, unit, self.width)?;
            if unit == 0 {
                return Ok(0);
            }
            index += 1;
            copied += 1;
            source = source
                .checked_add(self.width as u64)
                .ok_or((source, MemoryAccess::Read))?;
        }
        self.clear_with(emu, ERANGE)
    }
}

pub(crate) fn dispatch_bounded(api: &str, emu: &mut Emu, runtime: Runtime) -> bool {
    let operation = match api {
        "strcpy_s" | "wcscpy_s" => Operation::Copy,
        "strncpy_s" | "wcsncpy_s" => Operation::CopyCounted,
        "strcat_s" | "wcscat_s" => Operation::Append,
        _ => return false,
    };
    let width = if api.starts_with('w') {
        size_of::<u16>()
    } else {
        1
    };
    log_red!(emu, "{}!{api}", runtime.library());
    bounded(emu, runtime, operation, width);
    true
}

fn bounded(emu: &mut Emu, runtime: Runtime, operation: Operation, width: usize) {
    let call = WinApi(Cdecl);
    let dest = Destination {
        start: api_try!(emu, call.arg(emu, 0)),
        capacity: api_try!(emu, call.arg(emu, 1)),
        width,
    };
    let source = api_try!(emu, call.arg(emu, 2));
    let count = if matches!(operation, Operation::CopyCounted) {
        let n = api_try!(emu, call.arg(emu, 3));
        Some(if !emu.cfg.arch.is_64bits() && n == u32::MAX as u64 {
            _TRUNCATE
        } else {
            n
        })
    } else {
        None
    };
    match dest.copy(emu, source, operation, count) {
        Ok(error) => {
            if matches!(error, EINVAL | ERANGE) && !record_errno(emu, runtime, error) {
                return;
            }
            call.set_return(emu, error as u64);
        }
        Err((address, access)) => {
            emu.memory_exception(
                ExceptionType::BadAddressDereferencing,
                address,
                width,
                access,
            );
        }
    }
}
