use crate::resource::Resources;
use crate::{Device, Task};

pub struct TaskConstructor {
    function: fn(&Device, &mut Resources) -> Box<dyn Task>,
}

impl TaskConstructor {
    pub const fn new<T: Task + 'static>() -> Self {
        Self {
            function: construct::<T>,
        }
    }

    pub fn build(&self, device: &Device, res: &mut Resources) -> Box<dyn Task> {
        (self.function)(device, res)
    }
}

fn construct<T: Task + 'static>(device: &Device, res: &mut Resources) -> Box<dyn Task> {
    Box::new(T::new(device, res))
}