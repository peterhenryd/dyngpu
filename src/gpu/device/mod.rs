use pollster::FutureExt;
use crate::Error;

pub mod render_pipeline;
pub mod buffer;
pub mod bind_group;

#[derive(Debug, Clone)]
pub struct Device(std::sync::Arc<DeviceOwned>);

impl AsRef<wgpu::Device> for Device {
    fn as_ref(&self) -> &wgpu::Device {
        &self.0.as_ref().device
    }
}

impl Device {
    pub fn new(instance: &wgpu::Instance, surface: &wgpu::Surface<'_>) -> Result<Self, Error> {
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).block_on().ok_or(Error::RequestAdapter)?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None).block_on()?;

        Ok(Self(std::sync::Arc::new(DeviceOwned { adapter, device, queue })))
    }

    pub fn borrow(&self) -> (&wgpu::Adapter, &wgpu::Device, &wgpu::Queue) {
        (&self.0.adapter, &self.0.device, &self.0.queue)
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.0.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.0.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.0.queue
    }

    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.0.device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default())
    }

    pub fn submit_encoder(&self, encoder: wgpu::CommandEncoder) {
        self.0.queue.submit(Some(encoder.finish()));
    }
}

#[derive(Debug)]
struct DeviceOwned {
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}