pub mod executor;
pub mod constructor;

use crate::Device;
use crate::resource::Resources;

pub trait Task {
    fn new(device: &Device, res: &mut Resources) -> Self where Self: Sized;

    fn update(&mut self, device: &Device);

    fn render(&self, device: &Device, render_pass: &mut wgpu::RenderPass);
}