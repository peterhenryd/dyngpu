use crate::{gpu, task};
use crate::gpu::RenderGpu;
use wgpu::RenderPass;

pub struct TaskExecutor<S> {
    constructors: Vec<task::Constructor<S>>,
    tasks: Vec<Box<dyn task::Task<State = S>>>,
}

impl<S> TaskExecutor<S> {
    pub fn load_pending_tasks(&mut self, render: &mut RenderGpu) {
        let mut tasks = Vec::with_capacity(self.constructors.len());
        for constructor in &self.constructors {
            let task = constructor.build(render.context.clone(), &mut render.resources);
            tasks.push(task);
        }
        self.constructors.clear();
        self.tasks.extend(tasks);
    }

    pub fn queue_task_constructor(&mut self, constructor: task::Constructor<S>) {
        self.constructors.push(constructor);
    }

    pub fn queue_task<T: task::Task<State = S> + 'static>(&mut self) {
        self.queue_task_constructor(task::Constructor::new::<T>())
    }

    pub fn update_active_tasks(&mut self, gpu: gpu::Context, state: &S) {
        for task in &mut self.tasks {
            task.update(state, gpu.clone());
        }
    }

    pub fn render_active_tasks<'e>(&self, gpu: gpu::Context, render_pass: &mut RenderPass<'e>) {
        for task in &self.tasks {
            task.render(gpu.clone(), render_pass);
        }
    }

    pub fn remove_active_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn remove_pending_tasks(&mut self) {
        self.constructors.clear();
    }

    pub fn remove_all_tasks(&mut self) {
        self.remove_active_tasks();
        self.remove_pending_tasks();
    }
}

impl<S> Default for TaskExecutor<S> {
    fn default() -> Self {
        Self {
            constructors: vec![],
            tasks: vec![],
        }
    }
}

impl<S> std::fmt::Debug for TaskExecutor<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskExecutor").finish()
    }
}