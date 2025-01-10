use std::any::{Any, TypeId};

#[derive(Default)]
pub struct TypeMap {
    map: hashbrown::HashMap<TypeId, Box<dyn Any>>,
}

impl std::fmt::Debug for TypeMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TypeMap").finish_non_exhaustive()
    }
}

impl TypeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_default<T: Default + 'static>(self) -> Self {
        self.with(T::default())
    }

    pub fn with<T: 'static>(self, value: T) -> Self {
        self.with_id(TypeId::of::<T>(), Box::new(value))
    }

    pub fn with_id(mut self, id: TypeId, value: Box<dyn Any>) -> Self {
        self.set_by_id(id, value);
        self
    }

    pub fn set<T: 'static>(&mut self, value: T) {
        self.set_by_id(TypeId::of::<T>(), Box::new(value));
    }

    pub fn set_by_id(&mut self, id: TypeId, value: Box<dyn Any>) {
        self.map.insert(id, value);
    }

    pub fn contains_id(&self, id: TypeId) -> bool {
        self.map.contains_key(&id)
    }

    pub fn contains<T: 'static>(&self) -> bool {
        self.contains_id(TypeId::of::<T>())
    }

    pub fn set_default<T: Default + 'static>(&mut self) {
        self.set(T::default());
    }

    pub fn add<T: 'static>(&mut self, value: T) {
        self.add_by_id(TypeId::of::<T>(), Box::new(value));
    }

    pub fn add_by_id(&mut self, id: TypeId, value: Box<dyn Any>) {
        if !self.contains_id(id) {
            self.set_by_id(id, value);
        }
    }

    pub fn add_default<T: Default + 'static>(&mut self) {
        self.add(T::default());
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.get_by_id(TypeId::of::<T>()).map(|x| x.downcast_ref::<T>()).flatten()
    }

    pub fn get_by_id(&self, id: TypeId) -> Option<&Box<dyn Any>> {
        self.map.get(&id)
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_mut_by_id(TypeId::of::<T>()).map(|x| x.downcast_mut::<T>()).flatten()
    }

    pub fn get_mut_by_id(&mut self, id: TypeId) -> Option<&mut Box<dyn Any>> {
        self.map.get_mut(&id)
    }

    pub fn get_defaulted<T: Default + 'static>(&mut self) -> &T {
        self.add_default::<T>();
        self.get().unwrap()
    }

    pub fn get_mut_defaulted<T: Default + 'static>(&mut self) -> &mut T {
        self.add_default::<T>();
        self.get_mut().unwrap()
    }

    pub fn require<T: 'static>(&self) -> &T {
        self.get().unwrap()
    }

    pub fn require_mut<T: 'static>(&mut self) -> &mut T {
        self.get_mut().unwrap()
    }
}