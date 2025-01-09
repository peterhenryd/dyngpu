use std::sync::Arc;
use wgpu::{ShaderModule, ShaderModuleDescriptor};
use crate::{gpu, resource};

#[derive(Default)]
pub struct Shaders(hashbrown::HashMap<String, Arc<ShaderModule>>);

pub struct ShadersCtx<'a> {
    shaders: &'a Shaders,
}

impl<'a> ShadersCtx<'a> {
    pub fn get(&self, name: impl AsRef<str>) -> Option<Arc<ShaderModule>> {
        self.shaders.0.get(name.as_ref()).cloned()
    }

    pub fn require(&self, name: impl AsRef<str>) -> Arc<ShaderModule> {
        self.get(name).unwrap()
    }
}

impl std::ops::Index<&'static str> for Shaders {
    type Output = ShaderModule;

    fn index(&self, index: &str) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

pub struct ShadersCtxMut<'w, 'a> {
    ctx: gpu::Context<'w>,
    shaders: &'a mut Shaders,
}

impl ShadersCtxMut<'_, '_> {
    pub fn create(&mut self, name: impl Into<String>, shader: ShaderModuleDescriptor) -> Arc<ShaderModule> {
        let module = self.ctx.device().create_shader_module(shader);
        self.insert(name, module)
    }

    pub fn insert(&mut self, name: impl Into<String>, shader: ShaderModule) -> Arc<ShaderModule> {
        let name = name.into();
        self.shaders.0.insert(name.clone(), Arc::new(shader));
        self.shaders.0.get(&name).unwrap().clone()
    }
}

impl resource::Resource for Shaders {
    type Output<'w, 'a> = ShadersCtx<'a>;
    type OutputMut<'w, 'a> = ShadersCtxMut<'w, 'a>;

    fn contextualize<'w>(&self, _: gpu::Context<'w>) -> Self::Output<'w, '_> {
        ShadersCtx { shaders: self, }
    }

    fn contextualize_mut<'w>(&mut self, ctx: gpu::Context<'w>) -> Self::OutputMut<'w, '_> {
        ShadersCtxMut { ctx, shaders: self }
    }
}