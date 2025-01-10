use dyngpu::{Device, Error, RenderApp, Resources, Task};

pub fn main() -> Result<(), Error> {
    RenderApp::stateless()
        .with_window_attributes(|a| a.with_title("Triangle"))
        .with_stage_task::<RenderTriangle>()
        .run()
}

pub struct RenderTriangle {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

pub const VERTEX_BUFFER_LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: size_of::<[f32; 3]>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array!(0 => Float32x3),
};

impl Task for RenderTriangle {
    fn new(device: &Device, res: &mut Resources) -> Self where Self: Sized {
        let vertices: [[f32; 3]; 3] = [[0., 0.5, 0.], [-0.5, -0.5, 0.], [0.5, -0.5, 0.]];
        let vertex_buffer = device.build_buffer().contents_slice(&vertices).vert().finish();

        let shader = res.create_shader("triangle", wgpu::include_wgsl!("triangle.wgsl"));
        let render_pipeline = device.build_pipeline(&shader)
            .vert_buffer(VERTEX_BUFFER_LAYOUT)
            .finish();


        Self {
            vertex_buffer,
            render_pipeline,
        }
    }

    fn update(&mut self, _: &Device) {}

    fn render(&self, _: &Device, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..3, 0..1);
    }
}