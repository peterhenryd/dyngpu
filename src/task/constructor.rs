use crate::{collections, gpu, task};

pub struct TaskConstructor<S> {
    function: fn(gpu::Context, &mut collections::TypeMap) -> Box<dyn task::Task<State = S>>,
}

impl<S> TaskConstructor<S> {
    pub const fn new<T: task::Task<State = S> + 'static>() -> Self {
        Self {
            function: construct::<S, T>,
        }
    }

    pub fn build(&self, gpu: gpu::Context, res: &mut collections::TypeMap) -> Box<dyn task::Task<State = S>> {
        (self.function)(gpu, res)
    }
}

fn construct<S, T: task::Task<State = S> + 'static>(gpu: gpu::Context, res: &mut collections::TypeMap) -> Box<dyn task::Task<State = S>> {
    Box::new(T::new(gpu, res))
}