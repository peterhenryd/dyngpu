use crate::Device;
use bytemuck::NoUninit;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{Buffer, BufferDescriptor, BufferUsages};

pub struct BufferBuilder<'a> {
    device: Device,
    label: Option<&'a str>,
    contents: Option<&'a [u8]>,
    size: u64,
    usage: BufferUsages,
}

impl<'a> BufferBuilder<'a> {
    pub fn size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn contents(mut self, contents: &'a [u8]) -> Self {
        self.contents = Some(contents);
        self
    }

    pub fn contents_slice(self, contents: &'a [impl NoUninit]) -> Self {
        self.contents(bytemuck::cast_slice(contents))
    }

    pub fn finish(self) -> Buffer {
        if let Some(contents) = self.contents {
            self.device.0.device.create_buffer_init(&BufferInitDescriptor {
                label: self.label,
                contents,
                usage: self.usage,
            })
        } else {
            self.device.0.device.create_buffer(&BufferDescriptor {
                label: self.label,
                size: self.size,
                usage: self.usage,
                mapped_at_creation: true,
            })
        }
    }

    pub fn vert(mut self) -> Self {
        self.usage.insert(BufferUsages::VERTEX);
        self
    }

    pub fn uniform(mut self) -> Self {
        self.usage.insert(BufferUsages::UNIFORM);
        self
    }

    pub fn copy_dst(mut self) -> Self {
        self.usage.insert(BufferUsages::COPY_DST);
        self
    }
}

impl Device {
    pub fn build_buffer(&self) -> BufferBuilder<'_> {
        BufferBuilder {
            label: None,
            device: self.clone(),
            contents: None,
            size: 0,
            usage: BufferUsages::empty(),
        }
    }
}