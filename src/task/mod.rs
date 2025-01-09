pub mod executor;
pub mod constructor;

use crate::{collections, gpu};

pub use executor::TaskExecutor as Executor;
pub use constructor::TaskConstructor as Constructor;

pub trait Task {
    type State;

    fn new(gpu: gpu::Context, res: &mut collections::TypeMap) -> Self where Self: Sized;

    fn update(&mut self, state: &Self::State, gpu: gpu::Context);

    fn render(&self, gpu: gpu::Context, render_pass: &mut wgpu::RenderPass);
}