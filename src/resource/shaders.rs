use crate::{Device, Resource, Resources};
use hashbrown::HashMap;
use std::sync::Arc;
use wgpu::{ShaderModule, ShaderModuleDescriptor};

pub struct Shaders {
    device: Device,
    module_map: HashMap<String, Arc<ShaderModule>>,
}

impl Shaders {
    pub fn get_module(&self, name: impl AsRef<str>) -> Option<Arc<ShaderModule>> {
        self.module_map.get(name.as_ref()).cloned()
    }

    pub fn module(&self, name: impl AsRef<str>) -> Arc<ShaderModule> {
        self.get_module(name).unwrap()
    }

    pub fn create(&mut self, name: impl Into<String>, shader: ShaderModuleDescriptor) -> Arc<ShaderModule> {
        let module = self.device.device().create_shader_module(shader);
        self.insert(name, module)
    }

    pub fn insert(&mut self, name: impl Into<String>, shader: ShaderModule) -> Arc<ShaderModule> {
        let name = name.into();
        self.module_map.insert(name.clone(), Arc::new(shader));
        self.module_map.get(&name).unwrap().clone()
    }
}

impl std::ops::Index<&'static str> for Shaders {
    type Output = ShaderModule;

    fn index(&self, index: &str) -> &Self::Output {
        self.module_map.get(index).unwrap().as_ref()
    }
}

impl std::ops::IndexMut<&'static str> for Shaders {
    fn index_mut(&mut self, index: &'static str) -> &mut Self::Output {
        self.module_map.get_mut(index).unwrap()
    }
}

impl Resource for Shaders {
    fn create(device: &Device) -> Self {
        Self {
            device: device.clone(),
            module_map: HashMap::default(),
        }
    }
}

impl Resources {
    pub fn shader(&mut self, name: impl AsRef<str>) -> Option<Arc<ShaderModule>> {
        self.get::<Shaders>().get_module(name)
    }

    pub fn create_shader(&mut self, name: impl Into<String>, shader: ShaderModuleDescriptor) -> Arc<ShaderModule> {
        let shaders = self.get_mut::<Shaders>();
        shaders.create(name, shader)
    }
}