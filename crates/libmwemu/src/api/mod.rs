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
        Ok(())
    }
}
