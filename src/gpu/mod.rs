pub mod device;
pub mod surface;

pub use device::*;
pub use surface::*;

use crate::{Resources, Resolution, Error, Stages};

pub struct Gpu<'w> {
    device: Device,
    surface: Surface<'w>,
    resources: Resources,
}

impl<'w> Gpu<'w> {
    pub fn new(
        resolution: impl Resolution<u32>,
        target: impl Into<wgpu::SurfaceTarget<'w>>
    ) -> Result<Self, Error> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(target)?;
        let device = Device::new(&instance, &surface)?;
        let surface = Surface::new(device.clone(), surface, resolution);
        let resources = Resources::new(device.clone());

        Ok(Self { device, surface, resources })
    }

    pub fn borrow_mut(&mut self) -> (&Device, &mut Surface<'w>, &mut Resources) {
        (&self.device, &mut self.surface, &mut self.resources)
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn surface(&self) -> &Surface<'w> {
        &self.surface
    }

    pub fn surface_mut(&mut self) -> &mut Surface<'w> {
        &mut self.surface
    }

    pub fn render(&mut self, render_passes: &mut Stages) {
        render_passes.update(self);
        render_passes.render(self);
    }

    pub fn res(&self) -> &Resources {
        &self.resources
    }

    pub fn res_mut(&mut self) -> &mut Resources {
        &mut self.resources
    }
}

