use crate::windows::constants::errors::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(super) enum Kind {
    Anonymous,
    Named,
    Mailslot,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub(super) struct Endpoint {
    pub channel: u64,
    pub server: bool,
    pub read: bool,
    pub write: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub(super) struct Channel {
    pub kind: Kind,
    pub name: Option<String>,
    pub mode: u32,
    pub inbound: VecDeque<u8>,
    pub outbound: VecDeque<u8>,
    pub messages: VecDeque<Vec<u8>>,
    pub max_message: u32,
    pub timeout: u32,
    pub server_open: bool,
    pub clients: usize,
    pub was_connected: bool,
    queued: usize,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct IpcState {
    last_id: u64,
    pub(super) endpoints: BTreeMap<u32, Endpoint>,
    pub(super) channels: BTreeMap<u64, Channel>,
}

impl IpcState {
    pub(super) fn create(
        &mut self,
        kind: Kind,
        name: Option<String>,
        mode: u32,
        max_message: u32,
        timeout: u32,
    ) -> Result<u64, u64> {
        if name.as_ref().is_some_and(|s| self.named(s).is_some()) {
            return Err(if kind == Kind::Mailslot {
                ERROR_ALREADY_EXISTS
            } else {
                ERROR_PIPE_BUSY
            });
        }
        let id = self.last_id.checked_add(1).ok_or(ERROR_NOT_ENOUGH_MEMORY)?;
        self.last_id = id;
        self.channels.insert(
            id,
            Channel {
                kind,
                name,
                mode,
                max_message,
                timeout,
                inbound: Default::default(),
                outbound: Default::default(),
                messages: Default::default(),
                server_open: true,
                clients: 0,
                was_connected: false,
                queued: 0,
            },
        );
        Ok(id)
    }

    pub(super) fn named(&self, name: &str) -> Option<u64> {
        self.channels
            .iter()
            .find(|(_, c)| c.server_open && c.name.as_deref() == Some(name))
            .map(|(id, _)| *id)
    }

    pub(crate) fn contains(&self, handle: u64) -> bool {
        self.endpoint(handle).is_some()
    }

    pub(super) fn endpoint(&self, handle: u64) -> Option<Endpoint> {
        self.endpoints.get(&u32::try_from(handle).ok()?).copied()
    }

    pub(super) fn close(&mut self, handle: u64) -> bool {
        let Ok(key) = u32::try_from(handle) else {
            return false;
        };
        let Some(end) = self.endpoints.remove(&key) else {
            return false;
        };
        if let Some(c) = self.channels.get_mut(&end.channel) {
            if end.server {
                c.server_open = false;
            } else {
                c.clients -= 1;
            }
            if !c.server_open && c.clients == 0 {
                self.channels.remove(&end.channel);
            }
        }
        true
    }

    pub(super) fn disconnect(&mut self, handle: u64) -> Result<(), u64> {
        let end = self.endpoint(handle).ok_or(ERROR_INVALID_HANDLE)?;
        let c = self
            .channels
            .get_mut(&end.channel)
            .ok_or(ERROR_PIPE_NOT_CONNECTED)?;
        if !end.server || c.kind != Kind::Named {
            return Err(ERROR_INVALID_HANDLE);
        }
        if !c.was_connected {
            return Err(ERROR_PIPE_NOT_CONNECTED);
        }
        // Detached client handles stay valid for CloseHandle alone.
        for client in self
            .endpoints
            .values_mut()
            .filter(|e| e.channel == end.channel && !e.server)
        {
            client.channel = 0;
        }
        c.inbound.clear();
        c.outbound.clear();
        c.queued = 0;
        c.clients = 0;
        c.was_connected = false;
        Ok(())
    }

    pub(super) fn write(
        &mut self,
        end: Endpoint,
        bytes: &[u8],
        limit: usize,
    ) -> Result<usize, u64> {
        let c = self
            .channels
            .get_mut(&end.channel)
            .ok_or(ERROR_PIPE_NOT_CONNECTED)?;
        if !end.write {
            return Err(ERROR_ACCESS_DENIED);
        }
        if !c.server_open || (end.server && c.clients == 0) {
            return Err(ERROR_BROKEN_PIPE);
        }
        if c.queued.saturating_add(bytes.len()) > limit {
            return Err(ERROR_NOT_ENOUGH_MEMORY);
        }
        if c.kind == Kind::Mailslot {
            if c.max_message != 0 && bytes.len() > c.max_message as usize {
                return Err(ERROR_BAD_LENGTH);
            }
            if c.messages.len() >= limit / size_of::<Vec<u8>>().max(1) {
                return Err(ERROR_NOT_ENOUGH_MEMORY);
            }
            c.messages.push_back(bytes.to_vec());
        } else if end.server {
            c.outbound.extend(bytes);
        } else {
            c.inbound.extend(bytes);
        }
        c.queued += bytes.len();
        Ok(bytes.len())
    }

    pub(super) fn read(&mut self, end: Endpoint, size: usize) -> Result<Vec<u8>, u64> {
        let c = self
            .channels
            .get_mut(&end.channel)
            .ok_or(ERROR_PIPE_NOT_CONNECTED)?;
        if !end.read {
            return Err(ERROR_ACCESS_DENIED);
        }
        let result = if c.kind == Kind::Mailslot {
            let msg = c.messages.front().ok_or(if c.timeout == 0 {
                ERROR_SEM_TIMEOUT
            } else {
                ERROR_NOT_SUPPORTED
            })?;
            if msg.len() > size {
                return Err(ERROR_INSUFFICIENT_BUFFER);
            }
            c.messages.pop_front().ok_or(ERROR_SEM_TIMEOUT)?
        } else {
            let queue = if end.server {
                &mut c.inbound
            } else {
                &mut c.outbound
            };
            if queue.is_empty() && size != 0 {
                return Err(if !c.server_open || c.was_connected && c.clients == 0 {
                    ERROR_BROKEN_PIPE
                } else {
                    ERROR_NOT_SUPPORTED
                });
            }
            queue.drain(..size.min(queue.len())).collect()
        };
        c.queued -= result.len();
        Ok(result)
    }
}
