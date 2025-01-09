extern crate alloc;

use crate::error::Error;
use crate::gpu::RenderGpu;
use crate::resolution::GetResolution;
use crate::task::executor::TaskExecutor;
use crate::task::Task;
use wgpu::SurfaceTarget;

#[cfg(feature = "winit")]
pub mod winit;
pub mod gpu;
pub mod error;
pub mod resolution;
pub mod task;
pub mod collections;
pub mod resource;

pub mod prelude {
    pub use crate::error::*;
    pub use crate::gpu::*;
    pub use crate::resolution::*;
    #[cfg(feature = "winit")]
    pub use crate::winit::*;
}

#[derive(Debug)]
pub struct Render<'w, S> {
    gpu: Option<RenderGpu<'w>>,
    task_executor: TaskExecutor<S>,
}

impl<S> Default for Render<'_, S> {
    fn default() -> Self {
        Self {
            gpu: None,
            task_executor: TaskExecutor::default(),
        }
    }
}

impl<'w, S> Render<'w, S> {
    pub fn draw_frame<'e>(&mut self, state: &S) {
        let Some(gpu) = &mut self.gpu else { return };
        gpu.render(&mut self.task_executor, state);
    }

    pub fn add_task<T: Task<State = S> + 'static>(&mut self) {
        self.task_executor.queue_task::<T>();
    }

    pub fn initialize<T>(&mut self, target: T) -> Result<(), Error>
    where T: Into<SurfaceTarget<'w>>,
          T: GetResolution<u32> {
        let resolution = target.get_resolution();
        self.gpu = Some(RenderGpu::new(target, resolution)?);
        self.task_executor.load_pending_tasks(self.gpu.as_mut().unwrap());
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.gpu.is_some()
    }

    pub fn get_gpu(&self) -> Option<&RenderGpu<'w>> {
        self.gpu.as_ref()
    }

    pub fn gpu(&self) -> &RenderGpu<'w> {
        self.get_gpu().unwrap()
    }

    pub fn get_gpu_mut(&mut self) -> Option<&mut RenderGpu<'w>> {
        self.gpu.as_mut()
    }

    pub fn gpu_mut(&mut self) -> &mut RenderGpu<'w> {
        self.get_gpu_mut().unwrap()
    }
}

macro_rules! reexport {
    ($(mod $name:ident;)+) => {
        $(mod $name;)+
        $(
        pub use $name::*;
        )+
    };
}

pub(crate) use reexport;