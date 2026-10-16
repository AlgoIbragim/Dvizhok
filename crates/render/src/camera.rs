//! # Camera - Камеры
//! 
//! Проекции камеры: перспективная, ортографическая.

use glam::*;

/// Компонент камеры
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    /// Проекция
    pub projection: CameraProjection,
    /// Ближняя плоскость
    pub near: f32,
    /// Дальняя плоскость
    pub far: f32,
    /// Viewport (x, y, width, height) - None = fullscreen
    pub viewport: Option<Vec4>,
    /// Флаг активности
    pub is_active: bool,
    /// Порядок рендеринга (меньше = раньше)
    pub order: i32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            projection: CameraProjection::Perspective(PerspectiveProjection::default()),
            near: 0.1,
            far: 1000.0,
            viewport: None,
            is_active: true,
            order: 0,
        }
    }
}

impl Camera {
    /// Создать перспективную камеру
    #[inline]
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            projection: CameraProjection::Perspective(PerspectiveProjection {
                fov_y,
                aspect,
            }),
            near,
            far,
            ..Default::default()
        }
    }

    /// Создать ортографическую камеру
    #[inline]
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self {
            projection: CameraProjection::Orthographic(OrthographicProjection {
                left,
                right,
                bottom,
                top,
            }),
            near,
            far,
            ..Default::default()
        }
    }

    /// Получить матрицу проекции
    #[inline]
    pub fn projection_matrix(&self) -> Mat4 {
        match self.projection {
            CameraProjection::Perspective(persp) => {
                Mat4::perspective_rh(persp.fov_y, persp.aspect, self.near, self.far)
            }
            CameraProjection::Orthographic(ortho) => {
                Mat4::orthographic_rh(
                    ortho.left,
                    ortho.right,
                    ortho.bottom,
                    ortho.top,
                    self.near,
                    self.far,
                )
            }
        }
    }

    /// Получить view матрицу из transform
    #[inline]
    pub fn view_matrix(transform: &lumina_core::Transform) -> Mat4 {
        // View matrix is inverse of camera transform
        transform.compute_matrix().inverse()
    }

    /// Получить view-projection матрицу
    #[inline]
    pub fn view_projection_matrix(&self, transform: &lumina_core::Transform) -> Mat4 {
        self.projection_matrix() * Self::view_matrix(transform)
    }

    /// Обновить aspect ratio
    #[inline]
    pub fn update_aspect(&mut self, aspect: f32) {
        if let CameraProjection::Perspective(ref mut p) = self.projection {
            p.aspect = aspect;
        }
    }

    /// Получить aspect ratio
    #[inline]
    pub fn aspect_ratio(&self) -> f32 {
        match self.projection {
            CameraProjection::Perspective(p) => p.aspect,
            CameraProjection::Orthographic(o) => (o.right - o.left) / (o.top - o.bottom),
        }
    }

    /// Мир в экранные координаты
    #[inline]
    pub fn world_to_screen(
        &self,
        world_pos: Vec3,
        camera_transform: &lumina_core::Transform,
        screen_size: Vec2,
    ) -> Option<Vec2> {
        let view_proj = self.view_projection_matrix(camera_transform);
        let clip = view_proj * world_pos.extend(1.0);
        
        if clip.w <= 0.0 {
            return None;
        }

        let ndc = clip.truncate() / clip.w;
        let screen = (ndc.xy() * 0.5 + 0.5) * screen_size;
        
        Some(screen)
    }

    /// Экран в мировые координаты (на плоскости z=dist)
    #[inline]
    pub fn screen_to_world(
        &self,
        screen_pos: Vec2,
        camera_transform: &lumina_core::Transform,
        screen_size: Vec2,
        distance: f32,
    ) -> Vec3 {
        let ndc = Vec2::new(
            2.0 * screen_pos.x / screen_size.x - 1.0,
            1.0 - 2.0 * screen_pos.y / screen_size.y,
        );

        let inv_view_proj = self.view_projection_matrix(camera_transform).inverse();
        let clip = Vec4::new(ndc.x, ndc.y, 0.5, 1.0);
        let world = inv_view_proj * clip;
        
        let world_pos = world.truncate() / world.w;
        let camera_pos = camera_transform.translation;
        
        let dir = (world_pos - camera_pos).normalize();
        camera_pos + dir * distance
    }

    /// Создать луч из экранных координат
    #[inline]
    pub fn screen_to_ray(
        &self,
        screen_pos: Vec2,
        camera_transform: &lumina_core::Transform,
        screen_size: Vec2,
    ) -> lumina_core::Ray {
        let origin = camera_transform.translation;
        let target = self.screen_to_world(screen_pos, camera_transform, screen_size, 1.0);
        let direction = (target - origin).normalize();
        
        lumina_core::Ray::new(origin, direction)
    }
}

/// Тип проекции камеры
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraProjection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

impl Default for CameraProjection {
    fn default() -> Self {
        Self::Perspective(PerspectiveProjection::default())
    }
}

/// Перспективная проекция
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerspectiveProjection {
    /// Вертикальный FOV в радианах
    pub fov_y: f32,
    /// Соотношение сторон
    pub aspect: f32,
}

impl Default for PerspectiveProjection {
    fn default() -> Self {
        Self {
            fov_y: std::f32::consts::FRAC_PI_4, // 45 degrees
            aspect: 16.0 / 9.0,
        }
    }
}

impl PerspectiveProjection {
    /// Создать из FOV в градусах
    #[inline]
    pub fn from_fov_degrees(fov_degrees: f32, aspect: f32) -> Self {
        Self {
            fov_y: fov_degrees.to_radians(),
            aspect,
        }
    }
}

/// Ортографическая проекция
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrthographicProjection {
    /// Левая граница
    pub left: f32,
    /// Правая граница
    pub right: f32,
    /// Нижняя граница
    pub bottom: f32,
    /// Верхняя граница
    pub top: f32,
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        Self {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
        }
    }
}

impl OrthographicProjection {
    /// Создать из width/height
    #[inline]
    pub fn from_size(width: f32, height: f32) -> Self {
        let half_width = width * 0.5;
        let half_height = height * 0.5;
        Self {
            left: -half_width,
            right: half_width,
            bottom: -half_height,
            top: half_height,
        }
    }

    /// Создать из размера и центра
    #[inline]
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half = size * 0.5;
        Self {
            left: center.x - half.x,
            right: center.x + half.x,
            bottom: center.y - half.y,
            top: center.y + half.y,
        }
    }

    /// Ширина
    #[inline]
    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    /// Высота
    #[inline]
    pub fn height(&self) -> f32 {
        self.top - self.bottom
    }

    /// Размер
    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }
}