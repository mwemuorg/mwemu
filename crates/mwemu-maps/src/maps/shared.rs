//! Shared backing storage for mapped views. Private maps retain their Vec fast path.
use crate::maps::Maps;
use crate::maps::mem64::{Mem64, Permission};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct SharedStorage {
    pub bytes: Rc<RefCell<Vec<u8>>>,
    pub dirty: Rc<Cell<bool>>,
}

impl SharedStorage {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Rc::new(RefCell::new(bytes)),
            dirty: Rc::new(Cell::new(false)),
        }
    }

    pub fn len(&self) -> usize {
        self.bytes.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn deep_clone(&self) -> Self {
        let clone = Self::new(self.bytes.borrow().clone());
        clone.dirty.set(self.dirty.get());
        clone
    }
}

impl Serialize for SharedStorage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.bytes.borrow().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SharedStorage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<u8>::deserialize(deserializer).map(Self::new)
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SharedView {
    pub storage: u64,
    pub offset: usize,
}

impl Maps {
    pub fn create_shared_storage(&mut self, bytes: Vec<u8>) -> u64 {
        let id = self
            .shared_stores
            .last_key_value()
            .map_or(1, |(&id, _)| id + 1);
        self.shared_stores.insert(id, SharedStorage::new(bytes));
        id
    }

    pub fn create_shared_view(
        &mut self,
        name: &str,
        address: u64,
        size: u64,
        storage: u64,
        offset: u64,
        permission: Permission,
    ) -> Result<(), String> {
        let offset = usize::try_from(offset).map_err(|_| "view offset overflow")?;
        let size = usize::try_from(size).map_err(|_| "view size overflow")?;
        let backing = self
            .shared_stores
            .get(&storage)
            .ok_or("unknown section")?
            .clone();
        if offset
            .checked_add(size)
            .is_none_or(|end| end > backing.len())
            || size == 0
            || !self.is_range_free(address, size as u64)
        {
            return Err("invalid view range".into());
        }
        if self.exists_mapname(name) {
            return Err("map name already exists".into());
        }
        let mut mem = Mem64::new(
            name.into(),
            address,
            address + size as u64,
            Vec::new(),
            permission,
        );
        mem.bind_shared(SharedView { storage, offset }, backing);
        let key = self.mem_slab.insert(mem);
        self.maps.insert(address, key);
        self.name_map.insert(name.into(), key);
        self.tlb.get_mut().flush();
        self.note_code_write(address, size as u64);
        Ok(())
    }

    pub fn reconnect_shared_views(&mut self) {
        for (_, mem) in self.mem_slab.iter_mut() {
            if let Some(view) = mem.shared_view()
                && let Some(backing) = self.shared_stores.get(&view.storage)
            {
                mem.bind_shared(view, backing.clone());
            }
        }
    }
}

impl Clone for Maps {
    fn clone(&self) -> Self {
        let mut clone = Self {
            buffers: self.buffers.clone(),
            allocations: self.allocations.clone(),
            banzai: self.banzai,
            mem_slab: self.mem_slab.clone(),
            maps: self.maps.clone(),
            name_map: self.name_map.clone(),
            is_64bits: self.is_64bits,
            tlb: self.tlb.clone(),
            max_alloc_size: self.max_alloc_size,
            code_writes: self.code_writes.clone(),
            shared_stores: self
                .shared_stores
                .iter()
                .map(|(&id, storage)| (id, storage.deep_clone()))
                .collect::<BTreeMap<_, _>>(),
        };
        clone.reconnect_shared_views();
        clone
    }
}
