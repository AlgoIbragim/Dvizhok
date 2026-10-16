//! # Lumina Render
//! 
//! Современный рендерер на базе wgpu с поддержкой:
//! - PBR материалов
//! - Deferred и Forward рендеринга
//! - HDR и постпроцессинга
//! - Вolumetric освещения
//! - Теней (CSM, VSM)
//! - GPU culling

pub mod context;
pub mod mesh;
pub mod material;
pub mod texture;
pub mod camera;
pub mod light;
pub mod shader;
pub mod pipeline;
pub mod render_graph;
pub mod render_pass;
pub mod resource;
pub mod queue;
pub mod bind_group;
pub mod sampler;
pub mod buffer;
pub mod uniform;
pub mod shadow;
pub mod post_process;
pub mod gizmos;

pub use context::*;
pub use mesh::*;
pub use material::*;
pub use texture::*;
pub use camera::*;
pub use light::*;
pub use shader::*;
pub use pipeline::*;
pub use render_graph::*;
pub use render_pass::*;
pub use resource::*;
pub use queue::*;
pub use bind_group::*;
pub use sampler::*;
pub use buffer::*;
pub use uniform::*;

/// Версия рендерера
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude
pub mod prelude {
    pub use crate::{
        RenderContext, RenderDevice, RenderQueue,
        Mesh, Vertex, IndexFormat, PrimitiveTopology,
        Material, MaterialHandle, StandardMaterial,
        Texture, TextureHandle, TextureFormat, TextureUsage,
        Camera, CameraProjection, PerspectiveProjection, OrthographicProjection,
        Light, LightType, DirectionalLight, PointLight, SpotLight,
        Shader, ShaderSource, ShaderStage,
        RenderPipeline, PipelineLayout,
        RenderGraph, RenderPass, RenderPassDescriptor,
        BindGroup, BindGroupLayout, BindGroupEntry,
        Buffer, BufferUsage, BufferDescriptor,
        Sampler, SamplerDescriptor, FilterMode, AddressMode,
        UniformBuffer, DynamicUniformBuffer,
        RenderQueue, QueueItem, SortKey,
        ResourceManager, Handle,
    };
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("WGPU error: {0}")]
    Wgpu(String),
    
    #[error("Shader compilation error: {0}")]
    ShaderCompilation(String),
    
    #[error("Pipeline creation error: {0}")]
    PipelineCreation(String),
    
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Buffer error: {0}")]
    Buffer(String),
    
    #[error("Texture error: {0}")]
    Texture(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type RenderResult<T> = Result<T, RenderError>;

/// Тип рендеринга
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderMode {
    /// Forward рендеринг
    Forward,
    /// Deferred рендеринг
    Deferred,
    /// Forward+ рендеринг
    ForwardPlus,
}

/// Качество рендеринга
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderQuality {
    Low,
    Medium,
    High,
    Ultra,
}

impl RenderQuality {
    /// Получить масштаб рендера
    pub fn render_scale(&self) -> f32 {
        match self {
            RenderQuality::Low => 0.75,
            RenderQuality::Medium => 1.0,
            RenderQuality::High => 1.25,
            RenderQuality::Ultra => 1.5,
        }
    }

    /// Получить качество теней
    pub fn shadow_quality(&self) -> u32 {
        match self {
            RenderQuality::Low => 1024,
            RenderQuality::Medium => 2048,
            RenderQuality::High => 4096,
            RenderQuality::Ultra => 8192,
        }
    }

    /// Включить SSAO
    pub fn ssao_enabled(&self) -> bool {
        matches!(self, RenderQuality::High | RenderQuality::Ultra)
    }

    /// Включить SSR
    pub fn ssr_enabled(&self) -> bool {
        matches!(self, RenderQuality::Ultra)
    }
}