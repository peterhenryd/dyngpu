use crate::error::Error;
use crate::resolution::Resolution;
use pollster::FutureExt;
use wgpu::{Adapter, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, Limits, LoadOp, MemoryHints, Operations, PowerPreference, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration, SurfaceTarget, TextureUsages, TextureViewDescriptor};

pub mod context;

pub use context::GpuContext as Context;
use crate::{collections, task};

#[derive(Debug)]
pub struct RenderGpu<'w> {
    pub context: Context<'w>,
    pub surface_configuration: SurfaceConfiguration,
    pub resources: collections::TypeMap,
}

impl<'w> RenderGpu<'w> {
    pub fn new(target: impl Into<SurfaceTarget<'w>>, resolution: impl Resolution<u32>) -> Result<Self, Error> {
        let instance = Instance::default();
        let surface = instance.create_surface(target)?;
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).block_on().ok_or(Error::RequestAdapter)?;
        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: MemoryHints::default(),
            label: None,
        }, None).block_on()?;
        let surface_configuration = configure_surface(&adapter, &device, &surface, resolution);
        let context = Context::new(device, queue, surface)?;
        let resources = collections::TypeMap::new();

        Ok(Self { context, surface_configuration, resources })
    }

    pub fn resize(&mut self, resolution: impl Resolution<u32>) {
        self.surface_configuration.width = resolution.get_width();
        self.surface_configuration.height = resolution.get_height();
        self.context.configure_surface(&self.surface_configuration);
    }

    pub fn ctx(&self) -> Context {
        self.context.clone()
    }

    pub fn render<S>(&mut self, task_executor: &mut task::Executor<S>, state: &S) {
        let surface_texture = self.context.surface().get_current_texture().unwrap();
        let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.context.device().create_command_encoder(&CommandEncoderDescriptor::default());

        task_executor.update_active_tasks(self.context.clone(), state);

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                label: None,
            });

            task_executor.render_active_tasks(self.context.clone(), &mut render_pass);

        }
        self.context.queue().submit(Some(encoder.finish()));
        surface_texture.present();
    }
}

pub fn configure_surface(adapter: &Adapter, device: &Device, surface: &Surface, resolution: impl Resolution<u32>) -> SurfaceConfiguration {
    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities.formats.iter()
        .find(|f| f.is_srgb())
        .copied()
        .unwrap_or(capabilities.formats[0]);
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format,
        width: resolution.get_width(),
        height: resolution.get_height(),
        present_mode: capabilities.present_modes[0],
        desired_maximum_frame_latency: 2,
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: vec![],
    };
    surface.configure(&device, &config);
    config
}