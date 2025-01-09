use std::fmt::{Debug, Formatter};
use crate::{collections, gpu, task};

pub struct TaskConstructor<S> {
    function: fn(gpu::Context, &mut collections::TypeMap) -> Box<dyn task::Task<State = S>>,
}

impl<S> Debug for TaskConstructor<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskConstructor").finish()
    }
}

fn construct<S, T: task::Task<State = S> + 'static>(gpu: gpu::Context, res: &mut collections::TypeMap) -> Box<dyn task::Task<State = S>> {
    Box::new(T::new(gpu, res))
}

impl<S> TaskConstructor<S> {
    pub fn new<T: task::Task<State = S> + 'static>() -> Self {
        Self {
            function: construct::<S, T>
        }
    }

    pub fn build(&self, gpu: gpu::Context, res: &mut collections::TypeMap) -> Box<dyn task::Task<State = S>> {
        (self.function)(gpu, res)
    }
}