use crate::{Gpu, Task, TaskConstructor, TaskExecutor};

pub mod task;
pub mod store;
pub mod state;

#[derive(Default)]
pub struct Stage {
    clear_color: wgpu::Color,
    task_executor: TaskExecutor,
}

impl Stage {
    pub fn with_task<T: Task + 'static>(mut self) -> Self {
        self.task_executor.queue_task_constructor(TaskConstructor::new::<T>());
        self
    }

    pub fn render(&self, gpu: &Gpu, render_pass: &mut wgpu::RenderPass<'_>) {
        self.task_executor.render_active_tasks(gpu.device(), render_pass);
    }

    pub fn initialize(&mut self, gpu: &mut Gpu) {
        let (device, _, res) = gpu.borrow_mut();
        self.task_executor.load_pending_tasks(device, res);
    }

    pub fn update(&mut self, gpu: &Gpu) {
        self.task_executor.update_active_tasks(gpu.device())
    }
}