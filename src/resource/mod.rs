pub mod shaders;

use crate::gpu;

pub trait Resource: Default {
    type Output<'w, 'a> where Self: 'a;
    type OutputMut<'w, 'a> where Self: 'a;

    fn contextualize<'w, 'a>(&'a self, ctx: gpu::Context<'w>) -> Self::Output<'w, 'a>;

    fn contextualize_mut<'w, 'a>(&'a mut self, ctx: gpu::Context<'w>) -> Self::OutputMut<'w, 'a>;
}