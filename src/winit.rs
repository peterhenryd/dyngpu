use crate::stage::task::Task;
use crate::{error, resolution, Renderer, Stage};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

impl<T: Copy> resolution::Resolution<T> for PhysicalSize<T> {
    fn get_width(&self) -> T {
        self.width
    }

    fn get_height(&self) -> T {
        self.height
    }
}

impl resolution::GetResolution<u32> for Window {
    type Resolution = PhysicalSize<u32>;

    fn get_resolution(&self) -> Self::Resolution {
        self.inner_size()
    }
}

impl resolution::GetResolution<u32> for Arc<Window> {
    type Resolution = PhysicalSize<u32>;

    fn get_resolution(&self) -> Self::Resolution {
        self.inner_size()
    }
}

pub struct RendererWindow<'w> {
    window: Option<Arc<Window>>,
    render: Renderer<'w>
}

impl Default for RendererWindow<'_> {
    fn default() -> Self {
        Self {
            window: None,
            render: Renderer::default()
        }
    }
}

impl<'w> std::ops::Deref for RendererWindow<'w> {
    type Target = Renderer<'w>;

    fn deref(&self) -> &Self::Target {
        &self.render
    }
}

impl<'w> std::ops::DerefMut for RendererWindow<'w> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render
    }
}

impl<'w> RendererWindow<'w> {
    pub fn is_initialized(&self) -> bool {
        self.window.is_some() && self.render.is_initialized()
    }

    pub fn initialize(
        &mut self,
        window_attributes: &mut Option<WindowAttributes>,
        event_loop: &ActiveEventLoop
    ) -> Result<(), error::Error> {
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

pub struct RenderApp<'w> {
    renderer: RendererWindow<'w>,
    window_attributes: Option<WindowAttributes>,
}

pub trait App {
    fn update(&mut self);
}

impl App for () {
    fn update(&mut self) {}
}

impl ApplicationHandler for RenderApp<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !self.renderer.is_initialized() {
            self.renderer.initialize(&mut self.window_attributes, event_loop).unwrap();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        use WindowEvent::*;
        match event {
            Resized(size) => {
                self.renderer.gpu_mut().surface_mut().resize(size);
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
                self.renderer.draw_frame();
                self.renderer.window.as_ref().unwrap().request_redraw();
            }
            _ => {}
        }
    }
}

impl Default for RenderApp<'_> {
    fn default() -> Self {
        Self {
            renderer: RendererWindow::default(),
            window_attributes: Some(WindowAttributes::default()
                .with_title("")
                .with_inner_size(LogicalSize::new(720, 480))
            ),
        }
    }
}

impl RenderApp<'_> {
    pub fn with_window_attributes(mut self, f: impl FnOnce(WindowAttributes) -> WindowAttributes) -> Self {
        self.window_attributes = Some(f(self.window_attributes.take().unwrap_or_default()));
        self
    }

    pub fn with_stage_task<T: Task + 'static>(self) -> Self {
        self.with_stage(|s| s.with_task::<T>())
    }

    pub fn with_stage(mut self, f: impl FnOnce(Stage) -> Stage) -> Self {
        self.renderer.stages_mut().add_stage(f(Stage::default()));
        self
    }

    pub fn run(mut self) -> Result<(), error::Error> {
        Ok(EventLoop::new()?.run_app(&mut self)?)
    }
}

impl RenderApp<'_> {
    pub fn stateless() -> Self {
        Self::default()
    }
}