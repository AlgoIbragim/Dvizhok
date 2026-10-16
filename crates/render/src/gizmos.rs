//! Gizmos - визуальная отладка

use glam::{Vec2, Vec3, Quat, Mat4};
use lumina_core::Color;

/// Gizmo для отрисовки отладочной графики
#[derive(Debug, Clone, PartialEq)]
pub struct Gizmo {
    /// Позиция
    pub position: Vec3,
    /// Поворот
    pub rotation: Quat,
    /// Масштаб
    pub scale: Vec3,
    /// Цвет
    pub color: Color,
    /// Тип gizmo
    pub shape: GizmoShape,
    /// Время жизни (None = бесконечно)
    pub lifetime: Option<f32>,
    /// Пространство координат
    pub space: GizmoSpace,
}

impl Default for Gizmo {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            color: Color::WHITE,
            shape: GizmoShape::Line(Vec3::ZERO, Vec3::X),
            lifetime: None,
            space: GizmoSpace::World,
        }
    }
}

impl Gizmo {
    /// Создать линию
    pub fn line(start: Vec3, end: Vec3, color: Color) -> Self {
        Self {
            position: start,
            color,
            shape: GizmoShape::Line(Vec3::ZERO, end - start),
            ..Default::default()
        }
    }

    /// Создать стрелку
    pub fn arrow(start: Vec3, direction: Vec3, color: Color) -> Self {
        Self {
            position: start,
            color,
            shape: GizmoShape::Arrow(direction),
            ..Default::default()
        }
    }

    /// Создать куб
    pub fn cube(position: Vec3, size: Vec3, color: Color) -> Self {
        Self {
            position,
            scale: size,
            color,
            shape: GizmoShape::Cube,
            ..Default::default()
        }
    }

    /// Создать сферу
    pub fn sphere(position: Vec3, radius: f32, color: Color) -> Self {
        Self {
            position,
            scale: Vec3::splat(radius),
            color,
            shape: GizmoShape::Sphere,
            ..Default::default()
        }
    }

    /// Создать цилиндр
    pub fn cylinder(position: Vec3, radius: f32, height: f32, color: Color) -> Self {
        Self {
            position,
            scale: Vec3::new(radius, height, radius),
            color,
            shape: GizmoShape::Cylinder,
            ..Default::default()
        }
    }

    /// Создать капсулу
    pub fn capsule(position: Vec3, radius: f32, height: f32, color: Color) -> Self {
        Self {
            position,
            scale: Vec3::new(radius, height, radius),
            color,
            shape: GizmoShape::Capsule,
            ..Default::default()
        }
    }

    /// Создать конус
    pub fn cone(position: Vec3, radius: f32, height: f32, color: Color) -> Self {
        Self {
            position,
            scale: Vec3::new(radius, height, radius),
            color,
            shape: GizmoShape::Cone,
            ..Default::default()
        }
    }

    /// Создать тор
    pub fn torus(position: Vec3, major_radius: f32, minor_radius: f32, color: Color) -> Self {
        Self {
            position,
            scale: Vec3::new(major_radius, minor_radius, major_radius),
            color,
            shape: GizmoShape::Torus,
            ..Default::default()
        }
    }

    /// Создать круг
    pub fn circle(position: Vec3, normal: Vec3, radius: f32, color: Color) -> Self {
        Self {
            position,
            rotation: Quat::from_rotation_arc(Vec3::Y, normal),
            scale: Vec3::splat(radius),
            color,
            shape: GizmoShape::Circle,
            ..Default::default()
        }
    }

    /// Создать сетку
    pub fn grid(position: Vec3, size: f32, divisions: u32, color: Color) -> Self {
        Self {
            position,
            scale: Vec3::splat(size),
            color,
            shape: GizmoShape::Grid { divisions },
            ..Default::default()
        }
    }

    /// Создать оси координат
    pub fn axes(position: Vec3, size: f32) -> Vec<Self> {
        vec![
            Self::arrow(position, Vec3::X * size, Color::RED),
            Self::arrow(position, Vec3::Y * size, Color::GREEN),
            Self::arrow(position, Vec3::Z * size, Color::BLUE),
        ]
    }

    /// Создать bounding box
    pub fn bounding_box(min: Vec3, max: Vec3, color: Color) -> Vec<Self> {
        let corners = [
            Vec3::new(min.x, min.y, min.z),
            Vec3::new(max.x, min.y, min.z),
            Vec3::new(max.x, max.y, min.z),
            Vec3::new(min.x, max.y, min.z),
            Vec3::new(min.x, min.y, max.z),
            Vec3::new(max.x, min.y, max.z),
            Vec3::new(max.x, max.y, max.z),
            Vec3::new(min.x, max.y, max.z),
        ];

        let edges = [
            (0, 1), (1, 2), (2, 3), (3, 0), // bottom face
            (4, 5), (5, 6), (6, 7), (7, 4), // top face
            (0, 4), (1, 5), (2, 6), (3, 7), // vertical edges
        ];

        edges
            .iter()
            .map(|(a, b)| Gizmo::line(corners[*a], corners[*b], color))
            .collect()
    }

    /// Создать frustum
    pub fn frustum(projection: Mat4, view: Mat4, color: Color) -> Vec<Self> {
        let inv_view_proj = (projection * view).inverse();
        
        let ndc_corners = [
            Vec3::new(-1.0, -1.0, 0.0), // near bottom left
            Vec3::new( 1.0, -1.0, 0.0), // near bottom right
            Vec3::new( 1.0,  1.0, 0.0), // near top right
            Vec3::new(-1.0,  1.0, 0.0), // near top left
            Vec3::new(-1.0, -1.0, 1.0), // far bottom left
            Vec3::new( 1.0, -1.0, 1.0), // far bottom right
            Vec3::new( 1.0,  1.0, 1.0), // far top right
            Vec3::new(-1.0,  1.0, 1.0), // far top left
        ];

        let world_corners: Vec<Vec3> = ndc_corners
            .iter()
            .map(|&c| {
                let world = inv_view_proj * Vec4::new(c.x, c.y, c.z, 1.0);
                world.truncate() / world.w
            })
            .collect();

        let edges = [
            // Near face
            (0, 1), (1, 2), (2, 3), (3, 0),
            // Far face
            (4, 5), (5, 6), (6, 7), (7, 4),
            // Connecting edges
            (0, 4), (1, 5), (2, 6), (3, 7),
        ];

        edges
            .iter()
            .map(|(a, b)| Gizmo::line(world_corners[*a], world_corners[*b], color))
            .collect()
    }
}

/// Форма gizmo
#[derive(Debug, Clone, PartialEq)]
pub enum GizmoShape {
    Line(Vec3, Vec3),
    Arrow(Vec3),
    Cube,
    Sphere,
    Cylinder,
    Capsule,
    Cone,
    Torus,
    Circle,
    Grid { divisions: u32 },
}

/// Пространство координат gizmo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GizmoSpace {
    #[default]
    World,
    Local,
    View,
}

/// Менеджер gizmos
#[derive(Debug, Default)]
pub struct GizmoManager {
    gizmos: Vec<Gizmo>,
}

impl GizmoManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, gizmo: Gizmo) {
        self.gizmos.push(gizmo);
    }

    pub fn add_batch(&mut self, gizmos: impl IntoIterator<Item = Gizmo>) {
        self.gizmos.extend(gizmos);
    }

    pub fn clear(&mut self) {
        self.gizmos.clear();
    }

    pub fn update(&mut self, dt: f32) {
        self.gizmos.retain(|g| {
            if let Some(lifetime) = g.lifetime {
                lifetime > 0.0
            } else {
                true
            }
        });

        for g in &mut self.gizmos {
            if let Some(ref mut lifetime) = g.lifetime {
                *lifetime -= dt;
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Gizmo> {
        self.gizmos.iter()
    }
}
