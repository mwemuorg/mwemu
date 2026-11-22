use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::api::unix::{ApiState, MemoryFault};
use crate::api::windows::policy::ApiPolicy;
use crate::emu::Emu;
use crate::emu::object_handle::{HandleManagement, MemoryHandles, WindowsHandle};
use crate::kernel::heap::KernelHeap;
use crate::maps::heap_allocation::{O1Heap, O1HeapSnapshot};
use crate::serialization::emu::SerializableEmu;
use crate::threading::context::{SignalStack, WindowsThreadState};
use crate::winapi::common::crt_environment::CrtEnvironment;
use crate::winapi::common::handles::ConsoleCursor;
use crate::winapi::common::ipc::IpcState;
use crate::winapi::common::process::Priorities;
use crate::winapi::common::winsock::sockets::SocketState;
use crate::winapi::environment_store::Environment;
use crate::winapi::libraries::kernel32::atoms::AtomTables;
use crate::winapi::libraries::ole32::runtime::Apartment;
use crate::winapi::profile_store::ProfileStore;
use crate::winapi::registry_store::Registry;
use crate::winapi::window_table::WindowTable;

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
pub(crate) struct NativeSnapshot {
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
    ipc: IpcState,
    tls_slots: Vec<bool>,
    fls_slots: Vec<bool>,
    veh_registration_ids: Vec<u32>,
    last_veh_id: u32,
    priorities: Priorities,
    last_luid: u64,
    atom_tables: AtomTables,
    profiles: ProfileStore,
    console_cursor: ConsoleCursor,
}

impl From<&Emu> for NativeSnapshot {
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
            ipc: emu.ipc.clone(),
            tls_slots: emu.windows_tls_slots.clone(),
            fls_slots: emu.windows_fls_slots.clone(),
            veh_registration_ids: emu.veh_registration_ids.clone(),
            last_veh_id: emu.last_veh_id,
            priorities: emu.priorities.clone(),
            last_luid: emu.last_luid,
            atom_tables: emu.atom_tables.clone(),
            profiles: emu.profiles.clone(),
            console_cursor: emu.console_cursor,
        }
    }
}

impl From<NativeSnapshot> for Emu {
    fn from(snapshot: NativeSnapshot) -> Self {
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
        emu.ipc = snapshot.ipc;
        emu.windows_tls_slots = snapshot.tls_slots;
        emu.windows_fls_slots = snapshot.fls_slots;
        emu.veh_registration_ids = snapshot.veh_registration_ids;
        emu.last_veh_id = snapshot.last_veh_id;
        emu.priorities = snapshot.priorities;
        emu.last_luid = snapshot.last_luid;
        emu.atom_tables = snapshot.atom_tables;
        emu.profiles = snapshot.profiles;
        emu.console_cursor = snapshot.console_cursor;
        emu
    }
}
