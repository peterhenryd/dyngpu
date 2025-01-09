use crate::gpu;
use wgpu::*;

pub struct RenderPipelineBuilder<'w, 'a> {
    pub ctx: gpu::Context<'w>,
    pub label: Option<&'a str>,
    pub primitive_state: PrimitiveState,
    pub vertex_state: VertexStateIntermediate<'a>,
    pub fragment_state: Option<FragmentStateIntermediate<'a>>,
    pub pipeline_layout: PipelineLayoutIntermediate<'a>,
}

impl<'a> RenderPipelineBuilder<'_, 'a> {
    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn unlabeled(mut self) -> Self {
        self.label = None;
        self
    }
}

pub struct VertexStateIntermediate<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: Option<&'a str>,
    pub compilation_options: PipelineCompilationOptions<'a>,
    pub buffers: Vec<VertexBufferLayout<'a>>,
}

impl<'a> VertexStateIntermediate<'a> {
    pub fn new(shader: &'a ShaderModule) -> Self {
        VertexStateIntermediate {
            module: shader,
            entry_point: Some("vs"),
            compilation_options: PipelineCompilationOptions::default(),
            buffers: vec![],
        }
    }

    pub fn build(&'a self) -> VertexState<'a> {
        VertexState {
            module: self.module,
            entry_point: self.entry_point,
            compilation_options: self.compilation_options.clone(),
            buffers: self.buffers.as_slice()
        }
    }
}

impl<'a> RenderPipelineBuilder<'_, 'a> {
    pub fn vert(mut self, shader: &'a ShaderModule, entry_point: &'a str) -> Self {
        self.vertex_state.module = shader;
        self.vertex_state.entry_point = Some(entry_point);
        self
    }

    pub fn vert_entry(mut self, entry_point: &'a str) -> Self {
        self.vertex_state.entry_point = Some(entry_point);
        self
    }

    pub fn vert_buffer(mut self, buffers: VertexBufferLayout<'a>) -> Self {
        self.vertex_state.buffers.push(buffers);
        self
    }
}


pub struct FragmentStateIntermediate<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: Option<&'a str>,
    pub compilation_options: PipelineCompilationOptions<'a>,
    pub targets: Vec<Option<ColorTargetState>>,
}

impl<'a> FragmentStateIntermediate<'a> {
    pub fn build(&'a self) -> FragmentState<'a> {
        FragmentState {
            module: self.module,
            entry_point: self.entry_point,
            compilation_options: self.compilation_options.clone(),
            targets: &self.targets,
        }
    }
}

impl<'a> RenderPipelineBuilder<'_, 'a> {
    pub fn frag(mut self, shader: &'a ShaderModule, entry_point: &'a str) -> Self {
        if let Some(state) = &mut self.fragment_state {
            state.module = shader;
            state.entry_point = Some(entry_point);
            return self
        }

        self.fragment_state = Some(FragmentStateIntermediate {
            module: shader,
            entry_point: Some(entry_point),
            compilation_options: Default::default(),
            targets: Self::DEFAULT_TARGETS.to_vec(),
        });
        self
    }

    pub fn frag_entry(mut self, entry_point: &'a str) -> Self {
        let Some(state) = &mut self.fragment_state else { return self };
        state.entry_point = Some(entry_point);

        self
    }

    pub fn default_frag(self) -> Self {
        let shader = self.vertex_state.module;
        self.frag(shader, "fs")
    }
}

#[derive(Default)]
pub struct PipelineLayoutIntermediate<'a> {
    pub label: Option<&'a str>,
    pub bind_group_layouts: Vec<&'a BindGroupLayout>,
    pub push_constant_ranges: Vec<PushConstantRange>,
}

impl<'a> RenderPipelineBuilder<'_, 'a> {
    pub fn bind_group(mut self, bind_group: &'a BindGroupLayout) -> Self {
        self.pipeline_layout.bind_group_layouts.push(bind_group);
        self
    }
}

impl<'a> RenderPipelineBuilder<'_, 'a> {
    const DEFAULT_TARGETS: &'static [Option<ColorTargetState>] = &[Some(ColorTargetState {
        format: TextureFormat::Bgra8UnormSrgb,
        blend: Some(BlendState::REPLACE),
        write_mask: ColorWrites::ALL,
    })];

    pub fn finish(&'a self) -> RenderPipeline {
        let layout = self.ctx.device().create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: self.pipeline_layout.bind_group_layouts.as_slice(),
            push_constant_ranges: self.pipeline_layout.push_constant_ranges.as_slice(),
            label: self.pipeline_layout.label,
        });

        self.ctx.device().create_render_pipeline(&RenderPipelineDescriptor {
            layout: Some(&layout),
            vertex: self.vertex_state.build(),
            primitive: self.primitive_state,
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: self.fragment_state.as_ref().map(|x| x.build()),
            multiview: None,
            cache: None,
            label: self.label,
        })
    }
}

impl<'w> gpu::Context<'w> {
    pub fn build_pipeline<'a>(&self, shader: &'a ShaderModule) -> RenderPipelineBuilder<'w, 'a> {
        self.build_vert_pipeline(shader).default_frag()
    }

    pub fn build_vert_pipeline<'a>(&self, shader: &'a ShaderModule) -> RenderPipelineBuilder<'w, 'a> {
        RenderPipelineBuilder {
            ctx: self.clone(),
            label: None,
            vertex_state: VertexStateIntermediate::new(shader),
            primitive_state: PrimitiveState::default(),
            fragment_state: None,
            pipeline_layout: PipelineLayoutIntermediate::default(),
        }
    }
}