use crate::{Gpu, Stage};

#[derive(Default)]
pub struct Stages {
    vec: Vec<Stage>,
}

impl Stages {
    pub fn add_stage(&mut self, stage: Stage) {
        self.vec.push(stage);
    }

    pub fn initialize(&mut self, gpu: &mut Gpu) {
        for stage in &mut self.vec {
            stage.initialize(gpu);
        }
    }

    pub fn update(&mut self, gpu: &Gpu) {
        for stage in &mut self.vec {
            stage.update(gpu);
        }
    }

    pub fn render(&self, gpu: &Gpu) {
        let (texture, view) = gpu.surface().current_texture_and_view();
        let mut encoder = gpu.device().create_encoder();

        for stage in &self.vec {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: clear_color(stage.clear_color),
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            stage.render(gpu, &mut render_pass);
        }
        gpu.device().submit_encoder(encoder);
        texture.present();
    }
}

pub fn clear_color<C>(color: C) -> wgpu::Operations<C> {
    wgpu::Operations {
        load: wgpu::LoadOp::Clear(color),
        store: wgpu::StoreOp::Store,
    }
}