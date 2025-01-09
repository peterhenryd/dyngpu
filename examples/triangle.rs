use dyngpu::error::Error;
use dyngpu::prelude::RenderApp;
use dyngpu::resource::shaders::Shaders;
use dyngpu::task::Task;
use dyngpu::{collections, gpu};
use wgpu::{include_wgsl, vertex_attr_array, Buffer, BufferAddress, RenderPass, RenderPipeline, VertexBufferLayout, VertexStepMode};

pub fn main() -> Result<(), Error> {
    RenderApp::stateless()
        .add_render_task::<RenderTriangle>()
        .run()
}

pub struct RenderTriangle {
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
}

impl Task for RenderTriangle {
    type State = ();

    fn new(gpu: gpu::Context, res: &mut collections::TypeMap) -> Self where Self: Sized {
        let mut shaders = res.resource_mut::<Shaders>(&gpu);
        let shader = shaders.create("triangle", include_wgsl!("triangle.wgsl"));

        let vertices: [[f32; 3]; 3] = [[0., 0.5, 0.], [-0.5, -0.5, 0.], [0.5, -0.5, 0.]];

        Self {
            vertex_buffer: gpu.build_buffer()
                .contents_slice(&vertices)
                .vert()
                .finish(),
            render_pipeline: gpu.build_pipeline(&shader)
                .vert_buffer(VertexBufferLayout {
                    array_stride: size_of::<[f32; 3]>() as BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &vertex_attr_array!(0 => Float32x3),
                })
                .finish(),
        }
    }

    fn update(&mut self, _: &Self::State, _: gpu::Context) {}

    fn render(&self, _: gpu::Context, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
}