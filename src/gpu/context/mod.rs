pub mod render_pipeline;
pub mod buffer;
pub mod bind_group;

#[derive(Debug, Clone)]
pub struct GpuContext<'w>(std::sync::Arc<GpuOwned<'w>>);

impl<'w> GpuContext<'w> {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, surface: wgpu::Surface<'w>) -> Result<Self, wgpu::RequestDeviceError> {
        Ok(Self(std::sync::Arc::new(GpuOwned { device, queue, surface })))
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.0.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.0.queue
    }

    pub fn surface(&self) -> &wgpu::Surface { &self.0.surface }

    pub fn configure_surface(&self, config: &wgpu::SurfaceConfiguration) {
        self.0.surface.configure(&self.0.device, config);
    }
}

#[derive(Debug)]
pub struct GpuOwned<'w> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'w>
}