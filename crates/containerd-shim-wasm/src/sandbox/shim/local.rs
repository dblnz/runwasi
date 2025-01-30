use std::collections::HashMap;
use std::fs::create_dir_all;
use std::ops::Not;
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use anyhow::Context as AnyhowContext;
use containerd_shim::api::{
    ConnectRequest, ConnectResponse, CreateTaskRequest, CreateTaskResponse, DeleteRequest, Empty,
    KillRequest, ShutdownRequest, StartRequest, StartResponse, StateRequest, StateResponse,
    StatsRequest, StatsResponse, WaitRequest, WaitResponse,
};
use containerd_shim::error::Error as ShimError;
use containerd_shim::protos::events::task::{TaskCreate, TaskDelete, TaskExit, TaskIO, TaskStart};
use containerd_shim::protos::shim::shim_ttrpc::Task;
use containerd_shim::protos::types::task::Status;
use containerd_shim::util::IntoOption;
use containerd_shim::{DeleteResponse, ExitSignal, TtrpcContext, TtrpcResult};
use log::debug;
use oci_spec::runtime::Spec;
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use super::otel::extract_context;
use crate::sandbox::instance::{Instance, InstanceConfig};
use crate::sandbox::shim::events::{EventSender, RemoteEventSender, ToTimestamp};
use crate::sandbox::shim::instance_data::InstanceData;
use crate::sandbox::{oci, Error, Result};
use crate::sys::metrics::get_metrics;

#[cfg(test)]
mod tests;

type LocalInstances<T> = RwLock<HashMap<String, Arc<InstanceData<T>>>>;

/// Local implements the Task service for a containerd shim.
/// It defers all task operations to the `Instance` implementation.
pub struct Local<T: Instance + Send + Sync, E: EventSender = RemoteEventSender> {
    pub engine: T::Engine,
    pub(super) instances: LocalInstances<T>,
    events: E,
    exit: Arc<ExitSignal>,
    namespace: String,
    containerd_address: String,
}

impl<T: Instance + Send + Sync, E: EventSender> Local<T, E> {
    /// Creates a new local task service.
    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    pub fn new(
        engine: T::Engine,
        events: E,
        exit: Arc<ExitSignal>,
        namespace: impl AsRef<str>,
        containerd_address: impl AsRef<str>,
    ) -> Self {
        let instances = RwLock::default();
        let namespace = namespace.as_ref().to_string();
        let containerd_address = containerd_address.as_ref().to_string();
        Self {
            engine,
            instances,
            events,
            exit,
            namespace,
            containerd_address,
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    pub(super) fn get_instance(&self, id: &str) -> Result<Arc<InstanceData<T>>> {
        let instance = self.instances.read().unwrap().get(id).cloned();
        instance.ok_or_else(|| Error::NotFound(id.to_string()))
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn has_instance(&self, id: &str) -> bool {
        self.instances.read().unwrap().contains_key(id)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn is_empty(&self) -> bool {
        self.instances.read().unwrap().is_empty()
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn instance_config(&self) -> InstanceConfig {
        InstanceConfig::new(&self.namespace, &self.containerd_address)
    }
}

// These are the same functions as in Task, but without the TtrcpContext, which is useful for testing
impl<T: Instance + Send + Sync, E: EventSender> Local<T, E> {
    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_create(&self, req: CreateTaskRequest) -> Result<CreateTaskResponse> {
        if !req.checkpoint().is_empty() || !req.parent_checkpoint().is_empty() {
            return Err(ShimError::Unimplemented("checkpoint is not supported".to_string()).into());
        }

        if req.terminal {
            return Err(Error::InvalidArgument(
                "terminal is not supported".to_string(),
            ));
        }

        if self.has_instance(&req.id) {
            return Err(Error::AlreadyExists(req.id));
        }

        let mut spec = Spec::load(Path::new(&req.bundle).join("config.json"))
            .map_err(|err| Error::InvalidArgument(format!("could not load runtime spec: {err}")))?;

        spec.canonicalize_rootfs(req.bundle()).map_err(|err| {
            ShimError::InvalidArgument(format!("could not canonicalize rootfs: {}", err))
        })?;

        let rootfs = spec
            .root()
            .as_ref()
            .ok_or_else(|| Error::InvalidArgument("rootfs is not set in runtime spec".to_string()))?
            .path();

        let _ = create_dir_all(rootfs);
        let rootfs_mounts = req.rootfs().to_vec();
        if !rootfs_mounts.is_empty() {
            for m in rootfs_mounts {
                let mount_type = m.type_().none_if(|&x| x.is_empty());
                let source = m.source.as_str().none_if(|&x| x.is_empty());

                #[cfg(unix)]
                containerd_shim::mount::mount_rootfs(
                    mount_type,
                    source,
                    &m.options.to_vec(),
                    rootfs,
                )?;
            }
        }

        let mut cfg = self.instance_config();
        cfg.set_bundle(&req.bundle)
            .set_stdin(&req.stdin)
            .set_stdout(&req.stdout)
            .set_stderr(&req.stderr);

        // Check if this is a cri container
        let instance = InstanceData::new(req.id(), cfg)?;

        self.instances
            .write()
            .unwrap()
            .insert(req.id().to_string(), Arc::new(instance));

        self.events.send(TaskCreate {
            container_id: req.id,
            bundle: req.bundle,
            rootfs: req.rootfs,
            io: Some(TaskIO {
                stdin: req.stdin,
                stdout: req.stdout,
                stderr: req.stderr,
                ..Default::default()
            })
            .into(),
            ..Default::default()
        });

        debug!("create done");

        // Per the spec, the prestart hook must be called as part of the create operation
        debug!("call prehook before the start");
        oci::setup_prestart_hooks(spec.hooks())?;

        Ok(CreateTaskResponse {
            pid: std::process::id(),
            ..Default::default()
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_start(&self, req: StartRequest) -> Result<StartResponse> {
        if req.exec_id().is_empty().not() {
            return Err(ShimError::Unimplemented("exec is not supported".to_string()).into());
        }

        let i = self.get_instance(req.id())?;
        let pid = i.start()?;

        self.events.send(TaskStart {
            container_id: req.id().into(),
            pid,
            ..Default::default()
        });

        let events = self.events.clone();

        let id = req.id().to_string();

        thread::Builder::new()
            .name(format!("{id}-wait"))
            .spawn(move || {
                let (exit_code, timestamp) = i.wait();
                events.send(TaskExit {
                    container_id: id.clone(),
                    exit_status: exit_code,
                    exited_at: Some(timestamp.to_timestamp()).into(),
                    pid,
                    id,
                    ..Default::default()
                });
            })
            .context("could not spawn thread to wait exit")
            .map_err(Error::from)?;

        debug!("started: {:?}", req);

        Ok(StartResponse {
            pid,
            ..Default::default()
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_kill(&self, req: KillRequest) -> Result<Empty> {
        if !req.exec_id().is_empty() {
            return Err(Error::InvalidArgument("exec is not supported".to_string()));
        }
        self.get_instance(req.id())?.kill(req.signal())?;
        Ok(Empty::new())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_delete(&self, req: DeleteRequest) -> Result<DeleteResponse> {
        if !req.exec_id().is_empty() {
            return Err(Error::InvalidArgument("exec is not supported".to_string()));
        }

        let i = self.get_instance(req.id())?;

        i.delete()?;

        let pid = i.pid().unwrap_or_default();
        let (exit_code, timestamp) = i.wait_timeout(Duration::ZERO).unzip();
        let timestamp = timestamp.map(ToTimestamp::to_timestamp);

        self.instances.write().unwrap().remove(req.id());

        self.events.send(TaskDelete {
            container_id: req.id().into(),
            pid,
            exit_status: exit_code.unwrap_or_default(),
            exited_at: timestamp.clone().into(),
            ..Default::default()
        });

        Ok(DeleteResponse {
            pid,
            exit_status: exit_code.unwrap_or_default(),
            exited_at: timestamp.into(),
            ..Default::default()
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_wait(&self, req: WaitRequest) -> Result<WaitResponse> {
        if !req.exec_id().is_empty() {
            return Err(Error::InvalidArgument("exec is not supported".to_string()));
        }

        let i = self.get_instance(req.id())?;
        let (exit_code, timestamp) = i.wait();

        debug!("wait finishes");
        Ok(WaitResponse {
            exit_status: exit_code,
            exited_at: Some(timestamp.to_timestamp()).into(),
            ..Default::default()
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_state(&self, req: StateRequest) -> Result<StateResponse> {
        if !req.exec_id().is_empty() {
            return Err(Error::InvalidArgument("exec is not supported".to_string()));
        }

        let i = self.get_instance(req.id())?;
        let pid = i.pid();
        let (exit_code, timestamp) = i.wait_timeout(Duration::ZERO).unzip();
        let timestamp = timestamp.map(ToTimestamp::to_timestamp);

        let status = if pid.is_none() {
            Status::CREATED
        } else if exit_code.is_none() {
            Status::RUNNING
        } else {
            Status::STOPPED
        };

        Ok(StateResponse {
            bundle: i.config().get_bundle().to_string_lossy().to_string(),
            stdin: i.config().get_stdin().to_string_lossy().to_string(),
            stdout: i.config().get_stdout().to_string_lossy().to_string(),
            stderr: i.config().get_stderr().to_string_lossy().to_string(),
            pid: pid.unwrap_or_default(),
            exit_status: exit_code.unwrap_or_default(),
            exited_at: timestamp.into(),
            status: status.into(),
            ..Default::default()
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Debug"))]
    fn task_stats(&self, req: StatsRequest) -> Result<StatsResponse> {
        let i = self.get_instance(req.id())?;
        let pid = i
            .pid()
            .ok_or_else(|| Error::InvalidArgument("task is not running".to_string()))?;

        let metrics = get_metrics(pid)?;

        Ok(StatsResponse {
            stats: Some(metrics).into(),
            ..Default::default()
        })
    }
}

impl<T: Instance + Sync + Send, E: EventSender> Task for Local<T, E> {
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, level = "Info"))]
    fn create(
        &self,
        ctx: &TtrpcContext,
        req: CreateTaskRequest,
    ) -> TtrpcResult<CreateTaskResponse> {
        debug!("create: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        Ok(self.task_create(req)?)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn start(&self, ctx: &TtrpcContext, req: StartRequest) -> TtrpcResult<StartResponse> {
        debug!("start: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        Ok(self.task_start(req)?)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn kill(&self, ctx: &TtrpcContext, req: KillRequest) -> TtrpcResult<Empty> {
        debug!("kill: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        Ok(self.task_kill(req)?)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn delete(&self, ctx: &TtrpcContext, req: DeleteRequest) -> TtrpcResult<DeleteResponse> {
        debug!("delete: {:?}", req);
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));
        Ok(self.task_delete(req)?)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn wait(&self, ctx: &TtrpcContext, req: WaitRequest) -> TtrpcResult<WaitResponse> {
        debug!("wait: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        {
            use tracing::{span, Level, Span};
            let parent_span = Span::current();
            parent_span.set_parent(extract_context(&ctx.metadata));

            let (tx, rx) = std::sync::mpsc::channel();
            // Start a thread to export interval span for long wait

            let _ = thread::spawn(move || loop {
                let current_span =
                    span!(parent: &parent_span, Level::INFO, "task wait 60s interval");
                let _enter = current_span.enter();
                if rx.recv_timeout(Duration::from_secs(60)).is_ok() {
                    break;
                }
            });
            let result = self.task_wait(req)?;
            tx.send(()).unwrap();
            Ok(result)
        }

        #[cfg(not(feature = "opentelemetry"))]
        {
            Ok(self.task_wait(req)?)
        }
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn connect(&self, ctx: &TtrpcContext, req: ConnectRequest) -> TtrpcResult<ConnectResponse> {
        debug!("connect: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        let i = self.get_instance(req.id())?;
        let shim_pid = std::process::id();
        let task_pid = i.pid().unwrap_or_default();
        Ok(ConnectResponse {
            shim_pid,
            task_pid,
            ..Default::default()
        })
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn state(&self, ctx: &TtrpcContext, req: StateRequest) -> TtrpcResult<StateResponse> {
        debug!("state: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        Ok(self.task_state(req)?)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn shutdown(&self, ctx: &TtrpcContext, _: ShutdownRequest) -> TtrpcResult<Empty> {
        debug!("shutdown");

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        if self.is_empty() {
            self.exit.signal();
        }
        Ok(Empty::new())
    }

    #[cfg_attr(feature = "tracing", tracing::instrument(parent = tracing::Span::current(), skip_all, level = "Info"))]
    fn stats(&self, ctx: &TtrpcContext, req: StatsRequest) -> TtrpcResult<StatsResponse> {
        debug!("stats: {:?}", req);

        #[cfg(feature = "opentelemetry")]
        tracing::Span::current().set_parent(extract_context(&ctx.metadata));

        Ok(self.task_stats(req)?)
    }
}
