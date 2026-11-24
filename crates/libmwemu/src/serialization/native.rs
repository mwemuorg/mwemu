use crate::api::windows::export_index::ModuleExportIndex;
use crate::emu::loaders::guest_dll::GuestDll;
use crate::syscall::windows::syscall64::wnf::WnfSubscriptions;
use rand_chacha::ChaCha12Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::api::unix::{ApiState, MemoryFault};
use crate::api::windows::policy::ApiPolicy;
use crate::emu::object_handle::{HandleManagement, MemoryHandles, WindowsHandle};
use crate::emu::{Emu, RuntimeFunctionTable};
use crate::kernel::heap::KernelHeap;
use crate::maps::heap_allocation::{O1Heap, O1HeapSnapshot};
use crate::serialization::emu::SerializableEmu;
use crate::threading::context::{SignalStack, WindowsThreadState};
use crate::winapi::common::crt_environment::CrtEnvironment;
use crate::winapi::common::crypto_state::CryptoStore;
use crate::winapi::common::handles::ConsoleCursor;
use crate::winapi::common::ipc::IpcState;
use crate::winapi::common::process::Priorities;
use crate::winapi::common::winsock::sockets::SocketState;
use crate::winapi::environment_store::Environment;
use crate::winapi::libraries::kernel32::atoms::AtomTables;
use crate::winapi::libraries::ole32::runtime::Apartment;
use crate::winapi::profile_store::ProfileStore;
use crate::winapi::registry_store::Registry;
use crate::winapi::window_table::{TimerTable, WindowTable};

#[derive(Serialize, Deserialize)]
pub(crate) struct NativeSnapshotV3 {
    emulator: SerializableEmu,
    heaps: Vec<O1HeapSnapshot>,
    handles: MemoryHandles,
    guard_heap: Option<KernelHeap>,
}

impl From<&Emu> for NativeSnapshotV3 {
    fn from(emu: &Emu) -> Self {
        Self {
            emulator: SerializableEmu::from(emu),
            heaps: emu.heap_arenas.iter().map(|heap| heap.snapshot()).collect(),
            handles: emu.handle_management.memory_snapshot(),
            guard_heap: emu
                .kernel
                .as_ref()
                .filter(|kernel| kernel.ledger_only)
                .map(|kernel| kernel.heap.clone()),
        }
    }
}

impl From<NativeSnapshotV3> for Emu {
    fn from(snapshot: NativeSnapshotV3) -> Self {
        let mut emu: Emu = snapshot.emulator.into();
        emu.heap_arenas = snapshot
            .heaps
            .iter()
            .map(|heap| Box::new(O1Heap::from_snapshot(heap)))
            .collect();
        emu.handle_management = HandleManagement::from_memory_snapshot(snapshot.handles);
        if let Some(heap) = snapshot.guard_heap {
            emu.memory_guard_init();
            if let Some(kernel) = emu.kernel.as_mut() {
                kernel.heap = heap;
            }
        }
        emu
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NativeSnapshotV4 {
    state: NativeSnapshotV3,
    policy: ApiPolicy,
}

impl From<&Emu> for NativeSnapshotV4 {
    fn from(emu: &Emu) -> Self {
        Self {
            state: NativeSnapshotV3::from(emu),
            policy: emu.api_policy.clone(),
        }
    }
}

impl From<NativeSnapshotV4> for Emu {
    fn from(snapshot: NativeSnapshotV4) -> Self {
        let mut emu: Emu = snapshot.state.into();
        emu.api_policy = snapshot.policy;
        emu
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NativeSnapshotV5 {
    state: NativeSnapshotV4,
    unix_api: ApiState,
}

impl From<&Emu> for NativeSnapshotV5 {
    fn from(emu: &Emu) -> Self {
        Self {
            state: NativeSnapshotV4::from(emu),
            unix_api: emu.unix_api.clone(),
        }
    }
}

impl From<NativeSnapshotV5> for Emu {
    fn from(snapshot: NativeSnapshotV5) -> Self {
        let mut emu: Emu = snapshot.state.into();
        emu.unix_api = snapshot.unix_api;
        emu
    }
}

#[derive(Serialize, Deserialize)]
struct ThreadState {
    signal_stack: SignalStack,
    windows: WindowsThreadState,
    error_mode: u32,
    windows_start_address: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NativeSnapshotUnstable {
    state: NativeSnapshotV5,
    threads: Vec<ThreadState>,
    windows_handles: Vec<(usize, WindowsHandle)>,
    unix_memory_fault: Option<MemoryFault>,
    sockets: SocketState,
    oem_file_names: bool,
    app_user_model_id: Vec<u16>,
    registry: Registry,
    guest_windows: WindowTable,
    com_apartments: BTreeMap<u64, Apartment>,
    keyboard_state: BTreeMap<u64, Vec<u8>>,
    environment: Option<Environment>,
    crt_environments: [CrtEnvironment; 2],
    process_error_mode: u32,
    console_host_process: u64,
    raise_on_invalid_handle_close: bool,
    process_dep_flags: u32,
    ipc: IpcState,
    tls_slots: Vec<bool>,
    fls_slots: Vec<Option<u64>>,
    veh_registration_ids: Vec<u32>,
    last_veh_id: u32,
    priorities: Priorities,
    last_luid: u64,
    atom_tables: AtomTables,
    profiles: ProfileStore,
    rng: ChaCha12Rng,
    rng_seed: Option<u64>,
    crypto: CryptoStore,
    runtime_function_tables: Vec<RuntimeFunctionTable>,
    section_commitments: BTreeMap<u64, Vec<bool>>,
    locked_views: Vec<u64>,
    guest_dlls: BTreeMap<u64, GuestDll>,
    guest_exports: Vec<ModuleExportIndex>,
    user_timers: TimerTable,
    wnf_subscriptions: WnfSubscriptions,
    gdi_cookie: u64,
    console_cursor: ConsoleCursor,
}

impl From<&Emu> for NativeSnapshotUnstable {
    fn from(emu: &Emu) -> Self {
        Self {
            state: NativeSnapshotV5::from(emu),
            threads: emu
                .threads
                .iter()
                .map(|thread| ThreadState {
                    signal_stack: thread.signal_stack,
                    windows: thread.windows,
                    error_mode: thread.error_mode,
                    windows_start_address: thread.windows_start_address,
                })
                .collect(),
            windows_handles: emu.handle_management.windows_snapshot(),
            unix_memory_fault: emu.unix_api.memory_fault.clone(),
            sockets: emu.sockets.clone(),
            oem_file_names: emu.oem_file_names,
            app_user_model_id: emu.app_user_model_id.clone(),
            registry: emu.registry.clone(),
            guest_windows: emu.guest_windows.clone(),
            com_apartments: emu.com_apartments.clone(),
            keyboard_state: emu.keyboard_state.clone(),
            environment: emu.environment.clone(),
            crt_environments: emu.crt_environments.clone(),
            process_error_mode: emu.process_error_mode,
            console_host_process: emu.console_host_process,
            raise_on_invalid_handle_close: emu.raise_on_invalid_handle_close,
            process_dep_flags: emu.process_dep_flags,
            ipc: emu.ipc.clone(),
            tls_slots: emu.windows_tls_slots.clone(),
            fls_slots: emu.windows_fls_slots.clone(),
            veh_registration_ids: emu.veh_registration_ids.clone(),
            last_veh_id: emu.last_veh_id,
            priorities: emu.priorities.clone(),
            last_luid: emu.last_luid,
            atom_tables: emu.atom_tables.clone(),
            profiles: emu.profiles.clone(),
            rng: emu.rng.clone(),
            rng_seed: emu.rng_seed,
            crypto: emu.crypto.clone(),
            runtime_function_tables: emu.runtime_function_tables.clone(),
            guest_exports: emu
                .guest_dlls
                .keys()
                .filter_map(|base| emu.export_indexes.get_by_base(*base).cloned())
                .collect(),
            section_commitments: emu.maps.section_commitments.clone(),
            locked_views: emu
                .maps
                .allocations
                .values()
                .filter(|a| a.protection_locked)
                .map(|a| a.base)
                .collect(),
            guest_dlls: emu.guest_dlls.clone(),
            user_timers: emu.user_timers.clone(),
            wnf_subscriptions: emu.wnf_subscriptions.clone(),
            gdi_cookie: emu.gdi_cookie,
            console_cursor: emu.console_cursor,
        }
    }
}

impl From<NativeSnapshotUnstable> for Emu {
    fn from(snapshot: NativeSnapshotUnstable) -> Self {
        let mut emu: Emu = snapshot.state.into();
        for (thread, saved) in emu.threads.iter_mut().zip(snapshot.threads) {
            thread.signal_stack = saved.signal_stack;
            thread.windows = saved.windows;
            thread.error_mode = saved.error_mode;
            thread.windows_start_address = saved.windows_start_address;
        }
        emu.handle_management
            .restore_windows_snapshot(snapshot.windows_handles);
        emu.unix_api.memory_fault = snapshot.unix_memory_fault;
        emu.sockets = snapshot.sockets;
        emu.oem_file_names = snapshot.oem_file_names;
        emu.app_user_model_id = snapshot.app_user_model_id;
        emu.registry = snapshot.registry;
        emu.guest_windows = snapshot.guest_windows;
        emu.com_apartments = snapshot.com_apartments;
        emu.keyboard_state = snapshot.keyboard_state;
        emu.environment = snapshot.environment;
        emu.crt_environments = snapshot.crt_environments;
        emu.process_error_mode = snapshot.process_error_mode;
        emu.console_host_process = snapshot.console_host_process;
        emu.raise_on_invalid_handle_close = snapshot.raise_on_invalid_handle_close;
        emu.process_dep_flags = snapshot.process_dep_flags;
        emu.ipc = snapshot.ipc;
        emu.windows_tls_slots = snapshot.tls_slots;
        emu.windows_fls_slots = snapshot.fls_slots;
        emu.veh_registration_ids = snapshot.veh_registration_ids;
        emu.last_veh_id = snapshot.last_veh_id;
        emu.priorities = snapshot.priorities;
        emu.last_luid = snapshot.last_luid;
        emu.atom_tables = snapshot.atom_tables;
        emu.profiles = snapshot.profiles;
        emu.rng = snapshot.rng;
        emu.rng_seed = snapshot.rng_seed;
        emu.crypto = snapshot.crypto;
        emu.runtime_function_tables = snapshot.runtime_function_tables;
        emu.maps.section_commitments = snapshot.section_commitments;
        for base in snapshot.locked_views {
            if let Some(allocation) = emu.maps.allocations.get_mut(&base) {
                allocation.protection_locked = true;
            }
        }
        emu.guest_dlls = snapshot.guest_dlls;
        for module in snapshot.guest_exports {
            emu.export_indexes.register(module);
        }
        emu.user_timers = snapshot.user_timers;
        emu.wnf_subscriptions = snapshot.wnf_subscriptions;
        emu.gdi_cookie = snapshot.gdi_cookie;
        emu.console_cursor = snapshot.console_cursor;
        emu
    }
}
