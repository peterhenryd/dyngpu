use dyngpu::{Device, Error, RenderApp, Resources, Task};

fn main() -> Result<(), Error> {
    RenderApp::default()
        .with_stage_task::<RenderRainbow>()
        .run()
}

pub struct RenderRainbow {
    vertex_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
    update: u32,
    update_uniform: wgpu::Buffer,
    update_bind_group: wgpu::BindGroup,
}

impl Task for RenderRainbow {
    fn new(device: &Device, res: &mut Resources) -> Self
    where
        Self: Sized,
    {
        let update = 0;
        let update_uniform = device
            .build_buffer()
            .contents_slice(&[update])
            .uniform()
            .copy_dst()
            .finish();
        let (update_bind_group, layout) = device
            .build_bind_group()
            .uniform(&update_uniform, wgpu::ShaderStages::FRAGMENT)
            .finish_with_layout();

        let shader = res.create_shader("rainbow", wgpu::include_wgsl!("rainbow.wgsl"));
        let render_pipeline = device
            .build_pipeline(&shader)
            .vert_buffer(wgpu::VertexBufferLayout {
                array_stride: size_of::<[f32; 3]>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array!(0 => Float32x3),
            })
            .bind_group(layout.as_ref().unwrap())
            .finish();

        let vertices: [[f32; 3]; 3] = [[0., 0.5, 0.], [-0.5, -0.5, 0.], [0.5, -0.5, 0.]];
        let vertex_buffer = device
            .build_buffer()
            .contents_slice(&vertices)
            .vert()
            .finish();

        Self {
            vertex_buffer,
            render_pipeline,
            update,
            update_uniform,
            update_bind_group,
        }
    }

    fn update(&mut self, device: &Device) {
        self.update += 1;
        device.queue()
            .write_buffer(&self.update_uniform, 0, bytemuck::cast_slice(&[self.update]));
    }

    fn render(&self, _: &Device, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.update_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}