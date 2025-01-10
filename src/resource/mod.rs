pub mod shaders;

use std::any::TypeId;
use crate::collections::TypeMap;
use crate::Device;

pub trait Resource {
    fn create(device: &Device) -> Self;
}

pub struct Resources {
    device: Device,
    type_map: TypeMap,
}

impl Resources {
    pub fn new(device: Device) -> Self {
        Self {
            device,
            type_map: TypeMap::new(),
        }
    }

    pub fn get<'w, T: Resource + 'static>(&mut self) -> &T {
        let id = TypeId::of::<T>();
        if !self.type_map.contains_id(id) {
            self.type_map.set_by_id(id, Box::new(T::create(&self.device)));
        }

        self.type_map.get().unwrap()
    }

    pub fn get_mut<'w, T: Resource + 'static>(&mut self) -> &mut T {
        let id = TypeId::of::<T>();
        if !self.type_map.contains_id(id) {
            self.type_map.set_by_id(id, Box::new(T::create(&self.device)));
        }

        self.type_map.get_mut().unwrap()
    }
}