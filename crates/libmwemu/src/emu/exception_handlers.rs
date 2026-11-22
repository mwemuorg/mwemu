use std::mem::offset_of;

use goblin::pe::data_directories::{DataDirectory, SIZEOF_DATA_DIRECTORY};
use goblin::pe::exception::RuntimeFunction;
use goblin::pe::header::{PE_POINTER_OFFSET, SIZEOF_COFF_HEADER, SIZEOF_PE_MAGIC};
use goblin::pe::optional_header::{
    SIZEOF_STANDARD_FIELDS_64, SIZEOF_WINDOWS_FIELDS_64, WindowsFields64,
};
use rs_header::pe::shared::IMAGE_DIRECTORY_ENTRY_EXCEPTION;
use serde::{Deserialize, Serialize};

use crate::console::Console;
use crate::emu::Emu;
use crate::exception::handlers::{self, HandlerKind};
use crate::exception::seh32;
use crate::exception::types::{ExceptionRecord, ExceptionType, MemoryAccess};
use crate::maps::mem64::Permission;
use crate::threading::context::ArchThreadState;
use crate::windows::constants::exceptions::*;

const RUNTIME_FUNCTION_SIZE: u64 = size_of::<RuntimeFunction>() as u64;
const END_ADDRESS_OFFSET: u64 = offset_of!(RuntimeFunction, end_address) as u64;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct RuntimeFunctionTable {
    table: u64,
    base: u64,
    count: u32,
    first_rva: u32,
    end_rva: u32,
    sorted: bool,
}

impl Emu {
    pub fn veh(&self) -> u64 {
        self.veh_list().first().copied().unwrap_or(0)
    }

    pub fn veh_list(&self) -> &[u64] {
        match &self.threads[0].arch {
            ArchThreadState::X86 { veh, .. } => veh,
            _ => &[],
        }
    }

    pub fn set_veh(&mut self, value: u64) {
        if let ArchThreadState::X86 { veh, .. } = &mut self.threads[0].arch {
            veh.clear();
            self.veh_registration_ids.clear();
            if value != 0 {
                veh.push(value);
                self.veh_registration_ids.push(0);
            }
        }
    }

    pub fn add_veh(&mut self, first: bool, handler: u64) {
        self.insert_veh(first, handler, 0);
    }

    fn insert_veh(&mut self, first: bool, handler: u64, id: u32) {
        if let ArchThreadState::X86 { veh, .. } = &mut self.threads[0].arch {
            let index = if first { 0 } else { veh.len() };
            self.veh_registration_ids.resize(veh.len(), 0);
            veh.insert(index, handler);
            self.veh_registration_ids.insert(index, id);
        }
    }

    pub fn remove_veh(&mut self, handler: u64) {
        while let Some(i) = self.veh_list().iter().position(|&h| h == handler) {
            self.remove_veh_at(i);
        }
    }

    fn remove_veh_at(&mut self, index: usize) {
        if let ArchThreadState::X86 { veh, .. } = &mut self.threads[0].arch {
            self.veh_registration_ids.resize(veh.len(), 0);
            veh.remove(index);
            self.veh_registration_ids.remove(index);
        }
    }

    pub(crate) fn register_veh(&mut self, first: bool, handler: u64) -> u64 {
        if handler == 0 {
            return 0;
        }
        let Some(id) = self.last_veh_id.checked_add(1) else {
            return 0;
        };
        self.last_veh_id = id;
        self.insert_veh(first, handler, id);
        u64::from(id)
    }

    pub(crate) fn unregister_veh(&mut self, handle: u64) -> bool {
        if handle == 0 {
            return false;
        }
        let index = self
            .veh_registration_ids
            .iter()
            .position(|&id| u64::from(id) == handle)
            .or_else(|| {
                self.veh_list()
                    .iter()
                    .zip(&self.veh_registration_ids)
                    .position(|(&callback, &id)| id == 0 && callback == handle)
            });
        if let Some(i) = index {
            self.remove_veh_at(i);
            true
        } else {
            false
        }
    }

    pub fn uef(&self) -> u64 {
        match &self.threads[0].arch {
            ArchThreadState::X86 { uef, .. } => *uef,
            _ => unreachable!("uef() called on aarch64 emu"),
        }
    }

    pub fn set_uef(&mut self, value: u64) {
        match &mut self.threads[0].arch {
            ArchThreadState::X86 { uef, .. } => *uef = value,
            _ => unreachable!("set_uef() called on aarch64 emu"),
        }
    }

    pub fn eh_ctx(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx,
            _ => unreachable!("eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn set_eh_ctx(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { eh_ctx, .. } => *eh_ctx = value,
            _ => unreachable!("set_eh_ctx() called on aarch64 emu"),
        }
    }

    pub fn seh(&self) -> u64 {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh,
            _ => unreachable!("seh() called on aarch64 emu"),
        }
    }

    pub fn set_seh(&mut self, value: u64) {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { seh, .. } => *seh = value,
            _ => unreachable!("set_seh() called on aarch64 emu"),
        }
    }

    /// Resolve a guest PC to its half-open range in a static or registered function table.
    pub fn resolve_function_range(&self, addr: u64) -> Option<(u64, u64)> {
        let (base, entry) = self.runtime_function_entry(addr)?;
        let begin = self.maps.read_dword(entry)?;
        let end = self
            .maps
            .read_dword(entry.checked_add(END_ADDRESS_OFFSET)?)?;
        Some((
            base.checked_add(u64::from(begin))?,
            base.checked_add(u64::from(end))?,
        ))
    }

    pub(crate) fn register_runtime_functions(&mut self, table: u64, count: u32, base: u64) -> bool {
        let Ok(size) = usize::try_from(u64::from(count) * RUNTIME_FUNCTION_SIZE) else {
            return false;
        };
        if table == 0 || !self.maps.can_access_range(table, size, Permission::READ) {
            return false;
        }
        let mut registration = RuntimeFunctionTable {
            table,
            base,
            count,
            first_rva: u32::MAX,
            end_rva: 0,
            sorted: true,
        };
        for index in 0..u64::from(count) {
            let entry = table + index * RUNTIME_FUNCTION_SIZE;
            let Some(begin) = self.maps.read_dword(entry) else {
                return false;
            };
            let Some(end) = self.maps.read_dword(entry + END_ADDRESS_OFFSET) else {
                return false;
            };
            registration.sorted &= begin >= registration.end_rva;
            registration.first_rva = registration.first_rva.min(begin);
            registration.end_rva = registration.end_rva.max(end);
        }
        self.runtime_function_tables.push(registration);
        true
    }

    pub(crate) fn unregister_runtime_functions(&mut self, table: u64) -> bool {
        let Some(index) = self
            .runtime_function_tables
            .iter()
            .position(|item| item.table == table)
        else {
            return false;
        };
        self.runtime_function_tables.remove(index);
        true
    }

    pub(crate) fn runtime_function_entry(&self, addr: u64) -> Option<(u64, u64)> {
        if let Some((base, table, count)) = self.image_runtime_function_table(addr) {
            let rva = u32::try_from(addr.checked_sub(base)?).ok()?;
            return self
                .find_runtime_entry(table, count, rva, true)
                .map(|entry| (base, entry));
        }
        self.runtime_function_tables.iter().find_map(|item| {
            let rva = u32::try_from(addr.checked_sub(item.base)?).ok()?;
            if !(item.first_rva..item.end_rva).contains(&rva) {
                return None;
            }
            self.find_runtime_entry(item.table, item.count, rva, item.sorted)
                .map(|entry| (item.base, entry))
        })
    }

    fn image_runtime_function_table(&self, addr: u64) -> Option<(u64, u64, u32)> {
        let (base, image_size) = self.maps.find_pe_image_info(addr)?;
        let image_end = base.checked_add(image_size)?;
        let image_address = |rva: u64, size: u64| {
            let start = base.checked_add(rva)?;
            (start.checked_add(size)? <= image_end).then_some(start)
        };
        let read_dword = |rva| {
            self.maps
                .read_dword(image_address(rva, size_of::<u32>() as u64)?)
        };

        let nt_header = u64::from(read_dword(u64::from(PE_POINTER_OFFSET))?);
        let windows_fields =
            nt_header + (SIZEOF_PE_MAGIC + SIZEOF_COFF_HEADER + SIZEOF_STANDARD_FIELDS_64) as u64;
        let directory_count = read_dword(
            windows_fields + offset_of!(WindowsFields64, number_of_rva_and_sizes) as u64,
        )?;
        if directory_count as usize <= IMAGE_DIRECTORY_ENTRY_EXCEPTION {
            return None;
        }

        let directory = windows_fields
            + (SIZEOF_WINDOWS_FIELDS_64 + IMAGE_DIRECTORY_ENTRY_EXCEPTION * SIZEOF_DATA_DIRECTORY)
                as u64;
        let table_rva = read_dword(directory)?;
        let table_size = u64::from(read_dword(
            directory + offset_of!(DataDirectory, size) as u64,
        )?);
        if table_rva == 0 || table_size < RUNTIME_FUNCTION_SIZE {
            return None;
        }
        let table = image_address(u64::from(table_rva), table_size)?;
        Some((base, table, (table_size / RUNTIME_FUNCTION_SIZE) as u32))
    }

    fn find_runtime_entry(
        &self,
        table: u64,
        count: u32,
        addr_rva: u32,
        sorted: bool,
    ) -> Option<u64> {
        let mut lo = 0;
        let mut hi = u64::from(count);
        while lo < hi {
            let mid = if sorted { lo + (hi - lo) / 2 } else { lo };
            let entry_addr = table.checked_add(mid * RUNTIME_FUNCTION_SIZE)?;
            let begin = self.maps.read_dword(entry_addr)?;
            let end = self
                .maps
                .read_dword(entry_addr.checked_add(END_ADDRESS_OFFSET)?)?;
            if sorted && addr_rva < begin {
                hi = mid;
            } else if !(begin..end).contains(&addr_rva) {
                lo = mid + 1;
            } else {
                return Some(entry_addr);
            }
        }
        None
    }

    /// Trigger an exception while preserving the diagnostic hook interface.
    pub fn exception(&mut self, ex_type: ExceptionType) {
        let record = ExceptionRecord::new(ex_type, self.pc());
        self.dispatch_exception(ex_type, record);
    }

    pub(crate) fn memory_exception(
        &mut self,
        ex_type: ExceptionType,
        address: u64,
        size: usize,
        access: MemoryAccess,
    ) {
        let permission = match access {
            MemoryAccess::Read => Permission::READ,
            MemoryAccess::Write => Permission::WRITE,
            MemoryAccess::Execute => Permission::EXECUTE,
        };
        let fault_address = self
            .maps
            .first_inaccessible_address(address, size, permission)
            .unwrap_or(address);
        self.access_violation(ex_type, fault_address, access);
    }

    pub(crate) fn access_violation(
        &mut self,
        ex_type: ExceptionType,
        fault_address: u64,
        access: MemoryAccess,
    ) {
        let record = ExceptionRecord::access_violation(self.pc(), fault_address, access);
        self.dispatch_exception(ex_type, record);
    }

    fn dispatch_exception(&mut self, ex_type: ExceptionType, mut record: ExceptionRecord) {
        self.fault_count += 1;
        if let Some(mut hook) = self.hooks.hook_on_exception.take() {
            let handle = hook(self, self.pc(), ex_type);
            self.hooks.hook_on_exception = Some(hook);
            if !handle {
                log::trace!("cancelled exception handling from hook");
                return;
            }
        }
        if self.cfg.arch.is_aarch64() {
            log::error!(
                "unhandled aarch64 exception at 0x{:x}: {:?}",
                self.pc(),
                ex_type
            );
            self.stop();
            return;
        }
        record.nested = handlers::active_record(self).unwrap_or(0);
        self.dispatch_vectored_exception(&record, 0);
    }

    pub(crate) fn dispatch_vectored_exception(&mut self, record: &ExceptionRecord, index: usize) {
        if let Some(&address) = self.veh_list().get(index) {
            self.call_exception_handler(record, address, HandlerKind::Veh, index);
        } else {
            self.dispatch_structured_exception(record);
        }
    }

    pub(crate) fn dispatch_structured_exception(&mut self, record: &ExceptionRecord) {
        if self.cfg.is_x64() {
            handlers::search_x64(self, record);
            return;
        }
        let Some(registration) = seh32::guest_registration(self, self.seh() as u32) else {
            self.stop();
            return;
        };
        if registration == 0 || registration == EXCEPTION_CHAIN_END {
            self.dispatch_unhandled_exception(record);
            return;
        }
        let Some(address) = self
            .maps
            .read_dword(registration as u64 + size_of::<u32>() as u64)
        else {
            log::error!("unreadable x86 SEH registration at 0x{:x}", self.seh());
            self.stop();
            return;
        };
        if self.maps.read_dword(registration as u64).is_none() {
            log::error!("invalid x86 SEH registration at 0x{:x}", self.seh());
            self.stop();
            return;
        }
        if !self.running_script {
            let console = Console::new();
            console.print("jump the exception pointer (y/n)?");
            if console.cmd() != "y" {
                return;
            }
        }
        self.call_exception_handler(record, address as u64, HandlerKind::Seh, 0);
    }

    pub(crate) fn dispatch_unhandled_exception(&mut self, record: &ExceptionRecord) {
        if self.kernel_driver_active() {
            self.kernel_abort(format!(
                "unhandled exception 0x{:x} at 0x{:x}",
                record.code, record.address
            ));
            return;
        }
        if self.uef() != 0 {
            self.call_exception_handler(record, self.uef(), HandlerKind::Uef, 0);
        } else {
            log::error!(
                "unhandled exception 0x{:x} at 0x{:x}",
                record.code,
                record.address
            );
            self.stop();
        }
    }

    fn call_exception_handler(
        &mut self,
        record: &ExceptionRecord,
        address: u64,
        kind: HandlerKind,
        veh_index: usize,
    ) {
        if self
            .maps
            .get_mem_by_addr(address)
            .is_none_or(|mem| mem.read_code(address, 1).is_empty())
        {
            log::error!("inaccessible exception handler at 0x{:x}", address);
            self.stop();
            return;
        }
        handlers::enter_record(self, record, kind, veh_index);
        self.flags_mut().f_tf = false;
        if !self.process_terminated {
            self.regs_mut().rip = address;
            self.force_reload = true;
        }
    }
}
