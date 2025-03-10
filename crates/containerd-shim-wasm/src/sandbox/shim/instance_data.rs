use std::sync::{OnceLock, RwLock};
use std::time::Duration;

use chrono::{DateTime, Utc};

use crate::sandbox::shim::task_state::TaskState;
use crate::sandbox::{Instance, InstanceConfig, Result};

pub(super) struct InstanceData<T: Instance> {
    pub instance: T,
    cfg: InstanceConfig,
    pid: OnceLock<u32>,
    state: RwLock<TaskState>,
}

impl<T: Instance> InstanceData<T> {
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "Debug"))]
    pub fn new(id: impl AsRef<str> + std::fmt::Debug, cfg: InstanceConfig) -> Result<Self> {
        let id = id.as_ref().to_string();
        let instance = T::new(id, &cfg)?;
        Ok(Self {
            instance,
            cfg,
            pid: OnceLock::default(),
            state: RwLock::new(TaskState::Created),
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn pid(&self) -> Option<u32> {
        self.pid.get().copied()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn config(&self) -> &InstanceConfig {
        &self.cfg
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn start(&self) -> Result<u32> {
        let mut s = self.state.write().unwrap();
        s.start()?;

        let res = self.instance.start();

        // These state transitions are always `Ok(())` because
        // we hold the lock since `s.start()`
        let _ = match res {
            Ok(pid) => {
                let _ = self.pid.set(pid);
                s.started()
            }
            Err(_) => s.stop(),
        };

        res
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn kill(&self, signal: u32) -> Result<()> {
        let mut s = self.state.write().unwrap();
        s.kill()?;

        self.instance.kill(signal)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn delete(&self) -> Result<()> {
        let mut s = self.state.write().unwrap();
        s.delete()?;

        let res = self.instance.delete();

        if res.is_err() {
            // Always `Ok(())` because we hold the lock since `s.delete()`
            let _ = s.stop();
        }

        res
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn wait(&self) -> (u32, DateTime<Utc>) {
        let res = self.instance.wait();
        let mut s = self.state.write().unwrap();
        *s = TaskState::Exited;
        res
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self), level = "Debug"))]
    pub fn wait_timeout(
        &self,
        t: impl Into<Option<Duration>> + std::fmt::Debug,
    ) -> Option<(u32, DateTime<Utc>)> {
        let res = self.instance.wait_timeout(t);
        if res.is_some() {
            let mut s = self.state.write().unwrap();
            *s = TaskState::Exited;
        }
        res
    }
}
