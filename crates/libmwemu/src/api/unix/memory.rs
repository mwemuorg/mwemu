use crate::api::abi::unix::UnixApi;
use crate::api::unix::{MemoryAccess, MemoryError, api_try};
use crate::emu::Emu;
use crate::maps::mem64::Permission;

const COPY_CHUNK_BYTES: usize = 4096;

pub(crate) fn range(
    emu: &Emu,
    address: u64,
    size: u64,
    access: MemoryAccess,
) -> Result<(), MemoryError> {
    let error = MemoryError {
        address,
        size,
        access,
    };
    let count = usize::try_from(size).map_err(|_| error)?;
    let permission = match access {
        MemoryAccess::Read => Permission::READ,
        MemoryAccess::Write => Permission::WRITE,
        MemoryAccess::ObjectBounds => return Err(error),
    };
    if size == 0 || emu.maps.can_access_range(address, count, permission) {
        Ok(())
    } else {
        Err(error)
    }
}

fn offset(address: u64, count: u64, access: MemoryAccess) -> Result<u64, MemoryError> {
    address.checked_add(count).ok_or(MemoryError {
        address,
        size: count,
        access,
    })
}

fn byte(emu: &Emu, address: u64) -> Result<u8, MemoryError> {
    emu.maps.read_byte(address).ok_or(MemoryError {
        address,
        size: 1,
        access: MemoryAccess::Read,
    })
}

pub(crate) fn byte_at(emu: &Emu, address: u64, index: u64) -> Result<u8, MemoryError> {
    byte(emu, offset(address, index, MemoryAccess::Read)?)
}

pub(crate) fn write(emu: &mut Emu, address: u64, bytes: &[u8]) -> Result<(), MemoryError> {
    let size = bytes.len() as u64;
    range(emu, address, size, MemoryAccess::Write)?;
    if bytes.is_empty() || emu.maps.write_bytes(address, bytes) {
        Ok(())
    } else {
        Err(MemoryError {
            address,
            size,
            access: MemoryAccess::Write,
        })
    }
}

fn put_byte(emu: &mut Emu, address: u64, value: u8) -> Result<(), MemoryError> {
    if emu.maps.write_byte(address, value) {
        Ok(())
    } else {
        Err(MemoryError {
            address,
            size: 1,
            access: MemoryAccess::Write,
        })
    }
}

pub(crate) fn length(emu: &Emu, address: u64, limit: u64) -> Result<u64, MemoryError> {
    let mut size = 0;
    while size < limit {
        if byte(emu, offset(address, size, MemoryAccess::Read)?)? == 0 {
            break;
        }
        size += 1;
    }
    Ok(size)
}

pub(crate) fn copy(
    emu: &mut Emu,
    destination: u64,
    source: u64,
    count: u64,
) -> Result<(), MemoryError> {
    range(emu, source, count, MemoryAccess::Read)?;
    range(emu, destination, count, MemoryAccess::Write)?;
    let backwards = destination > source && destination - source < count;
    let mut buffer = [0; COPY_CHUNK_BYTES];
    let mut done = 0;
    while done < count {
        let size = (count - done).min(COPY_CHUNK_BYTES as u64) as usize;
        let at = if backwards {
            count - done - size as u64
        } else {
            done
        };
        for (index, value) in buffer[..size].iter_mut().enumerate() {
            *value = byte(emu, source + at + index as u64)?;
        }
        if !emu.maps.write_bytes(destination + at, &buffer[..size]) {
            return Err(MemoryError {
                address: destination + at,
                size: size as u64,
                access: MemoryAccess::Write,
            });
        }
        done += size as u64;
    }
    Ok(())
}

pub(crate) fn fill(
    emu: &mut Emu,
    destination: u64,
    value: u8,
    count: u64,
) -> Result<(), MemoryError> {
    range(emu, destination, count, MemoryAccess::Write)?;
    if emu.maps.fill_range(destination, value, count as usize) {
        Ok(())
    } else {
        Err(MemoryError {
            address: destination,
            size: count,
            access: MemoryAccess::Write,
        })
    }
}

fn object_bounds(destination: u64, count: u64, capacity: u64) -> Result<(), MemoryError> {
    if count <= capacity {
        Ok(())
    } else {
        Err(MemoryError {
            address: destination,
            size: count,
            access: MemoryAccess::ObjectBounds,
        })
    }
}

pub(crate) fn dispatch(emu: &mut Emu, symbol: &str, darwin: bool) -> bool {
    let name = if darwin && (!symbol.starts_with("__") || symbol.starts_with("___")) {
        symbol.strip_prefix('_').unwrap_or(symbol)
    } else {
        symbol
    };
    let (name, checked) = match name
        .strip_prefix("__")
        .and_then(|name| name.strip_suffix("_chk"))
    {
        Some(name) => (name, true),
        None => (name, false),
    };
    if checked
        && !matches!(
            name,
            "memcpy"
                | "memmove"
                | "memset"
                | "strcpy"
                | "strncpy"
                | "strcat"
                | "strncat"
                | "strlcpy"
                | "strlcat"
        )
    {
        return false;
    }
    let handler: fn(&mut Emu, bool) = match name {
        "memcpy" | "memmove" => memcpy,
        "memset" => memset,
        "bzero" => bzero,
        "memcmp" => memcmp,
        "memchr" => memchr,
        "strlen" => strlen,
        "strnlen" => strnlen,
        "strcasecmp" => strcasecmp,
        "strncasecmp" => strncasecmp,
        "strcmp" => strcmp,
        "strncmp" => strncmp,
        "strcpy" => strcpy,
        "strncpy" => strncpy,
        "strcat" => strcat,
        "strncat" => strncat,
        "strlcpy" => strlcpy,
        "strlcat" => strlcat,
        "strchr" => strchr,
        "strrchr" => strrchr,
        "strstr" => strstr,
        _ => return false,
    };
    log::info!(
        "{}** {} {} API {} {}",
        emu.colors.light_red,
        emu.pos,
        if darwin { "macOS" } else { "Linux" },
        symbol,
        emu.colors.nc
    );
    handler(emu, checked);
    true
}

fn memcpy(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    if checked {
        let capacity = api_try!(emu, call.arg(emu, 3));
        api_try!(emu, object_bounds(destination, count, capacity));
    }
    api_try!(emu, copy(emu, destination, source, count));
    call.set_return(emu, destination);
}

fn memset(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg_u8(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    if checked {
        let capacity = api_try!(emu, call.arg(emu, 3));
        api_try!(emu, object_bounds(destination, count, capacity));
    }
    api_try!(emu, fill(emu, destination, value, count));
    call.set_return(emu, destination);
}

fn bzero(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let count = api_try!(emu, call.arg(emu, 1));
    api_try!(emu, fill(emu, destination, 0, count));
}

fn memcmp(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let difference = api_try!(emu, compare_bytes(emu, left, right, count));
    call.set_return(emu, difference as u64);
}

pub(crate) fn compare_bytes(
    emu: &Emu,
    left: u64,
    right: u64,
    count: u64,
) -> Result<i32, MemoryError> {
    range(emu, left, count, MemoryAccess::Read)?;
    range(emu, right, count, MemoryAccess::Read)?;
    let mut difference = 0;
    for index in 0..count {
        difference = i32::from(byte(emu, left + index)?) - i32::from(byte(emu, right + index)?);
        if difference != 0 {
            break;
        }
    }
    Ok(difference)
}

fn memchr(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg_u8(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    for index in 0..count {
        let address = api_try!(emu, offset(source, index, MemoryAccess::Read));
        if api_try!(emu, byte(emu, address)) == value {
            call.set_return(emu, address);
            return;
        }
    }
    call.set_return(emu, 0);
}

fn strlen(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let size = api_try!(emu, length(emu, source, u64::MAX));
    call.set_return(emu, size);
}

pub(crate) fn compare_strings(
    emu: &Emu,
    left: u64,
    right: u64,
    count: u64,
    fold_case: bool,
) -> Result<i32, MemoryError> {
    for index in 0..count {
        let a = byte(emu, offset(left, index, MemoryAccess::Read)?)?;
        let b = byte(emu, offset(right, index, MemoryAccess::Read)?)?;
        let (a, b) = if fold_case {
            (a.to_ascii_lowercase(), b.to_ascii_lowercase())
        } else {
            (a, b)
        };
        if a != b || a == 0 {
            return Ok(i32::from(a) - i32::from(b));
        }
    }
    Ok(0)
}

fn strcmp(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let result = api_try!(emu, compare_strings(emu, left, right, u64::MAX, false));
    call.set_return(emu, result as u64);
}

fn strncmp(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let result = api_try!(emu, compare_strings(emu, left, right, count, false));
    call.set_return(emu, result as u64);
}

pub(crate) fn terminated_copy(
    emu: &mut Emu,
    destination: u64,
    source: u64,
    length: u64,
) -> Result<(), MemoryError> {
    let end = offset(destination, length, MemoryAccess::Write)?;
    range(
        emu,
        destination,
        length.saturating_add(1),
        MemoryAccess::Write,
    )?;
    copy(emu, destination, source, length)?;
    put_byte(emu, end, 0)
}

fn strcpy(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let capacity = if checked {
        api_try!(emu, call.arg(emu, 2))
    } else {
        u64::MAX
    };
    let size = api_try!(emu, length(emu, source, capacity));
    api_try!(
        emu,
        object_bounds(destination, size.saturating_add(1), capacity)
    );
    api_try!(emu, terminated_copy(emu, destination, source, size));
    call.set_return(emu, destination);
}

fn strncpy(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    if checked {
        let capacity = api_try!(emu, call.arg(emu, 3));
        api_try!(emu, object_bounds(destination, count, capacity));
    }
    let size = api_try!(emu, length(emu, source, count));
    api_try!(emu, range(emu, destination, count, MemoryAccess::Write));
    api_try!(emu, copy(emu, destination, source, size));
    if size < count {
        api_try!(emu, fill(emu, destination + size, 0, count - size));
    }
    call.set_return(emu, destination);
}

fn append(
    emu: &mut Emu,
    destination: u64,
    source: u64,
    count: u64,
    capacity: u64,
) -> Result<(), MemoryError> {
    let existing = length(emu, destination, capacity)?;
    object_bounds(destination, existing.saturating_add(1), capacity)?;
    let size = length(emu, source, count.min(capacity - existing))?;
    object_bounds(
        destination,
        existing.saturating_add(size).saturating_add(1),
        capacity,
    )?;
    terminated_copy(
        emu,
        offset(destination, existing, MemoryAccess::Write)?,
        source,
        size,
    )
}

fn strcat(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let capacity = if checked {
        api_try!(emu, call.arg(emu, 2))
    } else {
        u64::MAX
    };
    api_try!(emu, append(emu, destination, source, u64::MAX, capacity));
    call.set_return(emu, destination);
}

fn strncat(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let capacity = if checked {
        api_try!(emu, call.arg(emu, 3))
    } else {
        u64::MAX
    };
    api_try!(emu, append(emu, destination, source, count, capacity));
    call.set_return(emu, destination);
}

fn strlcpy(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    if checked {
        let capacity = api_try!(emu, call.arg(emu, 3));
        api_try!(emu, object_bounds(destination, count, capacity));
    }
    let size = api_try!(emu, length(emu, source, u64::MAX));
    if count > 0 {
        api_try!(
            emu,
            terminated_copy(emu, destination, source, size.min(count - 1))
        );
    }
    call.set_return(emu, size);
}

fn strlcat(emu: &mut Emu, checked: bool) {
    let call = UnixApi;
    let destination = api_try!(emu, call.arg(emu, 0));
    let source = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    if checked {
        let capacity = api_try!(emu, call.arg(emu, 3));
        api_try!(emu, object_bounds(destination, count, capacity));
    }
    let existing = api_try!(emu, length(emu, destination, count));
    let size = api_try!(emu, length(emu, source, u64::MAX));
    if existing < count {
        api_try!(
            emu,
            terminated_copy(
                emu,
                destination + existing,
                source,
                size.min(count - existing - 1)
            )
        );
    }
    call.set_return(emu, existing.wrapping_add(size));
}

pub(crate) fn find_character(
    emu: &Emu,
    source: u64,
    value: u8,
    last: bool,
) -> Result<u64, MemoryError> {
    let mut found = 0;
    let mut address = source;
    loop {
        let current = byte(emu, address)?;
        if current == value {
            found = address;
            if !last {
                return Ok(found);
            }
        }
        if current == 0 {
            return Ok(found);
        }
        address = offset(address, 1, MemoryAccess::Read)?;
    }
}

fn strchr(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg_u8(emu, 1));
    let result = api_try!(emu, find_character(emu, source, value, false));
    call.set_return(emu, result);
}

fn strrchr(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let value = api_try!(emu, call.arg_u8(emu, 1));
    let result = api_try!(emu, find_character(emu, source, value, true));
    call.set_return(emu, result);
}

pub(crate) fn find_string(emu: &Emu, mut source: u64, needle: u64) -> Result<u64, MemoryError> {
    if byte(emu, needle)? == 0 {
        return Ok(source);
    }
    loop {
        let mut index = 0;
        loop {
            let b = byte(emu, offset(needle, index, MemoryAccess::Read)?)?;
            if b == 0 {
                return Ok(source);
            }
            let a = byte(emu, offset(source, index, MemoryAccess::Read)?)?;
            if a == 0 {
                return Ok(0);
            }
            if a != b {
                break;
            }
            index += 1;
        }
        source = offset(source, 1, MemoryAccess::Read)?;
    }
}

fn strstr(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let needle = api_try!(emu, call.arg(emu, 1));
    let result = api_try!(emu, find_string(emu, source, needle));
    call.set_return(emu, result);
}

fn strnlen(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let source = api_try!(emu, call.arg(emu, 0));
    let count = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, length(emu, source, count));
    call.set_return(emu, size);
}

fn strcasecmp(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let result = api_try!(emu, compare_strings(emu, left, right, u64::MAX, true));
    call.set_return(emu, result as u64);
}

fn strncasecmp(emu: &mut Emu, _checked: bool) {
    let call = UnixApi;
    let left = api_try!(emu, call.arg(emu, 0));
    let right = api_try!(emu, call.arg(emu, 1));
    let count = api_try!(emu, call.arg(emu, 2));
    let result = api_try!(emu, compare_strings(emu, left, right, count, true));
    call.set_return(emu, result as u64);
}
