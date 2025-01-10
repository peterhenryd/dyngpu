pub mod collections;
pub mod error;
pub mod gpu;
pub mod resolution;
pub mod resource;
#[cfg(feature = "winit")]
pub mod winit;
pub mod stage;

pub use collections::*;
pub use error::*;
pub use gpu::Gpu;
pub use gpu::device::*;
pub use gpu::surface::*;
pub use stage::Stage;
pub use stage::task::*;
pub use stage::task::constructor::*;
pub use stage::task::executor::*;
pub use stage::store::*;
pub use resolution::*;
pub use resource::*;
pub use resource::shaders::*;
#[cfg(feature = "winit")]
pub use winit::*;

pub struct Renderer<'w> {
    gpu: Option<Gpu<'w>>,
    stages: Stages,
}

impl Default for Renderer<'_> {
    fn default() -> Self {
        Self {
            gpu: None,
            stages: Stages::default(),
        }
    }
}

impl<'w> Renderer<'w> {
    pub fn draw_frame<'e>(&mut self) {
        let Some(gpu) = &mut self.gpu else { return };
        gpu.render(&mut self.stages);
    }

    pub fn initialize<T>(&mut self, target: T) -> Result<(), Error>
    where T: Into<wgpu::SurfaceTarget<'w>>,
          T: GetResolution<u32> {
        let gpu = Gpu::new(target.get_resolution(), target)?;
        self.gpu = Some(gpu);
        self.stages.initialize(self.gpu.as_mut().unwrap());
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.gpu.is_some()
    }

    pub fn get_gpu(&self) -> Option<&Gpu<'w>> {
        self.gpu.as_ref()
    }

    pub fn gpu(&self) -> &Gpu<'w> {
        self.get_gpu().unwrap()
    }

    pub fn get_gpu_mut(&mut self) -> Option<&mut Gpu<'w>> {
        self.gpu.as_mut()
    }

    pub fn gpu_mut(&mut self) -> &mut Gpu<'w> {
        self.get_gpu_mut().unwrap()
    }

    pub fn stages(&self) -> &Stages {
        &self.stages
    }

    pub fn stages_mut(&mut self) -> &mut Stages {
        &mut self.stages
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