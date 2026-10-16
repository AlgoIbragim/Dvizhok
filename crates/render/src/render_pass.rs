//! Рендер проходы

use wgpu::{
    RenderPass as WgpuRenderPass, RenderPassDescriptor, RenderPassColorAttachment,
    RenderPassDepthStencilAttachment, LoadOp, StoreOp, Operations, Color,
};

/// Дескриптор рендер прохода
#[derive(Debug, Clone)]
pub struct RenderPassDesc {
    pub label: Option<String>,
    pub color_attachments: Vec<Option<RenderPassColorAttachmentDesc>>,
    pub depth_stencil_attachment: Option<RenderPassDepthStencilAttachmentDesc>,
}

/// Дескриптор цветного аттача
#[derive(Debug, Clone)]
pub struct RenderPassColorAttachmentDesc {
    pub view: wgpu::TextureView,
    pub resolve_target: Option<wgpu::TextureView>,
    pub ops: Operations<Color>,
}

/// Дескриптор depth-stencil аттача
#[derive(Debug, Clone)]
pub struct RenderPassDepthStencilAttachmentDesc {
    pub view: wgpu::TextureView,
    pub depth_ops: Option<Operations<f32>>,
    pub stencil_ops: Option<Operations<u32>>,
}

impl RenderPassDesc {
    pub fn new() -> Self {
        Self {
            label: None,
            color_attachments: Vec::new(),
            depth_stencil_attachment: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_color_attachment(
        mut self,
        view: wgpu::TextureView,
        load_op: LoadOp<Color>,
        store_op: StoreOp,
    ) -> Self {
        self.color_attachments.push(Some(RenderPassColorAttachmentDesc {
            view,
            resolve_target: None,
            ops: Operations {
                load: load_op,
                store: store_op,
            },
        }));
        self
    }

    pub fn with_clear_color(mut self, view: wgpu::TextureView, color: Color) -> Self {
        self.color_attachments.push(Some(RenderPassColorAttachmentDesc {
            view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(color),
                store: StoreOp::Store,
            },
        }));
        self
    }

    pub fn with_depth_attachment(
        mut self,
        view: wgpu::TextureView,
        load_op: LoadOp<f32>,
        store_op: StoreOp,
    ) -> Self {
        self.depth_stencil_attachment = Some(RenderPassDepthStencilAttachmentDesc {
            view,
            depth_ops: Some(Operations {
                load: load_op,
                store: store_op,
            }),
            stencil_ops: None,
        });
        self
    }

    pub fn with_clear_depth(mut self, view: wgpu::TextureView, depth: f32) -> Self {
        self.depth_stencil_attachment = Some(RenderPassDepthStencilAttachmentDesc {
            view,
            depth_ops: Some(Operations {
                load: LoadOp::Clear(depth),
                store: StoreOp::Store,
            }),
            stencil_ops: None,
        });
        self
    }

    pub fn build(&self) -> RenderPassDescriptor {
        RenderPassDescriptor {
            label: self.label.as_deref(),
            color_attachments: &self.color_attachments.iter()
                .map(|a| a.as_ref().map(|a| RenderPassColorAttachment {
                    view: &a.view,
                    resolve_target: a.resolve_target.as_ref(),
                    ops: a.ops.clone(),
                }))
                .collect::<Vec<_>>(),
            depth_stencil_attachment: self.depth_stencil_attachment.as_ref().map(|a| RenderPassDepthStencilAttachment {
                view: &a.view,
                depth_ops: a.depth_ops.clone(),
                stencil_ops: a.stencil_ops.clone(),
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        }
    }
}

impl Default for RenderPassDesc {
    fn default() -> Self {
        Self::new()
    }
}

/// Тип рендер прохода
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PassType {
    Shadow,
    DepthPrepass,
    GBuffer,
    Lighting,
    Transparent,
    PostProcess,
    UI,
    Custom(&'static str),
}

/// Информация о рендер проходе
#[derive(Debug, Clone)]
pub struct PassInfo {
    pub name: String,
    pub pass_type: PassType,
    pub dependencies: Vec<String>,
}
