use crate::gpu;
use wgpu::*;

pub struct BindGroupBuilder<'w, 'a> {
    ctx: gpu::Context<'w>,
    bind_group_layout: Option<&'a BindGroupLayout>,
    bind_group_layout_entries: Vec<BindGroupLayoutEntry>,
    bind_group_entries: Vec<BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'_, 'a> {
    pub fn override_layout(mut self, layout: &'a BindGroupLayout) -> Self {
        self.bind_group_layout = Some(layout);
        self
    }

    pub fn uniform(mut self, buffer: &'a Buffer, visibility: ShaderStages) -> Self {
        let binding = self.bind_group_layout_entries.len() as u32;
        self.bind_group_entries.push(BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        });
        if self.bind_group_layout.is_none() {
            self.bind_group_layout_entries.push(BindGroupLayoutEntry {
                binding,
                visibility,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            });
        }
        self
    }

    pub fn finish_with_layout(self) -> (BindGroup, Option<BindGroupLayout>) {
        let mut bind_group_layout = None;
        let layout = match self.bind_group_layout {
            None => {
                bind_group_layout = Some(self.ctx.device().create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &self.bind_group_layout_entries,
                }));
                bind_group_layout.as_ref().unwrap()
            }
            Some(x) => x,
        };

        let bind_group = self.ctx.device().create_bind_group(&BindGroupDescriptor {
            layout,
            entries: &self.bind_group_entries,
            label: None,
        });

        (bind_group, bind_group_layout)
    }
}

impl<'w> gpu::Context<'w> {
    pub fn build_bind_group<'a>(&self) -> BindGroupBuilder<'w, 'a> {
        BindGroupBuilder {
            ctx: self.clone(),
            bind_group_layout: None,
            bind_group_layout_entries: vec![],
            bind_group_entries: vec![],
        }
    }
}