//! # Mesh - Меши
//! 
//! Вершины, индексы, примитивы.

use wgpu::{Buffer, BufferUsages};
use bytemuck::{Pod, Zeroable};
use glam::*;

/// Вершина с позицией, нормалью, UV и цветом
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0, 0.0, 1.0],
            uv: [0.0; 2],
            color: [1.0; 4],
        }
    }
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2, color: Color) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            uv: uv.to_array(),
            color: color.to_vec4().to_array(),
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Normal
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // UV
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Color
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// Меш
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub topology: PrimitiveTopology,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            topology: PrimitiveTopology::TriangleList,
        }
    }
}

impl Mesh {
    pub fn new(topology: PrimitiveTopology) -> Self {
        Self {
            topology,
            ..Default::default()
        }
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = indices;
        self
    }

    /// Куб
    pub fn cube(size: f32) -> Self {
        let s = size * 0.5;
        let vertices = vec![
            // Front face
            Vertex::new(Vec3::new(-s, -s, s), Vec3::Z, Vec2::new(0.0, 1.0), Color::WHITE),
            Vertex::new(Vec3::new(s, -s, s), Vec3::Z, Vec2::new(1.0, 1.0), Color::WHITE),
            Vertex::new(Vec3::new(s, s, s), Vec3::Z, Vec2::new(1.0, 0.0), Color::WHITE),
            Vertex::new(Vec3::new(-s, s, s), Vec3::Z, Vec2::new(0.0, 0.0), Color::WHITE),
            // ... (other faces)
        ];

        let indices = vec![
            // Front
            0, 1, 2, 0, 2, 3,
            // ... (other faces)
        ];

        Self::new(PrimitiveTopology::TriangleList)
            .with_vertices(vertices)
            .with_indices(indices)
    }

    /// Плоскость
    pub fn plane(width: f32, height: f32) -> Self {
        let w = width * 0.5;
        let h = height * 0.5;

        let vertices = vec![
            Vertex::new(Vec3::new(-w, 0.0, -h), Vec3::Y, Vec2::new(0.0, 0.0), Color::WHITE),
            Vertex::new(Vec3::new(w, 0.0, -h), Vec3::Y, Vec2::new(1.0, 0.0), Color::WHITE),
            Vertex::new(Vec3::new(w, 0.0, h), Vec3::Y, Vec2::new(1.0, 1.0), Color::WHITE),
            Vertex::new(Vec3::new(-w, 0.0, h), Vec3::Y, Vec2::new(0.0, 1.0), Color::WHITE),
        ];

        let indices = vec![0, 2, 1, 0, 3, 2];

        Self::new(PrimitiveTopology::TriangleList)
            .with_vertices(vertices)
            .with_indices(indices)
    }

    /// Сфера
    pub fn sphere(radius: f32, segments: u32, rings: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for ring in 0..=rings {
            let v = ring as f32 / rings as f32;
            let phi = v * std::f32::consts::PI;

            for seg in 0..=segments {
                let u = seg as f32 / segments as f32;
                let theta = u * 2.0 * std::f32::consts::PI;

                let pos = Vec3::new(
                    radius * phi.sin() * theta.cos(),
                    radius * phi.cos(),
                    radius * phi.sin() * theta.sin(),
                );

                let normal = pos.normalize();
                let uv = Vec2::new(u, v);

                vertices.push(Vertex::new(pos, normal, uv, Color::WHITE));
            }
        }

        // Generate indices
        for ring in 0..rings {
            for seg in 0..segments {
                let a = ring * (segments + 1) + seg;
                let b = a + segments + 1;

                indices.push(a);
                indices.push(b);
                indices.push(a + 1);

                indices.push(b);
                indices.push(b + 1);
                indices.push(a + 1);
            }
        }

        Self::new(PrimitiveTopology::TriangleList)
            .with_vertices(vertices)
            .with_indices(indices)
    }

    /// Вычислить AABB
    pub fn compute_aabb(&self) -> lumina_core::AABB {
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for vertex in &self.vertices {
            let pos = Vec3::from_slice(&vertex.position);
            min = min.min(pos);
            max = max.max(pos);
        }

        lumina_core::AABB::new(min, max)
    }

    /// Перевернуть нормали
    pub fn flip_normals(&mut self) {
        for vertex in &mut self.vertices {
            vertex.normal[0] = -vertex.normal[0];
            vertex.normal[1] = -vertex.normal[1];
            vertex.normal[2] = -vertex.normal[2];
        }
    }

    /// Инвертировать индексы (flip winding)
    pub fn flip_winding(&mut self) {
        for chunk in self.indices.chunks_exact_mut(3) {
            chunk.swap(1, 2);
        }
    }
}

/// Типология примитивов
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

impl Default for PrimitiveTopology {
    fn default() -> Self {
        Self::TriangleList
    }
}

impl From<PrimitiveTopology> for wgpu::PrimitiveTopology {
    fn from(topology: PrimitiveTopology) -> Self {
        match topology {
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
            PrimitiveTopology::LineList => wgpu::PrimitiveTopology::LineList,
            PrimitiveTopology::LineStrip => wgpu::PrimitiveTopology::LineStrip,
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
        }
    }
}