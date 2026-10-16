//! # Transform - Трансформации объектов
//! 
//! Положение, вращение и масштаб игровых объектов.

use glam::*;

/// Трансформация игрового объекта
/// 
/// Порядок применения: Scale -> Rotate -> Translate
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    /// Единичная трансформация
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    /// Создать из компонентов
    #[inline]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            translation: Vec3::new(x, y, z),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Создать из translation
    #[inline]
    pub const fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    /// Создать из rotation
    #[inline]
    pub const fn from_rotation(rotation: Quat) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    /// Создать из scale
    #[inline]
    pub const fn from_scale(scale: Vec3) -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        }
    }

    /// Создать из массива
    #[inline]
    pub fn from_matrix(matrix: Mat4) -> Self {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
        Self {
            translation,
            rotation,
            scale,
        }
    }

    /// Получить матрицу трансформации
    #[inline]
    pub fn compute_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Получить инвертированную матрицу
    #[inline]
    pub fn compute_matrix_inverse(&self) -> Mat4 {
        self.compute_matrix().inverse()
    }

    /// Смотреть на цель
    #[inline]
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = (target - self.translation).normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);
        
        self.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
    }

    /// Получить forward вектор
    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    /// Получить right вектор
    #[inline]
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Получить up вектор
    #[inline]
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// Переместить относительно
    #[inline]
    pub fn translate(&mut self, translation: Vec3) {
        self.translation += translation;
    }

    /// Повернуть относительно
    #[inline]
    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }

    /// Масштабировать
    #[inline]
    pub fn scale_by(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    /// С трансляцией
    #[inline]
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    /// С вращением
    #[inline]
    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    /// С масштабом
    #[inline]
    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    /// Интерполяция между двумя трансформациями
    #[inline]
    pub fn lerp(&self, other: &Transform, t: f32) -> Transform {
        Transform {
            translation: self.translation.lerp(other.translation, t),
            rotation: self.rotation.slerp(other.rotation, t),
            scale: self.scale.lerp(other.scale, t),
        }
    }

    /// Трансформировать точку
    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.rotation * (point * self.scale) + self.translation
    }

    /// Трансформировать вектор (без translation)
    #[inline]
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (vector * self.scale)
    }

    /// Инвертировать
    #[inline]
    pub fn inverse(&self) -> Transform {
        let inv_scale = Vec3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        let inv_rotation = self.rotation.inverse();
        let inv_translation = -(inv_rotation * self.translation * inv_scale);
        
        Transform {
            translation: inv_translation,
            rotation: inv_rotation,
            scale: inv_scale,
        }
    }

    /// Проверка на валидность
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.translation.is_finite()
            && self.rotation.is_finite()
            && self.scale.is_finite()
            && self.scale.cmpge(Vec3::ZERO).all()
    }

    /// Округлить к валидным значениям
    #[inline]
    pub fn make_valid(&mut self) {
        if !self.translation.is_finite() {
            self.translation = Vec3::ZERO;
        }
        if !self.rotation.is_finite() || self.rotation.length_squared() < f32::EPSILON {
            self.rotation = Quat::IDENTITY;
        } else {
            self.rotation = self.rotation.normalize();
        }
        if !self.scale.is_finite() {
            self.scale = Vec3::ONE;
        } else {
            self.scale = self.scale.max(Vec3::splat(1e-5));
        }
    }
}

impl From<Mat4> for Transform {
    #[inline]
    fn from(matrix: Mat4) -> Self {
        Self::from_matrix(matrix)
    }
}

impl From<Transform> for Mat4 {
    #[inline]
    fn from(transform: Transform) -> Self {
        transform.compute_matrix()
    }
}

impl std::ops::Mul for Transform {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Transform {
            translation: self.transform_point(rhs.translation),
            rotation: self.rotation * rhs.rotation,
            scale: self.scale * rhs.scale,
        }
    }
}

impl std::ops::Mul<Vec3> for Transform {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        self.transform_point(rhs)
    }
}