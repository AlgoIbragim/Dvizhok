//! # Math - Математика для игрового движка
//! 
//! Оптимизированная математика поверх glam с игровыми абстракциями.

pub use glam::*;

/// Константы
pub const PI: f32 = std::f32::consts::PI;
pub const TAU: f32 = std::f32::consts::TAU;
pub const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2;
pub const FRAC_PI_4: f32 = std::f32::consts::FRAC_PI_4;
pub const E: f32 = std::f32::consts::E;

/// Прямоугольник
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    /// Создать из мин/макс
    #[inline]
    pub const fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Создать из центра и размера
    #[inline]
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half = size * 0.5;
        Self {
            min: center - half,
            max: center + half,
        }
    }

    /// Создать из origin и размера
    #[inline]
    pub const fn from_origin_size(origin: Vec2, size: Vec2) -> Self {
        Self {
            min: origin,
            max: Vec2::new(origin.x + size.x, origin.y + size.y),
        }
    }

    /// Ширина
    #[inline]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Высота
    #[inline]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Размер
    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    /// Центр
    #[inline]
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) * 0.5
    }

    /// Проверить содержит ли точку
    #[inline]
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
    }

    /// Проверить пересечение
    #[inline]
    pub fn intersects(&self, other: &Rect) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x
            && self.min.y <= other.max.y && self.max.y >= other.min.y
    }

    /// Пересечение двух прямоугольников
    #[inline]
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let min = Vec2::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y));
        let max = Vec2::new(self.max.x.min(other.max.x), self.max.y.min(other.max.y));
        
        if min.x < max.x && min.y < max.y {
            Some(Rect::new(min, max))
        } else {
            None
        }
    }

    /// Объединение двух прямоугольников
    #[inline]
    pub fn union(&self, other: &Rect) -> Rect {
        Rect::new(
            Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        )
    }

    /// Расширить на величину
    #[inline]
    pub fn inflate(&self, amount: f32) -> Rect {
        let delta = Vec2::splat(amount);
        Rect::new(self.min - delta, self.max + delta)
    }

    /// Увеличить
    #[inline]
    pub fn offset(&self, delta: Vec2) -> Rect {
        Rect::new(self.min + delta, self.max + delta)
    }
}

/// Ограничивающий box (AABB)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    /// Создать из мин/макс
    #[inline]
    pub const fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Создать из центра и полуразмеров
    #[inline]
    pub fn from_center_half_extents(center: Vec3, half_extents: Vec3) -> Self {
        Self {
            min: center - half_extents,
            max: center + half_extents,
        }
    }

    /// Центр
    #[inline]
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    /// Полуразмеры
    #[inline]
    pub fn half_extents(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    /// Размер
    #[inline]
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    /// Проверить содержит ли точку
    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        point.x >= self.min.x && point.x <= self.max.x
            && point.y >= self.min.y && point.y <= self.max.y
            && point.z >= self.min.z && point.z <= self.max.z
    }

    /// Проверить пересечение
    #[inline]
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x
            && self.min.y <= other.max.y && self.max.y >= other.min.y
            && self.min.z <= other.max.z && self.max.z >= other.min.z
    }

    /// Расширить для включения точки
    #[inline]
    pub fn expand_to_include(&mut self, point: Vec3) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    /// Объединить два AABB
    #[inline]
    pub fn merge(&self, other: &AABB) -> AABB {
        AABB::new(
            self.min.min(other.min),
            self.max.max(other.max),
        )
    }
}

/// Луч
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Создать луч
    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Получить точку на расстоянии t
    #[inline]
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Пересечение с плоскостью
    #[inline]
    pub fn intersect_plane(&self, plane_normal: Vec3, plane_d: f32) -> Option<f32> {
        let denom = plane_normal.dot(self.direction);
        if denom.abs() > f32::EPSILON {
            let t = -(plane_normal.dot(self.origin) + plane_d) / denom;
            if t >= 0.0 {
                return Some(t);
            }
        }
        None
    }

    /// Пересечение с AABB
    #[inline]
    pub fn intersect_aabb(&self, aabb: &AABB) -> Option<f32> {
        let inv_dir = Vec3::new(
            1.0 / self.direction.x,
            1.0 / self.direction.y,
            1.0 / self.direction.z,
        );

        let t1 = (aabb.min - self.origin) * inv_dir;
        let t2 = (aabb.max - self.origin) * inv_dir;

        let tmin = t1.min(t2);
        let tmax = t1.max(t2);

        let tmin = tmin.max_element();
        let tmax = tmax.min_element();

        if tmin <= tmax && tmax >= 0.0 {
            Some(if tmin >= 0.0 { tmin } else { 0.0 })
        } else {
            None
        }
    }

    /// Пересечение со сферой
    #[inline]
    pub fn intersect_sphere(&self, center: Vec3, radius: f32) -> Option<f32> {
        let oc = self.origin - center;
        let a = self.direction.length_squared();
        let half_b = oc.dot(self.direction);
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let root = (-half_b - sqrtd) / a;
            if root >= 0.0 {
                Some(root)
            } else {
                let root = (-half_b + sqrtd) / a;
                if root >= 0.0 {
                    Some(root)
                } else {
                    None
                }
            }
        }
    }
}

/// Сфера
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    #[inline]
    pub const fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    #[inline]
    pub fn contains(&self, point: Vec3) -> bool {
        self.center.distance_squared(point) <= self.radius * self.radius
    }

    #[inline]
    pub fn intersects(&self, other: &Sphere) -> bool {
        let distance_sq = self.center.distance_squared(other.center);
        let radius_sum = self.radius + other.radius;
        distance_sq <= radius_sum * radius_sum
    }

    /// Создать bounding sphere для точек
    #[inline]
    pub fn from_points(points: &[Vec3]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        // Find approximate center
        let mut center = Vec3::ZERO;
        for p in points {
            center += *p;
        }
        center /= points.len() as f32;

        // Find radius
        let mut radius_sq = 0.0f32;
        for p in points {
            radius_sq = radius_sq.max(center.distance_squared(*p));
        }

        Some(Self::new(center, radius_sq.sqrt()))
    }
}

/// Плоскость
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Plane {
    pub normal: Vec3,
    pub d: f32,
}

impl Plane {
    #[inline]
    pub fn new(normal: Vec3, d: f32) -> Self {
        let normal = normal.normalize();
        Self { normal, d }
    }

    #[inline]
    pub fn from_point_normal(point: Vec3, normal: Vec3) -> Self {
        let normal = normal.normalize();
        let d = -normal.dot(point);
        Self { normal, d }
    }

    #[inline]
    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
        let normal = (b - a).cross(c - a).normalize();
        let d = -normal.dot(a);
        Self { normal, d }
    }

    #[inline]
    pub fn distance_to_point(&self, point: Vec3) -> f32 {
        self.normal.dot(point) + self.d
    }

    #[inline]
    pub fn project_point(&self, point: Vec3) -> Vec3 {
        point - self.normal * self.distance_to_point(point)
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        let len = self.normal.length();
        if len > f32::EPSILON {
            Self {
                normal: self.normal / len,
                d: self.d / len,
            }
        } else {
            *self
        }
    }
}

/// Фрустум (пирамида видимости камеры)
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    pub planes: [Plane; 6],
}

impl Frustum {
    /// Проверить пересечение со сферой
    #[inline]
    pub fn intersects_sphere(&self, sphere: &Sphere) -> bool {
        for plane in &self.planes {
            let distance = plane.distance_to_point(sphere.center);
            if distance < -sphere.radius {
                return false;
            }
        }
        true
    }

    /// Проверить пересечение с AABB
    #[inline]
    pub fn intersects_aabb(&self, aabb: &AABB) -> bool {
        for plane in &self.planes {
            let mut positive = Vec3::ZERO;
            if plane.normal.x >= 0.0 {
                positive.x = aabb.max.x;
            } else {
                positive.x = aabb.min.x;
            }
            if plane.normal.y >= 0.0 {
                positive.y = aabb.max.y;
            } else {
                positive.y = aabb.min.y;
            }
            if plane.normal.z >= 0.0 {
                positive.z = aabb.max.z;
            } else {
                positive.z = aabb.min.z;
            }

            if plane.normal.dot(positive) + plane.d < 0.0 {
                return false;
            }
        }
        true
    }
}