use crate::error::Error;
use crate::resolution::{GetResolution, Resolution};
use crate::task::Task;
use crate::Render;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

impl<T: Copy> Resolution<T> for PhysicalSize<T> {
    fn get_width(&self) -> T {
        self.width
    }

    fn get_height(&self) -> T {
        self.height
    }
}

impl GetResolution<u32> for Window {
    type Resolution = PhysicalSize<u32>;

    fn get_resolution(&self) -> Self::Resolution {
        self.inner_size()
    }
}

impl GetResolution<u32> for Arc<Window> {
    type Resolution = PhysicalSize<u32>;

    fn get_resolution(&self) -> Self::Resolution {
        self.inner_size()
    }
}

pub struct RenderWindow<'w, S> {
    window: Option<Arc<Window>>,
    render: Render<'w, S>
}

impl<S> Default for RenderWindow<'_, S> {
    fn default() -> Self {
        Self {
            window: None,
            render: Render::<S>::default()
        }
    }
}

impl<'w, S> Deref for RenderWindow<'w, S> {
    type Target = Render<'w, S>;

    fn deref(&self) -> &Self::Target {
        &self.render
    }
}

impl<'w, S> DerefMut for RenderWindow<'w, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render
    }
}

impl<'w, S> RenderWindow<'w, S> {
    pub fn is_initialized(&self) -> bool {
        self.window.is_some() && self.render.is_initialized()
    }

    pub fn initialize(
        &mut self,
        window_attributes: &mut Option<WindowAttributes>,
        event_loop: &ActiveEventLoop
    ) -> Result<(), Error> {
        let window_attributes = window_attributes.take().unwrap_or_default()
            .with_title("")
            .with_inner_size(LogicalSize::new(720, 480));
        let window = event_loop.create_window(window_attributes)?;

        self.window = Some(Arc::new(window));
        self.render.initialize(self.window.as_ref().unwrap().clone())?;

        Ok(())
    }

    pub fn get_window(&self) -> Option<Arc<Window>> {
        self.window.clone()
    }

    pub fn window(&self) -> Arc<Window> {
        self.get_window().unwrap()
    }
}

pub struct RenderApp<'w, S = ()> {
    render: RenderWindow<'w, S>,
    window_attributes: Option<WindowAttributes>,
    state: S,
}

pub trait App {
    fn update(&mut self);
}

impl App for () {
    fn update(&mut self) {}
}

impl<S: App> ApplicationHandler for RenderApp<'_, S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.render.is_initialized() {
            self.render.initialize(&mut self.window_attributes, event_loop).unwrap();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        use WindowEvent::*;
        match event {
            Resized(size) => {
                self.render.gpu_mut().resize(size)
            },
            Moved(_) => {}
            CloseRequested | Destroyed => event_loop.exit(),
            DroppedFile(_) => {}
            HoveredFile(_) => {}
            HoveredFileCancelled => {}
            Focused(_) => {}
            KeyboardInput { .. } => {}
            ModifiersChanged(_) => {}
            Ime(_) => {}
            CursorMoved { .. } => {}
            CursorEntered { .. } => {}
            CursorLeft { .. } => {}
            MouseWheel { .. } => {}
            MouseInput { .. } => {}
            PinchGesture { .. } => {}
            PanGesture { .. } => {}
            DoubleTapGesture { .. } => {}
            RotationGesture { .. } => {}
            TouchpadPressure { .. } => {}
            AxisMotion { .. } => {}
            Touch(_) => {}
            ScaleFactorChanged { .. } => {}
            ThemeChanged(_) => {}
            Occluded(_) => {}
            RedrawRequested => {
                self.state.update();
                self.render.draw_frame(&self.state);
            }
            _ => {}
        }
    }
}

impl<S> From<S> for RenderApp<'_, S> {
    fn from(state: S) -> Self {
        Self {
            render: RenderWindow::default(),
            window_attributes: Some(WindowAttributes::default()
                .with_title("")
                .with_inner_size(LogicalSize::new(720, 480))
            ),
            state
        }
    }
}

impl<S: Default> Default for RenderApp<'_, S> {
    fn default() -> Self {
        Self::from(S::default())
    }
}

impl<S: App> RenderApp<'_, S> {
    pub fn with_window_attributes(mut self, function: impl FnOnce(WindowAttributes) -> WindowAttributes) -> Self {
        self.window_attributes = Some(function(self.window_attributes.take().unwrap_or_default()));
        self
    }

    pub fn add_render_task<T: Task<State = S> + 'static>(mut self) -> Self {
        self.render.task_executor.queue_task::<T>();
        self
    }

    pub fn run(mut self) -> Result<(), Error> {
        Ok(EventLoop::new()?.run_app(&mut self)?)
    }
}

impl RenderApp<'_> {
    pub fn stateless() -> Self {
        Self::default()
    }
}