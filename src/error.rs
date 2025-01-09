#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CreateSurface(#[from] wgpu::CreateSurfaceError),
    #[error("failed to request adapter")]
    RequestAdapter,
    #[error(transparent)]
    RequestDevice(#[from] wgpu::RequestDeviceError),
    #[cfg(feature = "winit")]
    #[error(transparent)]
    Os(#[from] winit::error::OsError),
    #[cfg(feature = "winit")]
    #[error(transparent)]
    EventLoop(#[from] winit::error::EventLoopError),
}