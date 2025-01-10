use crate::{Device, Resources, Task, TaskConstructor};

pub struct TaskExecutor {
    constructors: Vec<TaskConstructor>,
    tasks: Vec<Box<dyn Task>>,
}

impl TaskExecutor {
    pub fn load_pending_tasks(&mut self, device: &Device, res: &mut Resources) {
        for constructor in &self.constructors {
            let task = constructor.build(device, res);
            self.tasks.push(task);
        }
        self.constructors.clear();
    }

    pub fn queue_task_constructor(&mut self, constructor: TaskConstructor) {
        self.constructors.push(constructor);
    }

    pub fn queue_task<T: Task + 'static>(&mut self) {
        self.queue_task_constructor(TaskConstructor::new::<T>())
    }

    pub fn update_active_tasks(&mut self, device: &Device) {
        for task in &mut self.tasks {
            task.update(device);
        }
    }

    pub fn render_active_tasks<'e>(&self, device: &Device, render_pass: &mut wgpu::RenderPass<'e>) {
        for task in &self.tasks {
            task.render(device, render_pass);
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

impl Default for TaskExecutor {
    fn default() -> Self {
        Self {
            constructors: vec![],
            tasks: vec![],
        }
    }
}