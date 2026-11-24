use crate::maps::mem64::Permission;
use crate::winapi::common::wait::WaitObject;
use crate::winapi::libraries::gdi32::{DeviceContext, GraphicHandle};
pub(crate) use file_handle::FileHandle;
pub(crate) use heap_handle::HeapHandle;
pub(crate) use mapping_handle::MappingHandle;
use serde::{Deserialize, Serialize};
use slab::Slab;
use std::collections::{BTreeMap, VecDeque};
use toolhelp::ToolhelpSnapshot;

pub mod file_handle;
pub mod heap_handle;
mod hive_parser;
pub(crate) mod internet;
mod registry_watch;
use internet::InternetHandle;
pub mod mapping_handle;
mod registry_handle;
pub(crate) mod toolhelp;
mod windows_path;
// TODO: support more handle: registry, thread, etc
/*
Here the handle management is control by Slab and return a number, that number can be used as
handle id to get the right handle. In the document, it doesn't specific that the handle need to be divided by 4.

*/

#[derive(Clone, Serialize, Deserialize)]
pub struct SectionHandle {
    pub name: String,
    pub max_size: u64,
    pub protection: u32,
    pub storage: u64,
    pub access: Option<Permission>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct FoundFile {
    pub name: String,
    pub attributes: u32,
    pub times: [u64; 3],
    pub size: u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum ServiceObject {
    Manager { access: u32 },
    Service { name: String, access: u32 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum NativeObject {
    CsrPort,
    KnownDllDirectory { wow64: bool, access: u32 },
    KnownDllPath { wow64: bool, access: u32 },
    IoCompletion,
    WaitCompletionPacket,
    EtwRegistration,
    ConsoleDevice { access: u32, attributes: u32 },
    CsrClientPort { view_base: u64, storage: u64 },
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub(crate) struct HandleGrant {
    access: u32,
    attributes: u32,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct RegistryHandle {
    path: String,
    access: Option<u32>,
    tags: u16,
    notifications: Vec<registry_watch::NameNotification>,
}

enum HandleType {
    /// Permanently occupies slab slot 0 so no real handle ever gets the
    /// NULL-like key 0 (guests treat handle 0 as "no handle").
    Reserved,
    FileHandle(FileHandle),
    MappingHandle(MappingHandle),
    HeapHandle(HeapHandle),
    SectionHandle(SectionHandle),
    Toolhelp(ToolhelpSnapshot),
    Thread(u64, u32),
    RegistryKey(RegistryHandle),
    FileSearch(VecDeque<FoundFile>),
    Process(u32),
    Ipc,
    ProcessToken(u32),
    Waitable(u64, HandleGrant),
    ProcessAccess(u32, HandleGrant),
    Console(u32, u32),
    DeviceContext(DeviceContext),
    Service(ServiceObject),
    Internet(InternetHandle),
    UrlCacheGroup,
    EventLog {
        source: bool,
    },
    Graphic(GraphicHandle),
    Native(NativeObject),
}

#[derive(Serialize, Deserialize)]
pub(crate) enum MemoryHandle {
    Reserved,
    Heap(HeapHandle),
    Section(SectionHandle),
    Mapping(MappingHandle),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MemoryHandles {
    entries: Vec<(usize, MemoryHandle)>,
    process_heap_key: Option<u32>,
    sections: BTreeMap<u64, SectionHandle>,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum WindowsHandle {
    Toolhelp(ToolhelpSnapshot),
    Thread(u64, u32),
    RegistryKey(String),
    FileSearch(VecDeque<FoundFile>),
    Process(u32),
    Ipc,
    ProcessToken(u32),
    Waitable(u64, WaitObject, HandleGrant),
    ProcessAccess(u32, HandleGrant),
    Console(u32, u32),
    DeviceContext(DeviceContext),
    RegistryKeyAccess(String, u32),
    Service(ServiceObject),
    Internet(InternetHandle),
    UrlCacheGroup,
    EventLog { source: bool },
    Graphic(GraphicHandle),
    Native(NativeObject),
    TaggedRegistryKey(RegistryHandle, Vec<(u64, WaitObject)>),
}

pub struct HandleManagement {
    waitables: BTreeMap<u64, WaitObject>,
    number_of_handle: usize,
    handle_types: Slab<HandleType>,
    process_heap_key: Option<u32>,
    sections: BTreeMap<u64, SectionHandle>,
}

impl Default for HandleManagement {
    fn default() -> Self {
        Self::new()
    }
}

impl HandleManagement {
    pub fn new() -> Self {
        let mut handle_types = Slab::with_capacity(200);
        handle_types.insert(HandleType::Reserved);
        Self {
            handle_types,
            number_of_handle: 0,
            process_heap_key: None,
            sections: BTreeMap::new(),
            waitables: BTreeMap::new(),
        }
    }

    pub(crate) fn memory_snapshot(&self) -> MemoryHandles {
        let entries = self
            .handle_types
            .iter()
            .map(|(key, handle)| {
                let saved = match handle {
                    HandleType::HeapHandle(heap) => MemoryHandle::Heap(heap.clone()),
                    HandleType::SectionHandle(section) => MemoryHandle::Section(section.clone()),
                    HandleType::MappingHandle(mapping) => MemoryHandle::Mapping(mapping.clone()),
                    _ => MemoryHandle::Reserved,
                };
                (key, saved)
            })
            .collect();
        MemoryHandles {
            entries,
            process_heap_key: self.process_heap_key,
            sections: self.sections.clone(),
        }
    }

    pub(crate) fn from_memory_snapshot(snapshot: MemoryHandles) -> Self {
        let handle_types: Slab<_> = snapshot
            .entries
            .into_iter()
            .map(|(key, handle)| {
                let restored = match handle {
                    MemoryHandle::Reserved => HandleType::Reserved,
                    MemoryHandle::Heap(heap) => HandleType::HeapHandle(heap),
                    MemoryHandle::Section(section) => HandleType::SectionHandle(section),
                    MemoryHandle::Mapping(mapping) => HandleType::MappingHandle(mapping),
                };
                (key, restored)
            })
            .collect();
        let number_of_handle = handle_types
            .iter()
            .filter(|(_, h)| !matches!(h, HandleType::Reserved))
            .count();
        Self {
            handle_types,
            number_of_handle,
            process_heap_key: snapshot.process_heap_key,
            sections: snapshot.sections,
            waitables: BTreeMap::new(),
        }
    }

    pub(crate) fn windows_snapshot(&self) -> Vec<(usize, WindowsHandle)> {
        self.handle_types
            .iter()
            .filter_map(|(key, handle)| {
                let saved = match handle {
                    HandleType::Toolhelp(snapshot) => WindowsHandle::Toolhelp(snapshot.clone()),
                    HandleType::Thread(id, access) => WindowsHandle::Thread(*id, *access),
                    HandleType::Process(id) => WindowsHandle::Process(*id),
                    HandleType::ProcessAccess(id, mask) => WindowsHandle::ProcessAccess(*id, *mask),
                    HandleType::Ipc => WindowsHandle::Ipc,
                    HandleType::Waitable(id, grant) => {
                        WindowsHandle::Waitable(*id, self.waitables.get(id)?.clone(), *grant)
                    }
                    HandleType::Console(selector, mode) => WindowsHandle::Console(*selector, *mode),
                    HandleType::UrlCacheGroup => WindowsHandle::UrlCacheGroup,
                    HandleType::Internet(handle) => WindowsHandle::Internet(handle.clone()),
                    HandleType::Native(object) => WindowsHandle::Native(*object),
                    HandleType::Graphic(handle) => WindowsHandle::Graphic(handle.clone()),
                    HandleType::EventLog { source } => WindowsHandle::EventLog { source: *source },
                    HandleType::Service(service) => WindowsHandle::Service(service.clone()),
                    HandleType::DeviceContext(dc) => WindowsHandle::DeviceContext(dc.clone()),
                    HandleType::ProcessToken(rights) => WindowsHandle::ProcessToken(*rights),
                    HandleType::RegistryKey(key) => WindowsHandle::TaggedRegistryKey(
                        key.clone(),
                        key.notifications
                            .iter()
                            .filter_map(|watch| {
                                self.waitables
                                    .get(&watch.event)
                                    .cloned()
                                    .map(|event| (watch.event, event))
                            })
                            .collect(),
                    ),
                    HandleType::FileSearch(entries) => WindowsHandle::FileSearch(entries.clone()),
                    _ => return None,
                };
                Some((key, saved))
            })
            .collect()
    }

    pub(crate) fn restore_windows_snapshot(&mut self, snapshot: Vec<(usize, WindowsHandle)>) {
        // The memory snapshot reserves these keys, keeping older formats unchanged.
        for (key, saved) in snapshot {
            if let Some(handle @ HandleType::Reserved) = self.handle_types.get_mut(key) {
                *handle = match saved {
                    WindowsHandle::Toolhelp(snapshot) => HandleType::Toolhelp(snapshot),
                    WindowsHandle::FileSearch(entries) => HandleType::FileSearch(entries),
                    WindowsHandle::Thread(id, access) => HandleType::Thread(id, access),
                    WindowsHandle::Process(id) => HandleType::Process(id),
                    WindowsHandle::ProcessAccess(id, mask) => HandleType::ProcessAccess(id, mask),
                    WindowsHandle::Ipc => HandleType::Ipc,
                    WindowsHandle::Waitable(id, obj, grant) => {
                        self.waitables.entry(id).or_insert(obj);
                        HandleType::Waitable(id, grant)
                    }
                    WindowsHandle::Console(selector, mode) => HandleType::Console(selector, mode),
                    WindowsHandle::UrlCacheGroup => HandleType::UrlCacheGroup,
                    WindowsHandle::Internet(handle) => HandleType::Internet(handle),
                    WindowsHandle::Native(object) => HandleType::Native(object),
                    WindowsHandle::Graphic(handle) => HandleType::Graphic(handle),
                    WindowsHandle::EventLog { source } => HandleType::EventLog { source },
                    WindowsHandle::Service(service) => HandleType::Service(service),
                    WindowsHandle::DeviceContext(dc) => HandleType::DeviceContext(dc),
                    WindowsHandle::ProcessToken(rights) => HandleType::ProcessToken(rights),
                    WindowsHandle::RegistryKeyAccess(path, access) => {
                        HandleType::RegistryKey(RegistryHandle {
                            path,
                            access: Some(access),
                            tags: 0,
                            notifications: Vec::new(),
                        })
                    }
                    WindowsHandle::RegistryKey(path) => HandleType::RegistryKey(RegistryHandle {
                        path,
                        access: None,
                        tags: 0,
                        notifications: Vec::new(),
                    }),
                    WindowsHandle::TaggedRegistryKey(key, events) => {
                        for (id, event) in events {
                            self.waitables.entry(id).or_insert(event);
                        }
                        HandleType::RegistryKey(key)
                    }
                };
                self.number_of_handle += 1;
            }
        }
    }

    fn insert_registry_key(&mut self, path: String, access: Option<u32>) -> u32 {
        // Guest advapi32 uses the low two bits of a local HKEY to tag other providers.
        const OBJ_HANDLE_TAGBITS: usize = 0x3;
        let mut skipped = Vec::new();
        while self.handle_types.vacant_key() & OBJ_HANDLE_TAGBITS != 0 {
            skipped.push(self.handle_types.insert(HandleType::Reserved));
        }
        let handle = self
            .handle_types
            .insert(HandleType::RegistryKey(RegistryHandle {
                path,
                access,
                tags: 0,
                notifications: Vec::new(),
            })) as u32;
        for slot in skipped.into_iter().rev() {
            self.handle_types.remove(slot);
        }
        self.number_of_handle += 1;
        handle
    }

    pub(crate) fn open_registry_key(&mut self, path: String) -> u32 {
        self.insert_registry_key(path, None)
    }

    pub(crate) fn open_registry_key_access(&mut self, path: String, access: u32) -> u32 {
        self.insert_registry_key(path, Some(access))
    }

    pub(crate) fn registry_can_query(&self, handle: u64) -> bool {
        use crate::windows::constants::security::{
            GENERIC_ALL, GENERIC_READ, KEY_QUERY_VALUE, MAXIMUM_ALLOWED,
        };
        self.registry_handle(handle).is_some_and(|key| {
            key.access.is_none_or(|mask| {
                mask & (KEY_QUERY_VALUE | GENERIC_READ | GENERIC_ALL | MAXIMUM_ALLOWED) != 0
            })
        })
    }

    fn registry_handle(&self, handle: u64) -> Option<&RegistryHandle> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::RegistryKey(key) => Some(key),
            _ => None,
        }
    }

    pub(crate) fn registry_key(&self, handle: u64) -> Option<&str> {
        self.registry_handle(handle).map(|key| key.path.as_str())
    }

    pub(crate) fn registry_tags(&self, handle: u64) -> Option<u16> {
        self.registry_handle(handle).map(|key| key.tags)
    }

    pub(crate) fn registry_tags_mut(&mut self, handle: u64) -> Option<&mut u16> {
        match self.handle_types.get_mut(usize::try_from(handle).ok()?)? {
            HandleType::RegistryKey(key) => Some(&mut key.tags),
            _ => None,
        }
    }

    pub(crate) fn close_registry_key(&mut self, handle: u64) -> bool {
        if self.registry_key(handle).is_none() {
            return false;
        }
        if let HandleType::RegistryKey(key) = self.handle_types.remove(handle as usize) {
            self.finish_registry_notifications(key.notifications);
        }
        self.number_of_handle -= 1;
        true
    }

    pub(crate) fn invalidate_registry_key(&mut self, path: &str, include_root: bool) {
        let mut notifications = Vec::new();
        for (_, entry) in &mut self.handle_types {
            if let HandleType::RegistryKey(key) = entry
                && ((include_root && key.path.eq_ignore_ascii_case(path))
                    || registry_watch::descendant(&key.path, path))
            {
                key.path.clear();
                notifications.append(&mut key.notifications);
            }
        }
        self.finish_registry_notifications(notifications);
    }

    pub(crate) fn named_waitable(&self, name: &[u16]) -> Option<u64> {
        self.waitables
            .iter()
            .find_map(|(&id, object)| (object.name.as_deref() == Some(name)).then_some(id))
    }

    pub(crate) fn create_waitable(&mut self, object: WaitObject) -> Option<u64> {
        let id = self
            .waitables
            .last_key_value()
            .map(|(&id, _)| id)
            .unwrap_or(0)
            .checked_add(1)?;
        self.waitables.insert(id, object);
        Some(id)
    }

    pub(crate) fn open_waitable(&mut self, id: u64, access: u32, attributes: u32) -> u64 {
        self.number_of_handle += 1;
        self.handle_types
            .insert(HandleType::Waitable(id, HandleGrant { access, attributes })) as u64
    }

    pub(crate) fn waitable_id(&self, handle: u64) -> Option<u64> {
        self.waitable_handle(handle).map(|(id, _)| id)
    }

    pub(crate) fn waitable_handle(&self, handle: u64) -> Option<(u64, u32)> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::Waitable(id, grant) => Some((*id, grant.access)),
            _ => None,
        }
    }

    pub(crate) fn waitable(&mut self, id: u64) -> Option<&mut WaitObject> {
        self.waitables.get_mut(&id)
    }

    pub(crate) fn open_process_token(&mut self, rights: u32) -> u32 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::ProcessToken(rights)) as u32
    }

    pub(crate) fn process_token_rights(&self, handle: u64) -> Option<u32> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::ProcessToken(rights) => Some(*rights),
            _ => None,
        }
    }

    pub(crate) fn console_handle(&mut self, selector: u32, default_mode: u32) -> u64 {
        use crate::windows::constants::files::STD_INPUT_HANDLE;
        let mut mode = default_mode;
        for (key, entry) in &self.handle_types {
            if let HandleType::Console(existing, flags) = entry {
                if *existing == selector {
                    return key as u64;
                }
                if *existing != STD_INPUT_HANDLE && selector != STD_INPUT_HANDLE {
                    mode = *flags;
                }
            }
        }
        self.number_of_handle += 1;
        self.handle_types
            .insert(HandleType::Console(selector, mode)) as u64
    }

    pub(crate) fn console(&self, handle: u64) -> Option<(u32, u32)> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::Console(selector, mode) => Some((*selector, *mode)),
            _ => None,
        }
    }

    pub(crate) fn set_console_mode(&mut self, selector: u32, mode: u32) {
        use crate::windows::constants::files::STD_INPUT_HANDLE;
        for (_, entry) in &mut self.handle_types {
            if let HandleType::Console(existing, flags) = entry
                && (*existing == STD_INPUT_HANDLE) == (selector == STD_INPUT_HANDLE)
            {
                *flags = mode;
            }
        }
    }

    pub(crate) fn open_event_log(&mut self, source: bool) -> u64 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::EventLog { source }) as u64
    }

    pub(crate) fn is_event_log(&self, handle: u64) -> bool {
        usize::try_from(handle).ok().is_some_and(|key| {
            matches!(
                self.handle_types.get(key),
                Some(HandleType::EventLog { .. })
            )
        })
    }

    pub(crate) fn close_event_log(&mut self, handle: u64, source: bool) -> bool {
        let Ok(key) = usize::try_from(handle) else {
            return false;
        };
        if !matches!(self.handle_types.get(key), Some(HandleType::EventLog { source: actual }) if *actual == source)
        {
            return false;
        }
        self.handle_types.remove(key);
        self.number_of_handle -= 1;
        true
    }

    pub(crate) fn open_service_object(&mut self, service: ServiceObject) -> u64 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::Service(service)) as u64
    }

    pub(crate) fn service_manager_access(&self, handle: u64) -> Option<u32> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::Service(ServiceObject::Manager { access }) => Some(*access),
            _ => None,
        }
    }

    pub(crate) fn close_service_object(&mut self, handle: u64) -> bool {
        let Ok(key) = usize::try_from(handle) else {
            return false;
        };
        if !matches!(self.handle_types.get(key), Some(HandleType::Service(_))) {
            return false;
        }
        self.handle_types.remove(key);
        self.number_of_handle -= 1;
        true
    }

    pub(crate) fn insert_native_object(&mut self, object: NativeObject) -> u64 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::Native(object)) as u64
    }

    pub(crate) fn native_object(&self, handle: u64) -> Option<NativeObject> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::Native(object) => Some(*object),
            _ => None,
        }
    }

    pub(crate) fn insert_graphic(&mut self, graphic: GraphicHandle) -> u64 {
        self.number_of_handle += 1;
        let entry = self.handle_types.vacant_entry();
        let handle = graphic.handle(entry.key());
        entry.insert(HandleType::Graphic(graphic));
        handle
    }

    pub(crate) fn graphic(&self, object: u64) -> Option<&GraphicHandle> {
        let raw = usize::try_from(object).ok()?;
        if let Some(HandleType::Graphic(entry)) = self.handle_types.get(raw) {
            return Some(entry);
        }
        let slot = (object as u16) as usize;
        match self.handle_types.get(slot)? {
            HandleType::Graphic(entry) if entry.handle(slot) == object => Some(entry),
            _ => None,
        }
    }

    pub(crate) fn stock_graphic(&self, stock: i32) -> Option<u64> {
        self.handle_types
            .iter()
            .find_map(|(key, object)| match object {
                HandleType::Graphic(entry) if entry.stock == Some(stock) => Some(entry.handle(key)),
                _ => None,
            })
    }

    pub(crate) fn graphic_in_other_dc(&self, object: u64, owner: u64) -> bool {
        self.handle_types.iter().any(|(key, entry)| {
            key as u64 != owner
                && matches!(entry, HandleType::DeviceContext(dc) if dc.holds_graphic(object))
        })
    }

    pub(crate) fn delete_graphic(&mut self, object: u64) -> bool {
        let Some(entry) = self.graphic(object) else {
            return false;
        };
        if entry.stock.is_some() {
            return true;
        }
        if self.graphic_in_other_dc(object, 0) {
            return false;
        }
        self.handle_types.remove(object as usize);
        self.number_of_handle -= 1;
        true
    }

    pub(crate) fn insert_dc(&mut self, dc: DeviceContext) -> u64 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::DeviceContext(dc)) as u64
    }
    pub(crate) fn dc(&mut self, h: u64) -> Option<&mut DeviceContext> {
        match self.handle_types.get_mut(usize::try_from(h).ok()?)? {
            HandleType::DeviceContext(dc) => Some(dc),
            _ => None,
        }
    }
    pub(crate) fn release_dc(&mut self, h: u64, owner: Option<u64>) -> bool {
        if self.dc(h).is_none_or(|dc| dc.window != owner) {
            return false;
        }
        self.handle_types.remove(h as usize);
        self.number_of_handle -= 1;
        true
    }
    pub(crate) fn insert_ipc_handle(&mut self) -> u32 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::Ipc) as u32
    }

    pub(crate) fn insert_process_handle(&mut self, id: u32, access: u32, attributes: u32) -> u32 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::ProcessAccess(
            id,
            HandleGrant { access, attributes },
        )) as u32
    }

    pub(crate) fn is_console_device(&self, handle: u64) -> bool {
        matches!(
            self.native_object(handle),
            Some(NativeObject::ConsoleDevice { .. })
        )
    }

    pub(crate) fn native_type_name(&self, handle: u64) -> Option<&'static str> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::FileHandle(_) | HandleType::Console(..) | HandleType::Ipc => Some("File"),
            HandleType::SectionHandle(_) | HandleType::MappingHandle(_) => Some("Section"),
            HandleType::Process(_) | HandleType::ProcessAccess(..) => Some("Process"),
            HandleType::Thread(..) => Some("Thread"),
            HandleType::ProcessToken(_) => Some("Token"),
            HandleType::RegistryKey(_) => Some("Key"),
            HandleType::Native(NativeObject::KnownDllDirectory { .. }) => Some("Directory"),
            HandleType::Native(NativeObject::KnownDllPath { .. }) => Some("SymbolicLink"),
            HandleType::Native(NativeObject::ConsoleDevice { .. }) => Some("File"),
            HandleType::Native(NativeObject::IoCompletion) => Some("IoCompletion"),
            HandleType::Native(NativeObject::WaitCompletionPacket) => Some("WaitCompletionPacket"),
            _ => None,
        }
    }

    pub(crate) fn contains_handle(&self, handle: u64) -> bool {
        handle != 0 && usize::try_from(handle).is_ok_and(|index| self.handle_types.contains(index))
    }

    pub(crate) fn process_handle_id(&self, handle: u64) -> Option<u32> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::Process(id) | HandleType::ProcessAccess(id, _) => Some(*id),
            _ => None,
        }
    }

    pub(crate) fn handle_attributes(&self, handle: u64) -> Option<u32> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::ProcessAccess(_, grant) | HandleType::Waitable(_, grant) => {
                Some(grant.attributes)
            }
            HandleType::Native(NativeObject::ConsoleDevice { attributes, .. }) => Some(*attributes),
            HandleType::Process(_) => Some(0),
            _ => None,
        }
    }

    pub(crate) fn process_access(&self, handle: u64) -> Option<u32> {
        match self.handle_types.get(usize::try_from(handle).ok()?)? {
            HandleType::ProcessAccess(_, grant) => Some(grant.access),
            // Old snapshots recorded identity but no granted VM access.
            HandleType::Process(_) => Some(0),
            _ => None,
        }
    }

    pub(crate) fn insert_thread_handle(&mut self, id: u64, access: u32) -> u32 {
        let key = self.handle_types.insert(HandleType::Thread(id, access));
        self.number_of_handle += 1;
        key as u32
    }

    pub(crate) fn thread_handle_id(&self, handle: u64) -> Option<u64> {
        self.thread_handle(handle).map(|(id, _)| id)
    }

    pub(crate) fn thread_handle(&self, handle: u64) -> Option<(u64, u32)> {
        let key = usize::try_from(handle).ok()?;
        match self.handle_types.get(key) {
            Some(HandleType::Thread(id, access)) => Some((*id, *access)),
            _ => None,
        }
    }

    pub(crate) fn insert_toolhelp_snapshot(&mut self, snapshot: ToolhelpSnapshot) -> u32 {
        let key = self.handle_types.insert(HandleType::Toolhelp(snapshot));
        self.number_of_handle += 1;
        key as u32
    }

    pub(crate) fn toolhelp_snapshot(&self, handle: u64) -> Option<&ToolhelpSnapshot> {
        let key = usize::try_from(handle).ok()?;
        match self.handle_types.get(key) {
            Some(HandleType::Toolhelp(snapshot)) => Some(snapshot),
            _ => None,
        }
    }

    pub(crate) fn toolhelp_snapshot_mut(&mut self, handle: u64) -> Option<&mut ToolhelpSnapshot> {
        let key = usize::try_from(handle).ok()?;
        match self.handle_types.get_mut(key) {
            Some(HandleType::Toolhelp(snapshot)) => Some(snapshot),
            _ => None,
        }
    }

    pub(crate) fn insert_file_search(&mut self, entries: VecDeque<FoundFile>) -> u32 {
        self.number_of_handle += 1;
        self.handle_types.insert(HandleType::FileSearch(entries)) as u32
    }

    pub(crate) fn file_search(&mut self, handle: u64) -> Option<&mut VecDeque<FoundFile>> {
        match self.handle_types.get_mut(usize::try_from(handle).ok()?)? {
            HandleType::FileSearch(entries) => Some(entries),
            _ => None,
        }
    }

    pub(crate) fn close_file_search(&mut self, handle: u64) -> bool {
        if self.file_search(handle).is_none() {
            return false;
        }
        self.handle_types.remove(handle as usize);
        self.number_of_handle -= 1;
        true
    }

    pub fn insert_file_handle(&mut self, file_handle: FileHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::FileHandle(file_handle));
        self.number_of_handle += 1;
        key as u32 // Assuming u32 is sufficient for slab keys in your context
    }

    pub fn insert_mapping_handle(&mut self, mapping_handle: MappingHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::MappingHandle(mapping_handle));
        self.number_of_handle += 1;
        key as u32
    }

    // Method to get a mutable reference to a FileHandle by its key
    pub fn get_mut_file_handle(&mut self, key: u32) -> Option<&mut FileHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::FileHandle(fh) => Some(fh),
                // Add other handle type matches if/when they are implemented
                _ => None, // Handle exists but is not a FileHandle
            }
        } else {
            None // Handle key does not exist
        }
    }

    pub fn get_mut_mapping_handle(&mut self, key: u32) -> Option<&mut MappingHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::MappingHandle(mh) => Some(mh),
                _ => None,
            }
        } else {
            None
        }
    }

    // Method to remove a FileHandle (useful for CloseHandle)
    pub fn remove_file_handle(&mut self, key: u32) -> Option<FileHandle> {
        let file_handle = self.get_mut_file_handle(key)?;
        file_handle.close();
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::FileHandle(fh) => {
                    self.number_of_handle -= 1;
                    Some(fh)
                }
                _ => {
                    // Put it back if it wasn't a FileHandle, though this indicates a logic error
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn remove_mapping_handle(&mut self, key: u32) -> Option<MappingHandle> {
        if !matches!(
            self.handle_types.get(key as usize),
            Some(HandleType::MappingHandle(_))
        ) {
            return None;
        }
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::MappingHandle(mh) => {
                    self.number_of_handle -= 1;
                    Some(mh)
                }
                _ => {
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn insert_heap_handle(&mut self, heap_handle: HeapHandle) -> u32 {
        let key = self
            .handle_types
            .insert(HandleType::HeapHandle(heap_handle));
        self.number_of_handle += 1;
        key as u32
    }

    pub fn remove_heap_handle(&mut self, key: u32) -> Option<HeapHandle> {
        if !matches!(
            self.handle_types.get(key as usize),
            Some(HandleType::HeapHandle(_))
        ) {
            return None;
        }
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::HeapHandle(hh) => {
                    self.number_of_handle -= 1;
                    Some(hh)
                }
                _ => {
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }
    /// Return the slab key for the implicit process-heap `HeapHandle`,
    /// creating one (bound to arena 0) on first call. Slot 0 is reserved,
    /// so the first real handle (and thus this key) is always non-zero.
    pub fn get_or_insert_process_heap(&mut self) -> u32 {
        if let Some(k) = self.process_heap_key
            && matches!(
                self.handle_types.get(k as usize),
                Some(HandleType::HeapHandle(_))
            )
        {
            return k;
        }
        for (k, entry) in self.handle_types.iter() {
            if let HandleType::HeapHandle(hh) = entry
                && hh.arena == 0
            {
                let key = k as u32;
                self.process_heap_key = Some(key);
                return key;
            }
        }
        let key = self.insert_heap_handle(HeapHandle::new(0, 0, 0, 0));
        self.process_heap_key = Some(key);
        key
    }

    pub fn is_process_heap(&self, key: u32) -> bool {
        self.process_heap_key == Some(key)
    }

    /// Resolve a guest-visible heap handle (base address or legacy slab key)
    /// to its internal slab key. Tries base_addr lookup first, then falls
    /// back to direct slab key for backwards compatibility with tests.
    pub fn resolve_heap_key(&self, handle: u64) -> Option<u32> {
        if let Some(k) = self.key_for_base_addr(handle) {
            return Some(k);
        }
        if handle <= u32::MAX as u64 {
            let key = handle as u32;
            if matches!(
                self.handle_types.get(key as usize),
                Some(HandleType::HeapHandle(_))
            ) {
                return Some(key);
            }
        }
        None
    }

    /// Resolve a guest-supplied heap handle to its owning `HeapHandle`
    /// allocation context: `(arena_idx, maximum_size)`.
    ///
    /// Tries the value as a slab key first (legacy / test paths that pass
    /// small ints like `0x1234`), then as a real arena base address (the
    /// value `GetProcessHeap`/`HeapCreate` hand to the guest).
    pub fn heap_alloc_context(&self, handle: u64) -> Option<(usize, u64)> {
        if handle <= u32::MAX as u64 {
            let key = handle as u32;
            if let Some(HandleType::HeapHandle(hh)) = self.handle_types.get(key as usize) {
                return Some((hh.arena, hh.maxSZ as u64));
            }
        }
        let key = self.key_for_base_addr(handle)?;
        if let Some(HandleType::HeapHandle(hh)) = self.handle_types.get(key as usize) {
            Some((hh.arena, hh.maxSZ as u64))
        } else {
            None
        }
    }

    /// Resolve a guest-visible heap base address back to its slab key.
    pub fn key_for_base_addr(&self, addr: u64) -> Option<u32> {
        if addr == 0 {
            return None;
        }
        for (k, entry) in self.handle_types.iter() {
            if let HandleType::HeapHandle(hh) = entry
                && hh.base_addr == addr
            {
                return Some(k as u32);
            }
        }
        None
    }

    /// Record the arena base address on a heap handle after creation.
    pub fn set_heap_base_addr(&mut self, key: u32, addr: u64) {
        if let Some(HandleType::HeapHandle(hh)) = self.handle_types.get_mut(key as usize) {
            hh.base_addr = addr;
        }
    }

    /// Return the guest-visible base address for a heap slab key.
    pub fn heap_base_addr(&self, key: u32) -> Option<u64> {
        match self.handle_types.get(key as usize) {
            Some(HandleType::HeapHandle(hh)) if hh.base_addr != 0 => Some(hh.base_addr),
            _ => None,
        }
    }

    pub fn section_handle(&self, key: u64) -> Option<&SectionHandle> {
        let key = usize::try_from(key).ok()?;
        match self.handle_types.get(key) {
            Some(HandleType::SectionHandle(section)) => Some(section),
            _ => None,
        }
    }

    pub fn named_section(&self, name: &str) -> Option<&SectionHandle> {
        self.sections
            .values()
            .find(|section| !name.is_empty() && section.name == name)
    }

    pub fn forget_section(&mut self, storage: u64) {
        self.sections.remove(&storage);
    }

    pub fn section_uses_storage(&self, id: u64) -> bool {
        self.handle_types
            .iter()
            .any(|(_, h)| matches!(h, HandleType::SectionHandle(s) if s.storage == id))
    }

    pub fn close_handle(&mut self, key: u64) -> bool {
        let Ok(key) = usize::try_from(key) else {
            return false;
        };
        if !matches!(
            self.handle_types.get(key),
            Some(
                HandleType::FileHandle(_)
                    | HandleType::MappingHandle(_)
                    | HandleType::SectionHandle(_)
                    | HandleType::Toolhelp(_)
                    | HandleType::Thread(..)
                    | HandleType::Process(_)
                    | HandleType::ProcessAccess(_, _)
                    | HandleType::Ipc
                    | HandleType::Native(_)
                    | HandleType::Console(_, _)
                    | HandleType::ProcessToken(_)
                    | HandleType::Waitable(..)
            )
        ) {
            return false;
        }
        match self.handle_types.try_remove(key) {
            Some(HandleType::FileHandle(mut file)) => file.close(),
            Some(HandleType::Waitable(id, _)) => self.release_unreferenced_waitable(id),
            _ => {}
        }
        self.number_of_handle -= 1;
        true
    }

    pub fn insert_section_handle(&mut self, section: SectionHandle) -> u32 {
        self.sections
            .entry(section.storage)
            .or_insert_with(|| section.clone());
        let key = self.handle_types.insert(HandleType::SectionHandle(section));
        self.number_of_handle += 1;
        key as u32
    }

    pub fn get_mut_section_handle(&mut self, key: u32) -> Option<&mut SectionHandle> {
        if let Some(handle_type) = self.handle_types.get_mut(key as usize) {
            match handle_type {
                HandleType::SectionHandle(sh) => Some(sh),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn remove_section_handle(&mut self, key: u32) -> Option<SectionHandle> {
        if !matches!(
            self.handle_types.get(key as usize),
            Some(HandleType::SectionHandle(_))
        ) {
            return None;
        }
        if let Some(handle_type) = self.handle_types.try_remove(key as usize) {
            match handle_type {
                HandleType::SectionHandle(sh) => {
                    self.number_of_handle -= 1;
                    Some(sh)
                }
                _ => {
                    self.handle_types.insert(handle_type);
                    None
                }
            }
        } else {
            None
        }
    }

    /// Slab keys of every live heap handle. The process-heap key is first
    /// (matches `RtlGetProcessHeaps` semantics).
    pub fn heap_handle_keys(&self) -> Vec<u32> {
        let mut out = Vec::new();
        if let Some(pk) = self.process_heap_key {
            out.push(pk);
        }
        for (k, entry) in self.handle_types.iter() {
            if matches!(entry, HandleType::HeapHandle(_)) && Some(k as u32) != self.process_heap_key
            {
                out.push(k as u32);
            }
        }
        out
    }
}
