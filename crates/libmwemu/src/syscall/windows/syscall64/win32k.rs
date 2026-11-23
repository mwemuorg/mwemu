use crate::api::abi::windows::WindowsAbi::WinSyscall;
use crate::api::windows::policy;
use crate::emu::Emu;
use crate::maps::mem64::Permission;
use crate::maps::virtual_memory::{Allocation, MemoryKind, PAGE_SIZE, PageState};
use crate::syscall::windows;
use crate::winapi::environment_store::Encoding;
use crate::winapi::libraries::{gdi32, user32};
use crate::winapi::policy::api_try;
use crate::windows::constants::errors::ERROR_INVALID_PARAMETER;
use crate::windows::constants::memory::PAGE_READONLY;
use crate::windows::structures::{Peb64, TEB64, UnicodeString64};

const GUI_SERVICE_TABLE: u64 = 1;
const SERVICE_TABLE_SHIFT: u32 = 12;
const THREADSTATE_GETTHREADINFO: u32 = 14;

// The x64 shared section holds one entry per 16-bit GDI object index and a metadata page.
const GDI_ENTRY_SIZE: u64 = 24;
const GDI_ENTRY_COUNT: u64 = 1 << u16::BITS;
const GDI_METADATA: u64 = GDI_ENTRY_SIZE * GDI_ENTRY_COUNT;
const GDI_TABLE_SIZE: u64 = GDI_METADATA + PAGE_SIZE;
const GDI_STOCK_OBJECTS_OFFSET: u64 = GDI_METADATA + 0xb0;
const GDI_ENTRY_UNIQUE_OFFSET: u64 = 12;
const GDI_ENTRY_TYPE_OFFSET: u64 = 14;
const GDI_BASE_TYPE_MASK: u8 = 0x1f;

pub(super) fn is_gui_service(number: u64) -> bool {
    number >> SERVICE_TABLE_SHIFT == GUI_SERVICE_TABLE
}

pub(super) fn gateway(emu: &mut Emu, number: u64) {
    // A new module index is registered when the guest loader maps or replaces win32u.
    let name = emu
        .export_indexes
        .get_by_name("win32u")
        .and_then(|module| {
            module
                .service_names
                .get_or_init(|| windows::index_module_services(emu, "win32u"))
                .get(&number)
        })
        .cloned()
        .unwrap_or_else(|| format!("syscall {number:#x}"));
    windows::dispatch(emu, "win32u", number, &name, |emu| match name.as_str() {
        "NtGdiInit2" => init_gdi(emu),
        "NtUserGetThreadState" => get_thread_state(emu),
        "NtUserRegisterWindowMessage" => register_window_message(emu),
        _ => policy::unsupported_with_abi(emu, "win32u", &name, emu.pc(), WinSyscall),
    });
}

fn init_gdi(emu: &mut Emu) {
    let call = WinSyscall;
    let Some(teb) = emu.windows_teb() else {
        return call.set_return(emu, 0);
    };
    let Some(peb) = emu
        .maps
        .read_qword(teb + TEB64::PROCESS_ENVIRONMENT_BLOCK_OFFSET)
    else {
        return call.set_return(emu, 0);
    };
    let field = peb.checked_add(Peb64::GDI_SHARED_HANDLE_TABLE_OFFSET);
    let Some(field) = field.filter(|&p| emu.maps.validate_write_range(p, size_of::<u64>() as u64))
    else {
        return call.set_return(emu, 0);
    };
    let base = if let Some(table) = emu.maps.get_map_by_name("gdi_shared_handles") {
        table.get_base()
    } else {
        let Some(base) = emu.maps.alloc(GDI_TABLE_SIZE) else {
            return call.set_return(emu, 0);
        };
        let stocks = gdi32::stock_objects(emu);
        let Ok(table) =
            emu.maps
                .create_map("gdi_shared_handles", base, GDI_TABLE_SIZE, Permission::READ)
        else {
            return call.set_return(emu, 0);
        };
        for (index, handle) in stocks {
            table.force_write_bytes(
                base + GDI_STOCK_OBJECTS_OFFSET + index * 8,
                &handle.to_le_bytes(),
            );
            let entry = base + (handle as u16) as u64 * GDI_ENTRY_SIZE;
            let unique = (handle >> u16::BITS) as u16;
            table.force_write_bytes(entry + GDI_ENTRY_UNIQUE_OFFSET, &unique.to_le_bytes());
            table.force_write_bytes(
                entry + GDI_ENTRY_TYPE_OFFSET,
                &[(unique as u8) & GDI_BASE_TYPE_MASK],
            );
        }
        let mut allocation = Allocation::new(
            base,
            GDI_TABLE_SIZE,
            PAGE_READONLY,
            MemoryKind::Mapped,
            PageState::committed(PAGE_READONLY, Permission::READ),
        );
        allocation.maximum_permission = Some(Permission::READ);
        emu.maps.register_allocation(allocation);
        base
    };
    emu.maps.write_qword(field, base);
    if emu.gdi_cookie == 0 {
        emu.gdi_cookie = emu.random_u64().max(1);
    }
    call.set_return(emu, emu.gdi_cookie);
}

fn get_thread_state(emu: &mut Emu) {
    let call = WinSyscall;
    let query = api_try!(emu, call.arg_u32(emu, 0));
    if query != THREADSTATE_GETTHREADINFO {
        return policy::unsupported_with_abi(emu, "win32u", "NtUserGetThreadState", emu.pc(), call);
    }
    let Some(teb) = emu.windows_teb() else {
        return call.set_return(emu, 0);
    };
    let Some(info) = teb.checked_add(TEB64::WIN32_THREAD_INFO_OFFSET) else {
        return call.set_return(emu, 0);
    };
    let Some(current) = emu.maps.read_qword(info) else {
        return call.set_return(emu, 0);
    };
    // Win32ThreadInfo is an opaque token. Guest code must not dereference it.
    let ready = current != 0 || emu.maps.write_qword(info, !teb);
    call.set_return(emu, ready as u64);
}

fn register_window_message(emu: &mut Emu) {
    let call = WinSyscall;
    let descriptor = api_try!(emu, call.arg(emu, 0));
    let text = if descriptor != 0
        && emu.maps.can_access_range(
            descriptor,
            UnicodeString64::size() as usize,
            Permission::READ,
        ) {
        let name = UnicodeString64::load(descriptor, &emu.maps);
        // Win32k validates the descriptor, then interns the NUL-terminated name.
        if name.length.is_multiple_of(2) && name.length <= name.maximum_length && name.buffer != 0 {
            Encoding::Wide.read(emu, name.buffer).ok()
        } else {
            None
        }
    } else {
        None
    };
    let result = match text {
        Some(name) => user32::registered_message(emu, &name),
        None => {
            emu.set_last_error(ERROR_INVALID_PARAMETER);
            0
        }
    };
    call.set_return(emu, result);
}
