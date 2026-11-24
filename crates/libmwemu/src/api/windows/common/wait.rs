use crate::api::abi::windows::{
    ArgumentError, CallingConvention::Stdcall, WindowsAbi, WindowsAbi::WinApi,
};
use crate::emu::Emu;
use crate::emu::object_handle::NativeObject;
use crate::maps::mem64::Permission;
use crate::winapi::common::{memory, nt_api, process, threads};
use crate::winapi::environment_store::Encoding;
use crate::winapi::policy::{self, api_try};
use crate::windows::constants::errors::*;
use crate::windows::constants::process::*;
use crate::windows::constants::security::{
    GENERIC_ALL, GENERIC_EXECUTE, GENERIC_WRITE, MAXIMUM_ALLOWED, SYNCHRONIZE,
};
use crate::windows::constants::system::{
    TIMER_MODIFY_STATE, TIMER2_ATTRIBUTE_IR_TIMER, TIMER2_ATTRIBUTE_KNOWN_MASK,
    TIMER2_ATTRIBUTE_NOTIFICATION,
};
use serde::{Deserialize, Serialize};

const MAXIMUM_WAIT_OBJECTS: usize = 64;
const INFINITE: u32 = u32::MAX;
const MANUAL_RESET: u32 = 1;
const INITIALLY_SIGNALED: u32 = 2;
const HUNDRED_NS_PER_MS: u64 = 10_000;

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct WaitObject {
    pub(crate) name: Option<Vec<u16>>,
    state: State,
}

#[derive(Clone, Serialize, Deserialize)]
enum State {
    Event { manual: bool, signaled: bool },
    Mutex { owner: Option<u64>, depth: u32 },
    Semaphore { count: i32, limit: i32 },
    Timer(Timer),
    Timer2(Timer),
}

#[derive(Clone, Serialize, Deserialize)]
struct Timer {
    manual: bool,
    signaled: bool,
    deadline: Option<u64>,
    period: u32,
}

impl Timer {
    fn new(manual: bool) -> Self {
        Self {
            manual,
            signaled: false,
            deadline: None,
            period: 0,
        }
    }

    fn arm_relative(&mut self, now: u64, due: i64, period: u32) {
        self.deadline = Some(now.saturating_add(due.unsigned_abs().div_ceil(HUNDRED_NS_PER_MS)));
        self.period = period;
        self.signaled = false;
    }
}

impl WaitObject {
    pub(crate) fn set_event(&mut self, value: bool) -> bool {
        if let State::Event { signaled, .. } = &mut self.state {
            *signaled = value;
            true
        } else {
            false
        }
    }
}

impl State {
    fn ready(&mut self, tid: u64, now: u64) -> bool {
        match self {
            Self::Event { signaled, .. } => *signaled,
            Self::Mutex { owner, .. } => owner.is_none_or(|id| id == tid),
            Self::Semaphore { count, .. } => *count > 0,
            Self::Timer(timer) | Self::Timer2(timer) => {
                if let Some(due) = timer.deadline
                    && due <= now
                {
                    timer.signaled = true;
                    timer.deadline = if timer.period == 0 {
                        None
                    } else {
                        let step = timer.period as u64;
                        now.checked_add(step - (now - due) % step)
                    };
                }
                timer.signaled
            }
        }
    }

    fn acquire(&mut self, tid: u64) {
        match self {
            Self::Event {
                manual: false,
                signaled,
            } => *signaled = false,
            Self::Timer(timer) | Self::Timer2(timer) if !timer.manual => timer.signaled = false,
            Self::Mutex { owner, depth } => {
                *owner = Some(tid);
                *depth = depth.saturating_add(1);
            }
            Self::Semaphore { count, .. } => *count -= 1,
            _ => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Event,
    Mutex,
    Semaphore,
    Timer,
    Timer2,
}
impl State {
    fn kind(&self) -> Kind {
        match self {
            Self::Event { .. } => Kind::Event,
            Self::Mutex { .. } => Kind::Mutex,
            Self::Semaphore { .. } => Kind::Semaphore,
            Self::Timer(_) => Kind::Timer,
            Self::Timer2(_) => Kind::Timer2,
        }
    }
}
fn arguments(call: WindowsAbi, emu: &Emu, slots: usize) -> Result<Vec<u64>, ArgumentError> {
    (0..slots).map(|i| call.arg(emu, i)).collect()
}

fn return_win(emu: &mut Emu, slots: usize, result: Result<u64, u64>, failure: u64) {
    let call = WinApi(Stdcall);
    let value = result.unwrap_or_else(|error| {
        emu.set_last_error(error);
        failure
    });
    call.set_return(emu, value);
    call.cleanup_args(emu, slots);
}

fn object(
    emu: &mut Emu,
    name: Option<Vec<u16>>,
    kind: Kind,
    initial: Option<State>,
    access: u32,
    attributes: u32,
) -> Result<(u64, bool), u64> {
    let existing = name
        .as_ref()
        .and_then(|n| emu.handle_management.named_waitable(n));
    let id = if let Some(id) = existing {
        let obj = emu
            .handle_management
            .waitable(id)
            .ok_or(ERROR_INVALID_HANDLE)?;
        if obj.state.kind() != kind {
            return Err(ERROR_INVALID_HANDLE);
        }
        id
    } else {
        let state = initial.ok_or(ERROR_FILE_NOT_FOUND)?;
        emu.handle_management
            .create_waitable(WaitObject { name, state })
            .ok_or(ERROR_NOT_ENOUGH_MEMORY)?
    };
    Ok((
        emu.handle_management.open_waitable(id, access, attributes),
        existing.is_some(),
    ))
}

fn open_or_create(emu: &mut Emu, api: &str, kind: Kind, create: bool, ex: bool) {
    let call = WinApi(Stdcall);
    let (name_index, slots) = if !create {
        (2, 3)
    } else {
        match kind {
            Kind::Event | Kind::Mutex | Kind::Timer | Kind::Timer2 if ex => (1, 4),
            Kind::Event => (3, 4),
            Kind::Mutex | Kind::Timer | Kind::Timer2 => (2, 3),
            Kind::Semaphore => (3, if ex { 6 } else { 4 }),
        }
    };
    let args = api_try!(emu, arguments(call, emu, slots));
    let inherit = if !create {
        args[1] as u32 != 0
    } else if args[0] == 0 {
        false
    } else {
        let sa = args[0];
        let width = memory::pointer_size(emu);
        if !emu
            .maps
            .can_access_range(sa, (3 * width) as usize, Permission::READ)
        {
            return return_win(emu, slots, Err(ERROR_NOACCESS), 0);
        }
        if emu.maps.read_dword(sa) != Some((3 * width) as u32) {
            return return_win(emu, slots, Err(ERROR_INVALID_PARAMETER), 0);
        }
        if memory::read_pointer(emu, sa + width) != Some(0) {
            return policy::unsupported(emu, "kernel32", api, emu.pc());
        }
        emu.maps.read_dword(sa + 2 * width) != Some(0)
    };
    let name = if args[name_index] == 0 {
        None
    } else {
        match Encoding::from_wide(api.ends_with('W')).read(emu, args[name_index]) {
            Ok(value) if !value.is_empty() => Some(value),
            Ok(_) => None,
            Err(e) => return return_win(emu, slots, Err(e), 0),
        }
    };
    let initial = if !create {
        None
    } else {
        let flags = if ex { args[2] as u32 } else { 0 };
        let state = match kind {
            Kind::Event => {
                if ex && flags & !(MANUAL_RESET | INITIALLY_SIGNALED) != 0 {
                    return return_win(emu, slots, Err(ERROR_INVALID_PARAMETER), 0);
                }
                State::Event {
                    manual: if ex {
                        flags & MANUAL_RESET != 0
                    } else {
                        args[1] as u32 != 0
                    },
                    signaled: if ex {
                        flags & INITIALLY_SIGNALED != 0
                    } else {
                        args[2] as u32 != 0
                    },
                }
            }
            Kind::Mutex => {
                if ex && flags & !1 != 0 {
                    return return_win(emu, slots, Err(ERROR_INVALID_PARAMETER), 0);
                }
                let owned = if ex { flags != 0 } else { args[1] as u32 != 0 };
                State::Mutex {
                    owner: owned.then_some(emu.current_thread().id),
                    depth: u32::from(owned),
                }
            }
            Kind::Semaphore => {
                let count = args[1] as i32;
                let limit = args[2] as i32;
                if count < 0 || limit <= 0 || count > limit || ex && args[4] != 0 {
                    return return_win(emu, slots, Err(ERROR_INVALID_PARAMETER), 0);
                }
                State::Semaphore { count, limit }
            }
            Kind::Timer | Kind::Timer2 => {
                if ex && flags & !MANUAL_RESET != 0 {
                    policy::unsupported(emu, "kernel32", api, emu.pc());
                    return;
                }
                let manual = if ex {
                    flags & MANUAL_RESET != 0
                } else {
                    args[1] as u32 != 0
                };
                State::Timer(Timer::new(manual))
            }
        };
        Some(state)
    };
    let access = if !create {
        args[0] as u32
    } else if ex {
        args[slots - 1] as u32
    } else {
        u32::MAX
    };
    let attributes = if inherit { OBJ_INHERIT } else { 0 };
    let result = object(emu, name, kind, initial, access, attributes).map(|(h, exists)| {
        if create {
            emu.set_last_error(if exists {
                ERROR_ALREADY_EXISTS
            } else {
                ERROR_SUCCESS
            });
        }
        h
    });
    return_win(emu, slots, result, 0);
}

fn mutable_state(emu: &mut Emu, handle: u64) -> Result<&mut State, u64> {
    let id = emu
        .handle_management
        .waitable_id(handle)
        .ok_or(ERROR_INVALID_HANDLE)?;
    Ok(&mut emu
        .handle_management
        .waitable(id)
        .ok_or(ERROR_INVALID_HANDLE)?
        .state)
}

fn can_modify_timer(access: u32) -> bool {
    access & (TIMER_MODIFY_STATE | GENERIC_WRITE | GENERIC_ALL | MAXIMUM_ALLOWED) != 0
}

fn lacks_timer_access(emu: &Emu, handle: u64) -> bool {
    emu.handle_management
        .waitable_handle(handle)
        .is_some_and(|(_, access)| !can_modify_timer(access))
}

fn change(emu: &mut Emu, api: &str, handle: u64, amount: i32, previous: u64) -> Result<(), u64> {
    if previous != 0 && !emu.maps.validate_write_range(previous, 4) {
        return Err(ERROR_NOACCESS);
    }
    let tid = emu.current_thread().id;
    if api == "CancelWaitableTimer" && lacks_timer_access(emu, handle) {
        return Err(ERROR_ACCESS_DENIED);
    }
    let old = match (api, mutable_state(emu, handle)?) {
        ("SetEvent" | "ResetEvent", State::Event { signaled, .. }) => {
            let old = i32::from(*signaled);
            *signaled = api == "SetEvent";
            old
        }
        ("ReleaseMutex", State::Mutex { owner, depth }) => {
            if *owner != Some(tid) {
                return Err(ERROR_NOT_OWNER);
            }
            let old = *depth;
            *depth -= 1;
            if *depth == 0 {
                *owner = None;
            }
            old as i32
        }
        ("ReleaseSemaphore", State::Semaphore { count, limit }) => {
            if amount <= 0 {
                return Err(ERROR_INVALID_PARAMETER);
            }
            let old = *count;
            *count = count
                .checked_add(amount)
                .filter(|n| *n <= *limit)
                .ok_or(ERROR_TOO_MANY_POSTS)?;
            old
        }
        ("CancelWaitableTimer", State::Timer(timer) | State::Timer2(timer)) => {
            timer.deadline = None;
            0
        }
        _ => return Err(ERROR_INVALID_HANDLE),
    };
    if previous != 0 {
        emu.maps.write_dword(previous, old as u32);
    }
    Ok(())
}
fn set_timer(emu: &mut Emu, api: &str, extended: bool) {
    let call = WinApi(Stdcall);
    let slots = if extended { 7 } else { 6 };
    let args = api_try!(emu, arguments(call, emu, slots));
    let Some(due) = emu.maps.read_qword(args[1]).map(|v| v as i64) else {
        return return_win(emu, slots, Err(ERROR_NOACCESS), 0);
    };
    let period = args[2] as i32;
    if period < 0 {
        return return_win(emu, slots, Err(ERROR_INVALID_PARAMETER), 0);
    }
    if due > 0 || args[3] != 0 || args[5] != 0 || extended && args[6] != 0 {
        policy::unsupported(emu, "kernel32", api, emu.pc());
        return;
    }
    if lacks_timer_access(emu, args[0]) {
        return return_win(emu, slots, Err(ERROR_ACCESS_DENIED), 0);
    }
    let now = emu.tick as u64;
    let result = match mutable_state(emu, args[0]) {
        Ok(State::Timer(timer) | State::Timer2(timer)) => {
            timer.arm_relative(now, due, period as u32);
            Ok(1)
        }
        _ => Err(ERROR_INVALID_HANDLE),
    };
    return_win(emu, slots, result, 0);
}

enum Target {
    Object(u64),
    Thread(usize),
    Process,
}
enum WaitError {
    Win(u64),
    WouldBlock,
}

fn wait(emu: &mut Emu, handles: &[u64], all: bool, timeout: Option<u64>) -> Result<u64, WaitError> {
    let tid = emu.current_thread().id;
    let mut targets = Vec::with_capacity(handles.len());
    for &handle in handles {
        let target = if let Some((id, access)) = emu.handle_management.waitable_handle(handle) {
            if access & (SYNCHRONIZE | GENERIC_EXECUTE | GENERIC_ALL | MAXIMUM_ALLOWED) == 0 {
                return Err(WaitError::Win(ERROR_ACCESS_DENIED));
            }
            Target::Object(id)
        } else if let Some(index) = threads::thread_index(emu, handle) {
            Target::Thread(index)
        } else if process::query_current_process(emu, handle) {
            Target::Process
        } else {
            return Err(WaitError::Win(ERROR_INVALID_HANDLE));
        };
        if all
            && let Target::Object(id) = target
            && targets
                .iter()
                .any(|previous| matches!(previous,Target::Object(other) if *other == id))
        {
            return Err(WaitError::Win(ERROR_INVALID_PARAMETER));
        }
        targets.push(target);
    }
    let start = emu.tick as u64;
    let end = timeout.map(|t| start.saturating_add(t));
    let mut now = start;
    loop {
        let mut first = None;
        let mut count = 0;
        let mut next = None::<u64>;
        for (index, target) in targets.iter().enumerate() {
            let ready = match *target {
                Target::Process => false,
                Target::Thread(i) => emu.threads[i].exit_value.is_some(),
                Target::Object(id) => {
                    let state = &mut emu
                        .handle_management
                        .waitable(id)
                        .ok_or(WaitError::Win(ERROR_INVALID_HANDLE))?
                        .state;
                    let ready = state.ready(tid, now);
                    if !ready
                        && let State::Timer(timer) | State::Timer2(timer) = state
                        && let Some(due) = timer.deadline
                    {
                        next = Some(next.map_or(due, |v| v.min(due)));
                    }
                    ready
                }
            };
            if ready {
                first.get_or_insert(index);
                count += 1;
            }
        }
        if let Some(index) = first
            && (!all || count == targets.len())
        {
            for (i, t) in targets.iter().enumerate() {
                if (all || i == index)
                    && let Target::Object(id) = *t
                {
                    emu.handle_management
                        .waitable(id)
                        .ok_or(WaitError::Win(ERROR_INVALID_HANDLE))?
                        .state
                        .acquire(tid);
                }
            }
            emu.tick = now as usize;
            return Ok(if all {
                WAIT_OBJECT_0
            } else {
                WAIT_OBJECT_0 + index as u64
            });
        }
        if end != Some(now)
            && emu
                .threads
                .iter()
                .any(|thread| thread.id != tid && thread.exit_value.is_none())
        {
            return Err(WaitError::WouldBlock);
        }
        if let Some(due) = next
            && end.is_none_or(|end| due <= end)
        {
            now = due;
            continue;
        }
        if let Some(end) = end {
            if !emu.cfg.short_circuit_sleep {
                emu.tick = end as usize;
            }
            return Ok(WAIT_TIMEOUT);
        }
        return Err(WaitError::WouldBlock);
    }
}

fn handles(emu: &Emu, count: u32, pointer: u64) -> Result<Vec<u64>, u64> {
    if count == 0 || count as usize > MAXIMUM_WAIT_OBJECTS {
        return Err(ERROR_INVALID_PARAMETER);
    }
    let mut items = Vec::with_capacity(count as usize);
    for i in 0..count {
        let address = pointer
            .checked_add(u64::from(i) * memory::pointer_size(emu))
            .ok_or(ERROR_NOACCESS)?;
        let h = memory::read_pointer(emu, address).ok_or(ERROR_NOACCESS)?;
        if items.contains(&h) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        items.push(h);
    }
    Ok(items)
}
fn win_wait(emu: &mut Emu, library: &str, api: &str, multiple: bool, extended: bool) {
    let call = WinApi(Stdcall);
    let base = if multiple { 4 } else { 2 };
    let slots = base + usize::from(extended);
    let args = api_try!(emu, arguments(call, emu, slots));
    if extended && args[base] as u32 != 0 {
        policy::unsupported(emu, library, api, emu.pc());
        return;
    }
    let list = if multiple {
        handles(emu, args[0] as u32, args[1])
    } else {
        Ok(vec![args[0]])
    };
    let result = match list {
        Ok(values) => wait(
            emu,
            &values,
            multiple && args[2] as u32 != 0,
            match args[base - 1] as u32 {
                INFINITE => None,
                ms => Some(u64::from(ms)),
            },
        ),
        Err(e) => Err(WaitError::Win(e)),
    };
    match result {
        Ok(value) => return_win(emu, slots, Ok(value), WAIT_FAILED),
        Err(WaitError::Win(e)) => return_win(emu, slots, Err(e), WAIT_FAILED),
        Err(WaitError::WouldBlock) => policy::unsupported(emu, library, api, emu.pc()),
    }
}

pub fn WSACreateEvent(emu: &mut Emu) {
    let state = State::Event {
        manual: true,
        signaled: false,
    };
    let value = object(emu, None, Kind::Event, Some(state), u32::MAX, 0).map(|(handle, _)| handle);
    log_red!(emu, "ws2_32!WSACreateEvent");
    return_win(emu, 0, value, 0);
}

fn winsock_event(emu: &mut Emu, api: &str, operation: &str) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!{api} event={handle:#x}");
    let value = change(emu, operation, handle, 0, 0).map(|()| 1);
    return_win(emu, 1, value, 0);
}

pub fn WSASetEvent(emu: &mut Emu) {
    winsock_event(emu, "WSASetEvent", "SetEvent");
}

pub fn WSAResetEvent(emu: &mut Emu) {
    winsock_event(emu, "WSAResetEvent", "ResetEvent");
}

pub fn WSACloseEvent(emu: &mut Emu) {
    let call = WinApi(Stdcall);
    let handle = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "ws2_32!WSACloseEvent event={handle:#x}");
    let is_event = matches!(mutable_state(emu, handle), Ok(State::Event { .. }));
    let closed = is_event && emu.handle_management.close_handle(handle);
    return_win(emu, 1, closed.then_some(1).ok_or(ERROR_INVALID_HANDLE), 0);
}

pub fn WSAWaitForMultipleEvents(emu: &mut Emu) {
    log_red!(emu, "ws2_32!WSAWaitForMultipleEvents");
    win_wait(emu, "ws2_32", "WSAWaitForMultipleEvents", true, true);
}

pub(crate) fn dispatch(api: &str, library: &str, emu: &mut Emu) -> bool {
    let stem = api.strip_suffix(['A', 'W']).unwrap_or("");
    let root = stem.strip_suffix("Ex").unwrap_or(stem);
    let creation = match root {
        "CreateEvent" | "OpenEvent" => Some(Kind::Event),
        "CreateMutex" | "OpenMutex" => Some(Kind::Mutex),
        "CreateSemaphore" | "OpenSemaphore" => Some(Kind::Semaphore),
        "CreateWaitableTimer" | "OpenWaitableTimer" => Some(Kind::Timer),
        _ => None,
    };
    if let Some(kind) = creation
        && (root.starts_with("Create") || stem == root)
    {
        log_red!(emu, "{}!{}", library, api);
        open_or_create(emu, api, kind, root.starts_with("Create"), stem != root);
        return true;
    }
    let call = WinApi(Stdcall);
    match api {
        "WaitForSingleObject"
        | "WaitForSingleObjectEx"
        | "WaitForMultipleObjects"
        | "WaitForMultipleObjectsEx" => win_wait(
            emu,
            library,
            api,
            api.starts_with("WaitForMultiple"),
            api.ends_with("Ex"),
        ),
        "SetEvent" | "ResetEvent" | "ReleaseMutex" | "ReleaseSemaphore" | "CancelWaitableTimer" => {
            let slots = if api == "ReleaseSemaphore" { 3 } else { 1 };
            let args = match arguments(call, emu, slots) {
                Ok(args) => args,
                Err(e) => {
                    policy::argument_fault(emu, e);
                    return true;
                }
            };
            let result = change(
                emu,
                api,
                args[0],
                args.get(1).copied().unwrap_or_default() as i32,
                args.get(2).copied().unwrap_or_default(),
            )
            .map(|()| 1);
            return_win(emu, slots, result, 0);
        }
        "SetWaitableTimer" | "SetWaitableTimerEx" => set_timer(emu, api, api.ends_with("Ex")),
        _ => return false,
    }
    log_red!(emu, "{}!{}", library, api);
    true
}

fn timer2_manual_reset(timer_id: u64, flags: u32) -> Result<bool, u64> {
    if flags & !TIMER2_ATTRIBUTE_KNOWN_MASK != 0 {
        return Err(STATUS_INVALID_PARAMETER_4);
    }
    if flags & TIMER2_ATTRIBUTE_IR_TIMER != 0 {
        return Err(if timer_id == 0 {
            STATUS_INVALID_PARAMETER
        } else {
            STATUS_NOT_IMPLEMENTED
        });
    }
    if timer_id != 0 {
        return Err(STATUS_INVALID_PARAMETER_2);
    }
    Ok(flags & TIMER2_ATTRIBUTE_NOTIFICATION != 0)
}

fn nt_object(emu: &mut Emu, api: &str, kind: Kind, create: bool, call: WindowsAbi) {
    let timer2 = kind == Kind::Timer2;
    let slots = if !create {
        3
    } else if timer2 || matches!(kind, Kind::Event | Kind::Semaphore) {
        5
    } else {
        4
    };
    let args = api_try!(emu, arguments(call, emu, slots));
    let output = args[0];
    let attrs = args[2];
    let result = (|| {
        if output == 0 || !memory::writable_pointer(emu, output) {
            return Err(STATUS_ACCESS_VIOLATION);
        }
        let width = memory::pointer_size(emu);
        let name = if attrs == 0 {
            None
        } else {
            let size = 6 * width;
            if emu.maps.read_dword(attrs).map(u64::from) != Some(size) {
                return Err(STATUS_INVALID_PARAMETER);
            }
            if !emu
                .maps
                .can_access_range(attrs, size as usize, Permission::READ)
            {
                return Err(STATUS_ACCESS_VIOLATION);
            }
            if memory::read_pointer(emu, attrs + width) != Some(0)
                || memory::read_pointer(emu, attrs + 4 * width) != Some(0)
                || memory::read_pointer(emu, attrs + 5 * width) != Some(0)
            {
                return Err(STATUS_NOT_IMPLEMENTED);
            }
            let descriptor =
                memory::read_pointer(emu, attrs + 2 * width).ok_or(STATUS_ACCESS_VIOLATION)?;
            if descriptor == 0 {
                None
            } else {
                let length = emu
                    .maps
                    .read_word(descriptor)
                    .ok_or(STATUS_ACCESS_VIOLATION)? as usize;
                let capacity = emu
                    .maps
                    .read_word(descriptor.checked_add(2).ok_or(STATUS_ACCESS_VIOLATION)?)
                    .ok_or(STATUS_ACCESS_VIOLATION)? as usize;
                if length > capacity || !length.is_multiple_of(2) {
                    return Err(STATUS_INVALID_PARAMETER);
                }
                let ptr = memory::read_pointer(
                    emu,
                    descriptor
                        .checked_add(width)
                        .ok_or(STATUS_ACCESS_VIOLATION)?,
                )
                .ok_or(STATUS_ACCESS_VIOLATION)?;
                let mut units = Vec::with_capacity(length / 2);
                for off in (0..length).step_by(2) {
                    units.push(
                        emu.maps
                            .read_word(ptr.checked_add(off as u64).ok_or(STATUS_ACCESS_VIOLATION)?)
                            .ok_or(STATUS_ACCESS_VIOLATION)?,
                    );
                }
                let prefix: Vec<_> = r"\BaseNamedObjects\".encode_utf16().collect();
                let Some(local) = units.strip_prefix(prefix.as_slice()) else {
                    return Err(STATUS_NOT_IMPLEMENTED);
                };
                Some(local.to_owned())
            }
        };
        let initial = if !create {
            None
        } else {
            Some(match kind {
                Kind::Event | Kind::Timer if !timer2 && args[3] as u32 > 1 => {
                    return Err(STATUS_INVALID_PARAMETER);
                }
                Kind::Event => State::Event {
                    manual: args[3] == 0,
                    signaled: args[4] as u8 != 0,
                },
                Kind::Timer => State::Timer(Timer::new(args[3] as u32 == 0)),
                Kind::Timer2 => {
                    State::Timer2(Timer::new(timer2_manual_reset(args[1], args[3] as u32)?))
                }
                Kind::Mutex => {
                    let owned = args[3] as u8 != 0;
                    State::Mutex {
                        owner: owned.then_some(emu.current_thread().id),
                        depth: u32::from(owned),
                    }
                }
                Kind::Semaphore => {
                    let (count, limit) = (args[3] as i32, args[4] as i32);
                    if count < 0 || count > limit || limit <= 0 {
                        return Err(STATUS_INVALID_PARAMETER);
                    }
                    State::Semaphore { count, limit }
                }
            })
        };
        let existed = name
            .as_ref()
            .is_some_and(|n| emu.handle_management.named_waitable(n).is_some());
        let attributes = if attrs == 0 {
            0
        } else {
            emu.maps
                .read_dword(attrs + 3 * width)
                .ok_or(STATUS_ACCESS_VIOLATION)?
        };
        if attributes & !(OBJ_INHERIT | OBJ_OPENIF | OBJ_CASE_INSENSITIVE) != 0 {
            return Err(STATUS_NOT_IMPLEMENTED);
        }
        if create && existed && attrs != 0 && attributes & OBJ_OPENIF == 0 {
            return Err(STATUS_OBJECT_NAME_COLLISION);
        }
        let access = if timer2 {
            args[4] as u32
        } else {
            args[1] as u32
        };
        let (handle, already) = object(emu, name, kind, initial, access, attributes & OBJ_INHERIT)
            .map_err(|e| match e {
                ERROR_FILE_NOT_FOUND => STATUS_OBJECT_NAME_NOT_FOUND,
                ERROR_INVALID_HANDLE => STATUS_OBJECT_TYPE_MISMATCH,
                _ => STATUS_NO_MEMORY,
            })?;
        memory::write_pointer(emu, output, handle);
        Ok(if create && already {
            STATUS_OBJECT_NAME_EXISTS
        } else {
            STATUS_SUCCESS
        })
    })();
    let status = result.unwrap_or_else(|error| error);
    if status == STATUS_NOT_IMPLEMENTED {
        policy::unsupported_with_abi(emu, "ntdll", api, emu.pc(), call);
        return;
    }
    call.set_return(emu, status);
    call.cleanup_args(emu, slots);
}

fn set_timer2(emu: &mut Emu, api: &str, call: WindowsAbi) {
    let [handle, due_time, period, parameters] = api_try!(emu, call.args(emu));
    let result = (|| {
        if due_time == 0 {
            return Err(STATUS_INVALID_PARAMETER_2);
        }
        let due = emu
            .maps
            .read_qword(due_time)
            .ok_or(STATUS_ACCESS_VIOLATION)? as i64;
        if parameters != 0 {
            // T2_SET_PARAMETERS_V0 contains Version, Reserved, and signed NoWakeTolerance.
            if !emu.maps.can_access_range(parameters, 16, Permission::READ) {
                return Err(STATUS_ACCESS_VIOLATION);
            }
            let version = emu
                .maps
                .read_dword(parameters)
                .ok_or(STATUS_ACCESS_VIOLATION)?;
            let tolerance = emu
                .maps
                .read_qword(parameters + 8)
                .ok_or(STATUS_ACCESS_VIOLATION)? as i64;
            if version != 0 || tolerance < -1 {
                return Err(STATUS_INVALID_PARAMETER_3);
            }
        }
        let interval = if period == 0 {
            0
        } else {
            emu.maps.read_qword(period).ok_or(STATUS_ACCESS_VIOLATION)?
        };
        if due > 0 || interval != 0 {
            return Err(STATUS_NOT_IMPLEMENTED);
        }
        let (id, access) = emu
            .handle_management
            .waitable_handle(handle)
            .ok_or_else(|| {
                if emu.handle_management.contains_handle(handle) {
                    STATUS_OBJECT_TYPE_MISMATCH
                } else {
                    STATUS_INVALID_HANDLE
                }
            })?;
        let now = emu.tick as u64;
        let Some(WaitObject {
            state: State::Timer2(timer),
            ..
        }) = emu.handle_management.waitable(id)
        else {
            return Err(STATUS_OBJECT_TYPE_MISMATCH);
        };
        if !can_modify_timer(access) {
            return Err(STATUS_ACCESS_DENIED);
        }
        // Guest time advances while the CPU is awake, so the earliest permitted expiry is valid.
        timer.arm_relative(now, due, 0);
        Ok(())
    })();
    if result == Err(STATUS_NOT_IMPLEMENTED) {
        return policy::unsupported_with_abi(emu, "ntdll", api, emu.pc(), call);
    }
    let status = result.err().unwrap_or(STATUS_SUCCESS);
    log_red!(
        emu,
        "ntdll!{api} timer={handle:#x} due_time={due_time:#x} period={period:#x} parameters={parameters:#x} => {status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 4);
}

fn nt_operation(emu: &mut Emu, api: &str, op: &str, call: WindowsAbi) {
    let slots = match op {
        "Close" | "Clear" => 1,
        "Wait" | "ReleaseSemaphore" => 3,
        "WaitMultiple" => 5,
        _ => 2,
    };
    let args = api_try!(emu, arguments(call, emu, slots));
    let result = match op {
        "Close" => {
            emu.handle_management.close_handle(args[0]);
            Ok(STATUS_SUCCESS)
        }
        "Wait" | "WaitMultiple" => {
            let multiple = op == "WaitMultiple";
            let (alertable, time) = if multiple {
                (args[3], args[4])
            } else {
                (args[1], args[2])
            };
            if alertable as u8 != 0 {
                Err(WaitError::WouldBlock)
            } else if multiple && args[2] as u32 > 1 {
                Err(WaitError::Win(ERROR_INVALID_PARAMETER))
            } else {
                let timeout = if time == 0 {
                    Ok(None)
                } else {
                    match emu.maps.read_qword(time).map(|v| v as i64) {
                        Some(v) if v <= 0 => Ok(Some(v.unsigned_abs().div_ceil(HUNDRED_NS_PER_MS))),
                        Some(_) => Err(WaitError::WouldBlock),
                        None => Err(WaitError::Win(ERROR_NOACCESS)),
                    }
                };
                timeout.and_then(|timeout| {
                    let list = if multiple {
                        handles(emu, args[0] as u32, args[1]).map_err(WaitError::Win)?
                    } else {
                        vec![args[0]]
                    };
                    wait(emu, &list, multiple && args[2] as u32 == 0, timeout)
                })
            }
        }
        _ => {
            let (operation, amount, previous) = match op {
                "Clear" => ("ResetEvent", 0, 0),
                "ReleaseSemaphore" => (op, args[1] as i32, args[2]),
                _ => (op, 0, args[1]),
            };
            change(emu, operation, args[0], amount, previous)
                .map(|()| STATUS_SUCCESS)
                .map_err(WaitError::Win)
        }
    };
    let status = match result {
        Ok(value) => value,
        Err(WaitError::WouldBlock) => {
            policy::unsupported_with_abi(emu, "ntdll", api, emu.pc(), call);
            return;
        }
        Err(WaitError::Win(error)) => match error {
            ERROR_ACCESS_DENIED => STATUS_ACCESS_DENIED,
            ERROR_NOACCESS => STATUS_ACCESS_VIOLATION,
            ERROR_NOT_OWNER => STATUS_MUTANT_NOT_OWNED,
            ERROR_TOO_MANY_POSTS => STATUS_SEMAPHORE_LIMIT_EXCEEDED,
            ERROR_INVALID_PARAMETER => STATUS_INVALID_PARAMETER,
            _ => STATUS_INVALID_HANDLE,
        },
    };
    call.set_return(emu, status);
    call.cleanup_args(emu, slots);
}

fn create_completion_stub(
    emu: &mut Emu,
    call: WindowsAbi,
    api: &str,
    object: NativeObject,
    slots: usize,
) {
    let output = api_try!(emu, call.arg(emu, 0));
    let attributes = api_try!(emu, call.arg(emu, 2));
    if attributes != 0 {
        return policy::unsupported_with_abi(emu, "ntdll", api, emu.pc(), call);
    }
    let status = if memory::writable_pointer(emu, output) {
        let handle = emu.handle_management.insert_native_object(object);
        memory::write_pointer(emu, output, handle);
        STATUS_SUCCESS
    } else {
        STATUS_ACCESS_VIOLATION
    };
    log_red!(emu, "ntdll!{api} (stub) output={output:#x} => {status:#x}");
    call.set_return(emu, status);
    call.cleanup_args(emu, slots);
}

// Bootstrap creates these associations without consuming completion records.
fn associate_wait_completion_stub(emu: &mut Emu, call: WindowsAbi, api: &str) {
    let [
        packet,
        port,
        target,
        _key,
        _context,
        _io_status,
        _information,
        already_signaled,
    ] = api_try!(emu, call.args(emu));
    let tid = emu.current_thread().id;
    let now = emu.tick as u64;
    let status = if emu.handle_management.native_object(packet)
        != Some(NativeObject::WaitCompletionPacket)
        || emu.handle_management.native_object(port) != Some(NativeObject::IoCompletion)
    {
        STATUS_INVALID_HANDLE
    } else if already_signaled != 0 && !emu.maps.validate_write_range(already_signaled, 1) {
        STATUS_ACCESS_VIOLATION
    } else {
        match mutable_state(emu, target) {
            Ok(state) => {
                let ready = state.ready(tid, now);
                if already_signaled != 0 {
                    emu.maps.write_byte(already_signaled, ready as u8);
                }
                STATUS_SUCCESS
            }
            Err(_) => STATUS_INVALID_HANDLE,
        }
    };
    log_red!(
        emu,
        "ntdll!{api} (stub) packet={packet:#x} port={port:#x} target={target:#x} => {status:#x}"
    );
    call.set_return(emu, status);
    call.cleanup_args(emu, 8);
}

pub(crate) fn is_notification_event(emu: &mut Emu, handle: u64) -> bool {
    matches!(mutable_state(emu, handle), Ok(State::Event { .. }))
}

fn register_wnf_event_stub(emu: &mut Emu, call: WindowsAbi, api: &str) {
    let event = api_try!(emu, call.arg(emu, 0));
    let status = match mutable_state(emu, event) {
        Ok(State::Event { .. }) => STATUS_SUCCESS,
        Ok(_) => STATUS_OBJECT_TYPE_MISMATCH,
        Err(_) => STATUS_INVALID_HANDLE,
    };
    log_red!(emu, "ntdll!{api} (stub) event={event:#x} => {status:#x}");
    call.set_return(emu, status);
    call.cleanup_args(emu, 1);
}

pub(crate) fn dispatch_nt(api: &str, emu: &mut Emu, call: WindowsAbi) -> bool {
    let normalized = nt_api::normalize(api);
    let name = normalized.as_ref();
    match name {
        "NtCreateIoCompletion" => {
            create_completion_stub(emu, call, api, NativeObject::IoCompletion, 4)
        }
        "NtCreateWaitCompletionPacket" => {
            create_completion_stub(emu, call, api, NativeObject::WaitCompletionPacket, 3)
        }
        "NtSetTimer2" => set_timer2(emu, api, call),
        "NtAssociateWaitCompletionPacket" => associate_wait_completion_stub(emu, call, api),
        "NtSetWnfProcessNotificationEvent" => register_wnf_event_stub(emu, call, api),
        _ => return dispatch_wait(emu, call, api, name),
    }
    true
}

fn dispatch_wait(emu: &mut Emu, call: WindowsAbi, api: &str, name: &str) -> bool {
    let native = match name {
        "NtCreateEvent" | "NtOpenEvent" => Some(Kind::Event),
        "NtCreateMutant" | "NtOpenMutant" => Some(Kind::Mutex),
        "NtCreateSemaphore" | "NtOpenSemaphore" => Some(Kind::Semaphore),
        "NtCreateTimer" | "NtOpenTimer" => Some(Kind::Timer),
        "NtCreateTimer2" => Some(Kind::Timer2),
        _ => None,
    };
    if let Some(kind) = native {
        nt_object(emu, api, kind, name.starts_with("NtCreate"), call);
        return true;
    }
    let op = match name {
        "NtSetEvent" => "SetEvent",
        "NtResetEvent" => "ResetEvent",
        "NtClearEvent" => "Clear",
        "NtWaitForMultipleObjects" => "WaitMultiple",
        "NtReleaseSemaphore" => "ReleaseSemaphore",
        "NtReleaseMutant" => "ReleaseMutex",
        "NtWaitForSingleObject" => "Wait",
        "NtClose"
            if call.arg(emu, 0).is_ok_and(|h| {
                emu.handle_management.waitable_id(h).is_some()
                    || emu.handle_management.process_token_rights(h).is_some()
            }) =>
        {
            "Close"
        }
        _ => return false,
    };
    nt_operation(emu, api, op, call);
    true
}

macro_rules! public_wait_handlers {
    () => { $crate::winapi::common::wait::public_wait_handlers!(CancelWaitableTimer, CreateEventA, CreateEventExA, CreateEventExW, CreateEventW, CreateMutexA, CreateMutexExA, CreateMutexExW, CreateMutexW, CreateSemaphoreA, CreateSemaphoreExA, CreateSemaphoreExW, CreateSemaphoreW, CreateWaitableTimerA, CreateWaitableTimerExA, CreateWaitableTimerExW, CreateWaitableTimerW, OpenEventA, OpenEventW, OpenMutexA, OpenMutexW, OpenSemaphoreA, OpenSemaphoreW, OpenWaitableTimerA, OpenWaitableTimerW, ReleaseMutex, ReleaseSemaphore, ResetEvent, SetEvent, SetWaitableTimer, SetWaitableTimerEx, WaitForMultipleObjects, WaitForMultipleObjectsEx, WaitForSingleObject, WaitForSingleObjectEx); };
    ($($name:ident),+ $(,)?) => {$(
        pub fn $name(emu: &mut $crate::emu::Emu) {
            $crate::winapi::common::wait::dispatch(stringify!($name), "kernel32", emu);
        }
    )+};
}
pub(crate) use public_wait_handlers;

pub(crate) fn address_wait(emu: &mut Emu, library: &str, api: &str) {
    let call = WinApi(Stdcall);
    let address = api_try!(emu, call.arg(emu, 0));
    log_red!(emu, "{api} address={address:#x}");
    if api != "WaitOnAddress" {
        // Blocking waits enter policy, so there are no registered waiters to wake.
        call.cleanup_args(emu, 1);
        return;
    }
    let compare = api_try!(emu, call.arg(emu, 1));
    let size = api_try!(emu, call.arg(emu, 2));
    let timeout = api_try!(emu, call.arg_u32(emu, 3));
    let result = (|| {
        if !matches!(size, 1 | 2 | 4 | 8) {
            return Err(ERROR_INVALID_PARAMETER);
        }
        let get = |ptr: u64| {
            if ptr == 0 {
                None
            } else {
                memory::read_guest_bytes(emu, ptr, size as usize)
            }
        };
        let expected = get(compare).ok_or(ERROR_NOACCESS)?;
        let actual = get(address).ok_or(ERROR_NOACCESS)?;
        if actual != expected {
            return Ok(Some(()));
        }
        let id = emu.threads[emu.current_thread_id].id;
        if timeout == INFINITE
            || timeout != 0
                && emu
                    .threads
                    .iter()
                    .any(|t| t.id != id && t.exit_value.is_none())
        {
            return Ok(None);
        }
        if !emu.cfg.short_circuit_sleep {
            emu.tick = emu.tick.saturating_add(timeout as usize);
        }
        Err(ERROR_TIMEOUT)
    })();
    let value = match result {
        Ok(Some(())) => 1,
        Ok(None) => {
            policy::unsupported(emu, library, api, emu.pc());
            return;
        }
        Err(error) => {
            emu.set_last_error(error);
            0
        }
    };
    call.set_return(emu, value);
    call.cleanup_args(emu, 4);
}
