pub mod collections;
pub mod error;
pub mod gpu;
pub mod resolution;
pub mod resource;
pub mod task;
#[cfg(feature = "winit")]
pub mod winit;

pub mod prelude {
    pub use crate::Render;
    pub use crate::collections::TypeMap;
    pub use crate::error::Error;
    pub use crate::gpu::{Context, RenderGpu};
    pub use crate::gpu::context::GpuContext;
    pub use crate::resolution::Resolution;
    pub use crate::resource::Resource;
    pub use crate::resource::shaders::Shaders;
    pub use crate::task::Task;
    pub use crate::task::constructor::TaskConstructor;
    pub use crate::task::executor::TaskExecutor;
    #[cfg(feature = "winit")]
    pub use crate::winit::{App, RenderApp, RenderWindow};
}

pub struct Render<'w, S> {
    gpu: Option<gpu::RenderGpu<'w>>,
    task_executor: task::Executor<S>,
}

impl<S> Default for Render<'_, S> {
    fn default() -> Self {
        Self {
            gpu: None,
            task_executor: task::Executor::default(),
        }
    }
}

impl<'w, S> Render<'w, S> {
    pub fn draw_frame<'e>(&mut self, state: &S) {
        let Some(gpu) = &mut self.gpu else { return };
        gpu.render(&mut self.task_executor, state);
    }

    pub fn add_task<T: task::Task<State = S> + 'static>(&mut self) {
        self.task_executor.queue_task::<T>();
    }

    pub fn initialize<T>(&mut self, target: T) -> Result<(), error::Error>
    where T: Into<wgpu::SurfaceTarget<'w>>,
          T: resolution::GetResolution<u32> {
        let resolution = target.get_resolution();
        self.gpu = Some(gpu::RenderGpu::new(target, resolution)?);
        self.task_executor.load_pending_tasks(self.gpu.as_mut().unwrap());
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.gpu.is_some()
    }

    pub fn get_gpu(&self) -> Option<&gpu::RenderGpu<'w>> {
        self.gpu.as_ref()
    }

    pub fn gpu(&self) -> &gpu::RenderGpu<'w> {
        self.get_gpu().unwrap()
    }

    pub fn get_gpu_mut(&mut self) -> Option<&mut gpu::RenderGpu<'w>> {
        self.gpu.as_mut()
    }

    pub fn gpu_mut(&mut self) -> &mut gpu::RenderGpu<'w> {
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