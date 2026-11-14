use serde::{Deserialize, Serialize};

use crate::emu::Emu;
use crate::emu::object_handle::{HandleManagement, MemoryHandles};
use crate::kernel::heap::KernelHeap;
use crate::maps::heap_allocation::{O1Heap, O1HeapSnapshot};
use crate::serialization::emu::SerializableEmu;

#[derive(Serialize, Deserialize)]
pub(crate) struct NativeSnapshot {
    emulator: SerializableEmu,
    heaps: Vec<O1HeapSnapshot>,
    handles: MemoryHandles,
    guard_heap: Option<KernelHeap>,
}

impl From<&Emu> for NativeSnapshot {
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

impl From<NativeSnapshot> for Emu {
    fn from(snapshot: NativeSnapshot) -> Self {
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
