use crate::api::windows::policy::ApiDisposition;
use crate::emu::Emu;
use crate::err::MwemuError;

pub mod abi;
pub mod banzai;
pub mod linux;
pub mod macos;
pub mod printf;
pub mod strconv;
pub mod unix;
pub mod windows;

// The caller owns completion and fallback; matching never changes API policy.
macro_rules! dispatch_api {
    (
        $name:expr,
        { $($pattern:pat $(if $guard:expr)? => $handler:expr,)* },
        handled($result:pat) => $handled:expr,
        fallback => $fallback:expr $(,)?
    ) => {
        match $name {
            $($pattern $(if $guard)? => {
                let $result = $handler;
                $handled
            },)*
            _ => $fallback,
        }
    };
    (
        $name:expr, $emu:expr, registry $registry:expr,
        handled($result:pat) => $handled:expr,
        fallback => $fallback:expr $(,)?
    ) => {{
        let name = $name;
        match $registry.find(name) {
            Some(handler) => {
                let $result = handler($emu, name);
                $handled
            }
            None => $fallback,
        }
    }};
}

pub(crate) use dispatch_api;

impl Emu {
    pub(crate) fn api_stopped(&self) -> bool {
        self.api_policy.stopped() || self.unix_api.stopped()
    }

    pub(crate) fn check_api_stop(&self) -> Result<(), MwemuError> {
        if let Some(outcome) = &self.api_policy.last_outcome
            && let ApiDisposition::Stopped(reason) = outcome.disposition
        {
            return Err(MwemuError::new(&format!(
                "Windows API call stopped at {}!{} at 0x{:x}: {:?}",
                outcome.call.library, outcome.call.name, outcome.call.pc, reason
            )));
        }
        if let Some(fault) = &self.unix_api.fault {
            return Err(MwemuError::new(&format!(
                "Unix API call stopped at {}!{} at 0x{:x}: {:?}",
                fault.call.library, fault.call.name, fault.call.pc, fault.reason
            )));
        }
        if let Some(fault) = &self.unix_api.memory_fault {
            return Err(MwemuError::new(&format!(
                "Unix API memory fault at {}!{} at 0x{:x}: {:?}",
                fault.call.library, fault.call.name, fault.call.pc, fault.error
            )));
        }
        Ok(())
    }
}
