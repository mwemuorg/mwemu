use crate::emu;
use crate::emu::decoded_instruction::DecodedInstruction;
use crate::exception::types;

// Hook types using Box<dyn FnMut> for state capture instead of bare fn pointers.
// return: false will ignore interrupt handling like 0x80 -> linux
pub type TypeHookOnInterrupt = Box<dyn FnMut(&mut emu::Emu, u64, u64) -> bool>;
// return: allow handle exception?
pub type TypeHookOnException = Box<dyn FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool>;
// memory read is pre-read you can modify the value that is going to be read.
pub type TypeHookOnMemoryRead = Box<dyn FnMut(&mut emu::Emu, u64, u64, u32)>;
// the memory write is pre but you can change the value is going to be written.
pub type TypeHookOnMemoryWrite = Box<dyn FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128>;

// [BREAKING API CHANGE] returning false will skip the handling of the instruction
// Changed from &iced_x86::Instruction to &DecodedInstruction for arch parity.
// x86 hook users call ins.as_x86() to get the familiar type.
pub type TypeHookOnPreInstruction =
    Box<dyn FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool>;

pub type TypeHookOnPostInstruction =
    Box<dyn FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool)>;
pub type TypeHookOnWinApiCall = Box<dyn FnMut(&mut emu::Emu, u64, u64) -> bool>;
// (emu, syscall_nr): fires on every syscall/SVC across all OSes, before the
// internal dispatch. `syscall_nr` is the raw number the program invoked (rax /
// x8 / x16 / eax depending on arch); on Windows you can map it to the canonical
// number via `emu.syscall_number_map`. Return false if the hook fully handled
// the syscall (it set the return register itself) to skip the default handler.
pub type TypeHookOnSyscall = Box<dyn FnMut(&mut emu::Emu, u64) -> bool>;

pub struct Hooks {
    pub hook_on_interrupt: Option<TypeHookOnInterrupt>,
    pub hook_on_exception: Option<TypeHookOnException>,
    pub hook_on_memory_read: Option<TypeHookOnMemoryRead>,
    pub hook_on_memory_write: Option<TypeHookOnMemoryWrite>,
    pub hook_on_pre_instruction: Option<TypeHookOnPreInstruction>,
    pub hook_on_post_instruction: Option<TypeHookOnPostInstruction>,
    pub hook_on_winapi_call: Option<TypeHookOnWinApiCall>,
    pub hook_on_syscall: Option<TypeHookOnSyscall>,
}

impl Default for Hooks {
    fn default() -> Self {
        Self::new()
    }
}

impl Hooks {
    pub fn new() -> Hooks {
        Hooks {
            hook_on_interrupt: None,
            hook_on_exception: None,
            hook_on_memory_read: None,
            hook_on_memory_write: None,
            hook_on_pre_instruction: None,
            hook_on_post_instruction: None,
            hook_on_winapi_call: None,
            hook_on_syscall: None,
        }
    }

    pub fn on_interrupt(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static) {
        self.hook_on_interrupt = Some(Box::new(hook));
    }

    pub fn disable_interrupt(&mut self) {
        self.hook_on_interrupt = None;
    }

    pub fn on_exception(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, types::ExceptionType) -> bool + 'static,
    ) {
        self.hook_on_exception = Some(Box::new(hook));
    }

    pub fn disable_exception(&mut self) {
        self.hook_on_exception = None;
    }

    pub fn on_memory_read(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64, u32) + 'static) {
        self.hook_on_memory_read = Some(Box::new(hook));
    }

    pub fn disable_memory_read(&mut self) {
        self.hook_on_memory_read = None;
    }

    pub fn on_memory_write(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, u64, u32, u128) -> u128 + 'static,
    ) {
        self.hook_on_memory_write = Some(Box::new(hook));
    }

    pub fn disable_memory_write(&mut self) {
        self.hook_on_memory_write = None;
    }

    pub fn on_pre_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize) -> bool + 'static,
    ) {
        self.hook_on_pre_instruction = Some(Box::new(hook));
    }

    pub fn disable_pre_instruction(&mut self) {
        self.hook_on_pre_instruction = None;
    }

    pub fn on_post_instruction(
        &mut self,
        hook: impl FnMut(&mut emu::Emu, u64, &DecodedInstruction, usize, bool) + 'static,
    ) {
        self.hook_on_post_instruction = Some(Box::new(hook));
    }

    pub fn disable_post_instruction(&mut self) {
        self.hook_on_post_instruction = None;
    }

    pub fn on_winapi_call(&mut self, hook: impl FnMut(&mut emu::Emu, u64, u64) -> bool + 'static) {
        self.hook_on_winapi_call = Some(Box::new(hook));
    }

    pub fn disable_winapi_call(&mut self) {
        self.hook_on_winapi_call = None;
    }

    pub fn on_syscall(&mut self, hook: impl FnMut(&mut emu::Emu, u64) -> bool + 'static) {
        self.hook_on_syscall = Some(Box::new(hook));
    }

    pub fn disable_syscall(&mut self) {
        self.hook_on_syscall = None;
    }
}

impl emu::Emu {
    /// Run the installed `on_syscall` hook (if any) for `nr`. Returns `false`
    /// when the hook reports it fully handled the syscall and the default
    /// dispatch should be skipped. Uses take/restore so the hook can borrow the
    /// emulator mutably while it is stored inside it.
    pub fn call_syscall_hook(&mut self, nr: u64) -> bool {
        if let Some(mut hook_fn) = self.hooks.hook_on_syscall.take() {
            let proceed = hook_fn(self, nr);
            self.hooks.hook_on_syscall = Some(hook_fn);
            proceed
        } else {
            true
        }
    }
}
