use crate::{Device, Resolution};

#[derive(Debug)]
pub struct Surface<'w> {
    device: Device,
    raw: wgpu::Surface<'w>,
    config: wgpu::SurfaceConfiguration,
}

impl<'w> Surface<'w> {
    pub fn new(
        device: Device,
        surface: wgpu::Surface<'w>,
        resolution: impl Resolution<u32>
    ) -> Self {
        let capabilities = surface.get_capabilities(device.adapter());
        let format = capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: resolution.get_width(),
            height: resolution.get_height(),
            present_mode: capabilities.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(device.as_ref(), &config);
        Self {
            device,
            raw: surface,
            config,
        }
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.raw.get_current_texture()
    }

    pub fn current_texture(&self) -> wgpu::SurfaceTexture {
        self.get_current_texture().unwrap()
    }

    pub fn current_texture_and_view(&self) -> (wgpu::SurfaceTexture, wgpu::TextureView) {
        let texture = self.get_current_texture().unwrap();
        let view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        (texture, view)
    }

    pub fn resize(&mut self, resolution: impl Resolution<u32>) {
        self.config.width = resolution.get_width();
        self.config.height = resolution.get_height();
        self.raw.configure(self.device.as_ref(), &self.config);
    }
}

impl<'w> AsRef<wgpu::Surface<'w>> for Surface<'w> {
    fn as_ref(&self) -> &wgpu::Surface<'w> {
        &self.raw
    }
}