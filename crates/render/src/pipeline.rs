//! Рендер пайплайны

pub use wgpu::{
    PipelineLayout, RenderPipeline, PipelineLayoutDescriptor, RenderPipelineDescriptor,
    VertexState, FragmentState, PrimitiveState, DepthStencilState, MultisampleState,
    PrimitiveTopology, PolygonMode, CullMode, FrontFace, CompareFunction, TextureFormat,
    BlendState, BlendComponent, BlendFactor, BlendOperation, ColorWrites,
};

use wgpu::{
    PipelineLayout, RenderPipeline, PipelineLayoutDescriptor, RenderPipelineDescriptor,
    VertexState, FragmentState, PrimitiveState, DepthStencilState, MultisampleState,
    PrimitiveTopology, PolygonMode, CullMode, FrontFace, CompareFunction, TextureFormat,
    BlendState, BlendComponent, BlendFactor, BlendOperation, ColorWrites,
};

/// Хендл пайплайна
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PipelineHandle(pub u64);

/// Описание вершинного буфера
#[derive(Debug, Clone)]
pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

impl VertexBufferLayout {
    pub fn from_wgpu(layout: wgpu::VertexBufferLayout) -> Self {
        Self {
            array_stride: layout.array_stride,
            step_mode: match layout.step_mode {
                wgpu::VertexStepMode::Vertex => VertexStepMode::Vertex,
                wgpu::VertexStepMode::Instance => VertexStepMode::Instance,
            },
            attributes: layout.attributes.iter().map(|a| VertexAttribute {
                format: a.format,
                offset: a.offset,
                shader_location: a.shader_location,
            }).collect(),
        }
    }
}

/// Режим шага вершин
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

/// Атрибут вершины
#[derive(Debug, Clone, Copy)]
pub struct VertexAttribute {
    pub format: wgpu::VertexFormat,
    pub offset: u64,
    pub shader_location: u32,
}

/// Описание рендер пайплайна
#[derive(Debug, Clone)]
pub struct RenderPipelineDescriptor2 {
    pub label: Option<String>,
    pub layout: Option<PipelineLayout>,
    pub vertex: VertexState2,
    pub fragment: Option<FragmentState2>,
    pub primitive: PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: MultisampleState,
    pub multiview: Option<std::num::NonZeroU32>,
}

/// Состояние вершинного шейдера
#[derive(Debug, Clone)]
pub struct VertexState2 {
    pub shader: super::ShaderHandle,
    pub entry_point: String,
    pub buffers: Vec<VertexBufferLayout>,
    pub constants: std::collections::HashMap<String, f64>,
}

/// Состояние фрагментного шейдера
#[derive(Debug, Clone)]
pub struct FragmentState2 {
    pub shader: super::ShaderHandle,
    pub entry_point: String,
    pub targets: Vec<Option<ColorTargetState>>,
    pub constants: std::collections::HashMap<String, f64>,
}

/// Состояние целевого буфера цвета
#[derive(Debug, Clone)]
pub struct ColorTargetState {
    pub format: TextureFormat,
    pub blend: Option<BlendState>,
    pub write_mask: wgpu::ColorWrites,
}

impl Default for ColorTargetState {
    fn default() -> Self {
        Self {
            format: TextureFormat::Rgba8UnormSrgb,
            blend: Some(BlendState::ALPHA_BLENDING),
            write_mask: ColorWrites::ALL,
        }
    }
}

/// Предустановленные конфигурации пайплайнов
pub struct PipelinePresets;

impl PipelinePresets {
    /// PBR opaque материал
    pub fn pbr_opaque(format: TextureFormat, depth_format: TextureFormat) -> RenderPipelineDescriptor2 {
        RenderPipelineDescriptor2 {
            label: Some("PBR Opaque".to_string()),
            layout: None,
            vertex: VertexState2 {
                shader: super::ShaderHandle(0), // Will be set
                entry_point: "vs_main".to_string(),
                buffers: vec![],
                constants: std::collections::HashMap::new(),
            },
            fragment: Some(FragmentState2 {
                shader: super::ShaderHandle(0),
                entry_point: "fs_main".to_string(),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                constants: std::collections::HashMap::new(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(CullMode::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
        }
    }

    /// PBR с прозрачностью
    pub fn pbr_transparent(format: TextureFormat, depth_format: TextureFormat) -> RenderPipelineDescriptor2 {
        let mut desc = Self::pbr_opaque(format, depth_format);
        desc.label = Some("PBR Transparent".to_string());
        if let Some(ref mut frag) = desc.fragment {
            for target in frag.targets.iter_mut().flatten() {
                target.blend = Some(BlendState::ALPHA_BLENDING);
            }
        }
        desc.depth_stencil.as_mut().unwrap().depth_write_enabled = false;
        desc
    }

    /// Unlit материал
    pub fn unlit(format: TextureFormat, depth_format: TextureFormat) -> RenderPipelineDescriptor2 {
        let mut desc = Self::pbr_opaque(format, depth_format);
        desc.label = Some("Unlit".to_string());
        // Simpler vertex/fragment shaders would be used
        desc
    }

    /// Wireframe
    pub fn wireframe(format: TextureFormat, depth_format: TextureFormat) -> RenderPipelineDescriptor2 {
        let mut desc = Self::pbr_opaque(format, depth_format);
        desc.label = Some("Wireframe".to_string());
        desc.primitive.polygon_mode = PolygonMode::Line;
        desc.primitive.cull_mode = None;
        desc
    }

    /// Постобработка
    pub fn post_process(format: TextureFormat) -> RenderPipelineDescriptor2 {
        RenderPipelineDescriptor2 {
            label: Some("Post Process".to_string()),
            layout: None,
            vertex: VertexState2 {
                shader: super::ShaderHandle(0),
                entry_point: "vs_main".to_string(),
                buffers: vec![],
                constants: std::collections::HashMap::new(),
            },
            fragment: Some(FragmentState2 {
                shader: super::ShaderHandle(0),
                entry_point: "fs_main".to_string(),
                targets: vec![Some(ColorTargetState {
                    format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                constants: std::collections::HashMap::new(),
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        }
    }
}

/// Построитель пайплайна
pub struct PipelineBuilder {
    descriptor: RenderPipelineDescriptor2,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self {
            descriptor: RenderPipelineDescriptor2 {
                label: None,
                layout: None,
                vertex: VertexState2 {
                    shader: ShaderHandle(0),
                    entry_point: "vs_main".to_string(),
                    buffers: vec![],
                    constants: std::collections::HashMap::new(),
                },
                fragment: None,
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(CullMode::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview: None,
            }
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.descriptor.label = Some(label.into());
        self
    }

    pub fn with_vertex_shader(mut self, shader: ShaderHandle, entry_point: impl Into<String>) -> Self {
        self.descriptor.vertex.shader = shader;
        self.descriptor.vertex.entry_point = entry_point.into();
        self
    }

    pub fn with_fragment_shader(mut self, shader: ShaderHandle, entry_point: impl Into<String>) -> Self {
        self.descriptor.fragment = Some(FragmentState2 {
            shader,
            entry_point: entry_point.into(),
            targets: vec![Some(ColorTargetState::default())],
            constants: hashbrown::HashMap::new(),
        });
        self
    }

    pub fn with_color_target(mut self, format: TextureFormat, blend: Option<BlendState>) -> Self {
        if let Some(ref mut frag) = self.descriptor.fragment {
            frag.targets = vec![Some(ColorTargetState {
                format,
                blend,
                write_mask: ColorWrites::ALL,
            })];
        }
        self
    }

    pub fn with_depth_stencil(mut self, format: TextureFormat, write: bool, compare: CompareFunction) -> Self {
        self.descriptor.depth_stencil = Some(DepthStencilState {
            format,
            depth_write_enabled: write,
            depth_compare: compare,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        });
        self
    }

    pub fn with_primitive_topology(mut self, topology: PrimitiveTopology) -> Self {
        self.descriptor.primitive.topology = topology;
        self
    }

    pub fn with_cull_mode(mut self, cull_mode: Option<CullMode>) -> Self {
        self.descriptor.primitive.cull_mode = cull_mode;
        self
    }

    pub fn with_polygon_mode(mut self, mode: PolygonMode) -> Self {
        self.descriptor.primitive.polygon_mode = mode;
        self
    }

    pub fn with_multisample(mut self, count: u32, mask: u64, alpha_to_coverage: bool) -> Self {
        self.descriptor.multisample = MultisampleState {
            count,
            mask,
            alpha_to_coverage_enabled: alpha_to_coverage,
        };
        self
    }

    pub fn build(self) -> RenderPipelineDescriptor2 {
        self.descriptor
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}
