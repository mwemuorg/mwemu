use crate::api::abi::windows::{CallingConvention::Stdcall, WindowsAbi::WinApi};
use crate::emu::Emu;
use crate::maps::buffers::BufferKind;
use crate::maps::mem64::Permission;
use crate::winapi::common::{buffers, memory};
use crate::winapi::policy::{self, api_try};

const TERMINATOR: usize = size_of::<u16>();

fn item_size(emu: &Emu, at: u64) -> Option<usize> {
    if at == 0 {
        return None;
    }
    let size = u16::from_le_bytes(memory::read_array(emu, at)?) as usize;
    (size == 0 || (size >= TERMINATOR && emu.maps.can_access_range(at, size, Permission::READ)))
        .then_some(size)
}

struct List {
    last: u64,
    end: u64,
}

fn list(emu: &Emu, start: u64) -> Option<List> {
    let mut last = start;
    let mut end = start;
    loop {
        let size = item_size(emu, end)?;
        if size == 0 {
            return Some(List { last, end });
        }
        last = end;
        end = end.checked_add(size as u64)?;
    }
}

fn payload(emu: &Emu, pidl: u64) -> Option<Vec<u8>> {
    if pidl == 0 {
        return Some(Vec::new());
    }
    let length = usize::try_from(list(emu, pidl)?.end - pidl).ok()?;
    if length as u64 > emu.maps.max_alloc_size {
        return None;
    }
    memory::read_guest_bytes(emu, pidl, length)
}

fn store(emu: &mut Emu, first: &[u8], second: &[u8]) -> Option<u64> {
    let size = first
        .len()
        .checked_add(second.len())?
        .checked_add(TERMINATOR)?;
    let address = buffers::allocate(emu, size as u64, 16, 0, BufferKind::Task)?;
    let second_at = address + first.len() as u64;
    if (first.is_empty() || emu.maps.write_bytes(address, first))
        && (second.is_empty() || emu.maps.write_bytes(second_at, second))
        && emu.maps.write_word(second_at + second.len() as u64, 0)
    {
        Some(address)
    } else {
        buffers::free(emu, address, BufferKind::Task);
        None
    }
}

pub fn ILGetSize(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let size =
        list(emu, pidl).and_then(|value| u32::try_from(value.end - pidl + TERMINATOR as u64).ok());
    log_red!(emu, "shell32!ILGetSize pidl={pidl:#x}");
    call.set_return(emu, u64::from(size.unwrap_or(0)));
    call.cleanup_args(emu, 1);
}

pub fn ILGetNext(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let next = item_size(emu, pidl)
        .filter(|&size| size != 0)
        .and_then(|size| pidl.checked_add(size as u64));
    log_red!(emu, "shell32!ILGetNext pidl={pidl:#x}");
    call.set_return(emu, next.unwrap_or(0));
    call.cleanup_args(emu, 1);
}

pub fn ILFindLastID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let last = list(emu, pidl).map_or(0, |list| list.last);
    log_red!(emu, "shell32!ILFindLastID pidl={pidl:#x}");
    call.set_return(emu, last);
    call.cleanup_args(emu, 1);
}

pub fn ILRemoveLastID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let removed = list(emu, pidl)
        .is_some_and(|list| list.last != list.end && emu.maps.write_word(list.last, 0));
    log_red!(emu, "shell32!ILRemoveLastID pidl={pidl:#x}");
    call.set_return(emu, removed.into());
    call.cleanup_args(emu, 1);
}

fn clone_list(emu: &mut Emu, api: &str, first_only: bool) {
    let call = WinApi(Stdcall);
    let pidl = api_try!(emu, call.arg(emu, 0));
    let data = if pidl == 0 {
        None
    } else if first_only {
        item_size(emu, pidl).and_then(|size| memory::read_guest_bytes(emu, pidl, size))
    } else {
        payload(emu, pidl)
    };
    let result = data.and_then(|bytes| store(emu, &bytes, &[]));
    log_red!(emu, "shell32!{api} pidl={pidl:#x}");
    call.set_return(emu, result.unwrap_or(0));
    call.cleanup_args(emu, 1);
}

pub fn ILClone(emu: &mut Emu) {
    clone_list(emu, "ILClone", false);
}

pub fn ILCloneFirst(emu: &mut Emu) {
    clone_list(emu, "ILCloneFirst", true);
}

pub fn ILCombine(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [parent, child] = api_try!(emu, call.args(emu));
    let result = (|| {
        if parent == 0 && child == 0 {
            return None;
        }
        let parent = payload(emu, parent)?;
        let child = payload(emu, child)?;
        store(emu, &parent, &child)
    })();
    log_red!(emu, "shell32!ILCombine parent={parent:#x} child={child:#x}");
    call.set_return(emu, result.unwrap_or(0));
    call.cleanup_args(emu, 2);
}

pub fn ILAppendID(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let [pidl, item, append] = api_try!(emu, call.args(emu));
    let result = (|| {
        if item == 0 {
            return Some(pidl);
        }
        let before = payload(emu, pidl)?;
        let size = item_size(emu, item)?;
        let added = memory::read_guest_bytes(emu, item, size)?;
        let result = if append as u32 != 0 {
            store(emu, &before, &added)
        } else {
            store(emu, &added, &before)
        }?;
        buffers::free(emu, pidl, BufferKind::Task);
        Some(result)
    })();
    log_red!(
        emu,
        "shell32!ILAppendID pidl={pidl:#x} item={item:#x} append={append:#x}",
        append = append as u32
    );
    call.set_return(emu, result.unwrap_or(0));
    call.cleanup_args(emu, 3);
}

pub fn SHAlloc(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let size = api_try!(emu, call.arg(emu, 0));
    let result = buffers::allocate(emu, size, 16, 0, BufferKind::Task);
    log_red!(emu, "shell32!SHAlloc size={size}");
    call.set_return(emu, result.unwrap_or(0));
    call.cleanup_args(emu, 1);
}

fn free(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "shell32!{api} address={address:#x}");
    buffers::free(emu, address, BufferKind::Task);
    call.cleanup_args(emu, 1);
}

pub fn ILFree(emu: &mut Emu) {
    free(emu, "ILFree");
}

pub fn SHFree(emu: &mut Emu) {
    free(emu, "SHFree");
}

fn compare(emu: &mut Emu, api: &str) {
    let call = WinApi(Stdcall);
    let [parent, child] = api_try!(emu, call.args(emu));
    let immediate = api == "ILIsParent" && api_try!(emu, call.arg_u32(emu, 2)) != 0;
    log_red!(
        emu,
        "shell32!{api} first_pidl={parent:#x} second_pidl={child:#x}"
    );
    let result = (|| {
        if parent == 0 || child == 0 {
            return Some(0);
        }
        let prefix = payload(emu, parent)?;
        let contents = payload(emu, child)?;
        if !contents.starts_with(&prefix) {
            // Different binary identifiers can refer to the same namespace item.
            return None;
        }
        let suffix = &contents[prefix.len()..];
        Some(match api {
            "ILIsEqual" => u64::from(suffix.is_empty()),
            "ILFindChild" => child + prefix.len() as u64,
            _ if immediate => u64::from(
                !suffix.is_empty()
                    && usize::from(u16::from_le_bytes([suffix[0], suffix[1]])) == suffix.len(),
            ),
            _ => 1,
        })
    })();
    if let Some(value) = result {
        call.set_return(emu, value);
        call.cleanup_args(emu, if api == "ILIsParent" { 3 } else { 2 });
    } else {
        policy::unsupported(emu, "shell32", api, emu.pc());
    }
}

pub fn ILIsEqual(emu: &mut Emu) {
    compare(emu, "ILIsEqual");
}

pub fn ILFindChild(emu: &mut Emu) {
    compare(emu, "ILFindChild");
}

pub fn ILIsParent(emu: &mut Emu) {
    compare(emu, "ILIsParent");
}
