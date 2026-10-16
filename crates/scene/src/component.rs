//! # Component - Компоненты сцены
//! 
//! Базовые компоненты для игровых объектов.

use lumina_core::*;
use lumina_render::*;

/// Компонент имени
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    pub name: String,
}

impl Default for Name {
    fn default() -> Self {
        Self {
            name: "Entity".to_string(),
        }
    }
}

impl Name {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
        }
    }
}

/// Компонент активности
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Active {
    pub active: bool,
}

impl Active {
    pub const ACTIVE: Self = Self { active: true };
    pub const INACTIVE: Self = Self { active: false };

    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

/// Компонент видимости
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Visible {
    pub visible: bool,
}

impl Visible {
    pub const VISIBLE: Self = Self { visible: true };
    pub const HIDDEN: Self = Self { visible: false };

    pub fn new(visible: bool) -> Self {
        Self { visible }
    }
}

/// Компонент слоя
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Layer {
    pub layer: u32,
}

impl Layer {
    pub const DEFAULT: Self = Self { layer: 0 };

    pub fn new(layer: u32) -> Self {
        Self { layer }
    }
}

/// Компонент тега
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag {
    pub value: u32,
}

impl Default for Tag {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl Tag {
    pub const NONE: Self = Self { value: 0 };

    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

/// Bundle компонентов для трансформации
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformBundle {
    pub local: Transform,
    pub global: GlobalTransform,
}

impl Default for TransformBundle {
    fn default() -> Self {
        Self {
            local: Transform::default(),
            global: GlobalTransform::default(),
        }
    }
}

impl TransformBundle {
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            local: transform,
            global: GlobalTransform::default(),
        }
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.local = transform;
        self
    }
}

/// Global transform (computed from hierarchy)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalTransform {
    pub matrix: Mat4,
}

impl Default for GlobalTransform {
    fn default() -> Self {
        Self {
            matrix: Mat4::IDENTITY,
        }
    }
}

impl GlobalTransform {
    pub fn from_matrix(matrix: Mat4) -> Self {
        Self { matrix }
    }

    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.matrix.transform_point3(point)
    }

    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.matrix.transform_vector3(vector)
    }

    pub fn rotation(&self) -> Quat {
        let (_scale, rotation, _translation) = self.matrix.to_scale_rotation_translation();
        rotation
    }

    pub fn translation(&self) -> Vec3 {
        self.matrix.w_axis.truncate()
    }

    pub fn scale(&self) -> Vec3 {
        let (scale, _rotation, _translation) = self.matrix.to_scale_rotation_translation();
        scale
    }
}

/// Bundle для рендеринга
#[derive(Debug, Clone)]
pub struct RenderBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<Material>,
    pub visible: Visible,
    pub layer: Layer,
}

impl Default for RenderBundle {
    fn default() -> Self {
        Self {
            mesh: Handle::default(),
            material: Handle::default(),
            visible: Visible::VISIBLE,
            layer: Layer::DEFAULT,
        }
    }
}

impl RenderBundle {
    pub fn with_mesh(mut self, mesh: Handle<Mesh>) -> Self {
        self.mesh = mesh;
        self
    }

    pub fn with_material(mut self, material: Handle<Material>) -> Self {
        self.material = material;
        self
    }

    pub fn with_visible(mut self, visible: Visible) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.layer = layer;
        self
    }
}

// Placeholder types
pub struct Handle<T>(std::marker::PhantomData<T>);
impl<T> Default for Handle<T> {
    fn default() -> Self { Self(std::marker::PhantomData) }
}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self { Self(std::marker::PhantomData) }
}

pub struct Mesh;
pub struct Material;