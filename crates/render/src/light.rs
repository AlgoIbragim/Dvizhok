//! # Light - Освещение
//! 
//! Источники света: направленный, точечный, прожектор.

use lumina_core::Color;
use glam::*;

/// Компонент источника света
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    /// Тип и параметры света
    pub light_type: LightType,
    /// Цвет света
    pub color: Color,
    /// Интенсивность
    pub intensity: f32,
    /// Диапазон (для точечных и прожекторов)
    pub range: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: LightType::Point,
            color: Color::WHITE,
            intensity: 1.0,
            range: 10.0,
        }
    }
}

impl Light {
    /// Создать направленный свет
    #[inline]
    pub fn directional(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional { direction: direction.normalize() },
            color,
            intensity,
            range: f32::INFINITY,
            ..Default::default()
        }
    }

    /// Создать точечный свет
    #[inline]
    pub fn point(color: Color, intensity: f32, range: f32) -> Self {
        Self {
            light_type: LightType::Point,
            color,
            intensity,
            range,
            ..Default::default()
        }
    }

    /// Создать прожектор
    #[inline]
    pub fn spot(direction: Vec3, color: Color, intensity: f32, range: f32, inner_angle: f32, outer_angle: f32) -> Self {
        Self {
            light_type: LightType::Spot {
                direction: direction.normalize(),
                inner_angle: inner_angle.clamp(0.0, std::f32::consts::PI),
                outer_angle: outer_angle.clamp(0.0, std::f32::consts::PI),
            },
            color,
            intensity,
            range,
        }
    }

    /// Является ли направленным светом
    #[inline]
    pub fn is_directional(&self) -> bool {
        matches!(self.light_type, LightType::Directional { .. })
    }

    /// Является ли точечным светом
    #[inline]
    pub fn is_point(&self) -> bool {
        matches!(self.light_type, LightType::Point)
    }

    /// Является ли прожектором
    #[inline]
    pub fn is_spot(&self) -> bool {
        matches!(self.light_type, LightType::Spot { .. })
    }

    /// Получить направление (для directional/spot)
    #[inline]
    pub fn direction(&self) -> Option<Vec3> {
        match self.light_type {
            LightType::Directional { direction } => Some(direction),
            LightType::Spot { direction, .. } => Some(direction),
            _ => None,
        }
    }

    /// Получить углы (для spot)
    #[inline]
    pub fn spot_angles(&self) -> Option<(f32, f32)> {
        match self.light_type {
            LightType::Spot { inner_angle, outer_angle, .. } => {
                Some((inner_angle, outer_angle))
            }
            _ => None,
        }
    }

    /// Рассчитать интенсивность на расстоянии (для point/spot)
    #[inline]
    pub fn intensity_at_distance(&self, distance: f32) -> f32 {
        if self.is_directional() {
            return self.intensity;
        }

        // Inverse square law with smooth falloff
        let dist_sq = distance * distance;
        let range_sq = self.range * self.range;
        
        if dist_sq >= range_sq {
            return 0.0;
        }

        let falloff = 1.0 - dist_sq / range_sq;
        self.intensity * falloff * falloff / (1.0 + dist_sq)
    }

    /// Вычислить освещение в точке
    #[inline]
    pub fn compute_lighting(&self, position: Vec3, normal: Vec3, light_pos: Vec3) -> f32 {
        match self.light_type {
            LightType::Directional { direction } => {
                let light_dir = -direction;
                normal.dot(light_dir).max(0.0)
            }
            LightType::Point => {
                let light_dir = (light_pos - position).normalize();
                let distance = light_pos.distance(position);
                let attenuation = self.intensity_at_distance(distance);
                normal.dot(light_dir).max(0.0) * attenuation
            }
            LightType::Spot { direction, inner_angle, outer_angle } => {
                let light_dir = (light_pos - position).normalize();
                let spot_dir = -direction;
                let cos_angle = spot_dir.dot(light_dir);
                let cos_outer = outer_angle.cos();
                let cos_inner = inner_angle.cos();
                
                if cos_angle < cos_outer {
                    return 0.0;
                }
                
                let spot_attenuation = if cos_angle >= cos_inner {
                    1.0
                } else {
                    (cos_angle - cos_outer) / (cos_inner - cos_outer)
                };
                
                let distance = light_pos.distance(position);
                let attenuation = self.intensity_at_distance(distance);
                
                normal.dot(light_dir).max(0.0) * attenuation * spot_attenuation
            }
        }
    }
}

/// Тип источника света
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    /// Направленный свет (солнце)
    Directional {
        direction: Vec3,
    },
    /// Точечный свет (лампочка)
    Point,
    /// Прожектор
    Spot {
        direction: Vec3,
        inner_angle: f32,
        outer_angle: f32,
    },
}

/// Параметры ambient освещения
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AmbientLight {
    pub color: Color,
    pub intensity: f32,
}

impl Default for AmbientLight {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            intensity: 0.1,
        }
    }
}

/// Environment lighting (IBL)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnvironmentLight {
    /// HDR skybox/environment map
    pub irradiance: f32,
    /// Specular reflection intensity
    pub specular: f32,
}

impl Default for EnvironmentLight {
    fn default() -> Self {
        Self {
            irradiance: 1.0,
            specular: 1.0,
        }
    }
}

/// Кластеризованное освещение (для Forward+)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClusteredLighting {
    /// Число кластеров по X
    pub cluster_count_x: u32,
    /// Число кластеров по Y
    pub cluster_count_y: u32,
    /// Число кластеров по Z (глубина)
    pub cluster_count_z: u32,
    /// Максимальное число источников на кластер
    pub max_lights_per_cluster: u32,
}

impl Default for ClusteredLighting {
    fn default() -> Self {
        Self {
            cluster_count_x: 16,
            cluster_count_y: 9,
            cluster_count_z: 24,
            max_lights_per_cluster: 256,
        }
    }
}

/// Параметры теней
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowSettings {
    /// Включить тени
    pub enabled: bool,
    /// Размер карты теней
    pub map_size: u32,
    /// Дальность теней
    pub distance: f32,
    /// Мягкие тени (PCF)
    pub soft_shadows: bool,
    /// Каскады (для CSM)
    pub cascade_count: u32,
    /// Размеры каскадов
    pub cascade_splits: [f32; 4],
}

impl Default for ShadowSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            map_size: 2048,
            distance: 100.0,
            soft_shadows: true,
            cascade_count: 4,
            cascade_splits: [0.05, 0.15, 0.4, 1.0],
        }
    }
}

/// Глобальные настройки освещения
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LightingSettings {
    /// Ambient свет
    pub ambient: AmbientLight,
    /// Environment (IBL)
    pub environment: EnvironmentLight,
    /// Настройки теней
    pub shadows: ShadowSettings,
    /// Кластеризованное освещение
    pub clustered: ClusteredLighting,
    /// Максимальное число источников
    pub max_lights: u32,
    /// Exposure
    pub exposure: f32,
    /// HDR peak brightness
    pub hdr_peak: f32,
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            ambient: AmbientLight::default(),
            environment: EnvironmentLight::default(),
            shadows: ShadowSettings::default(),
            clustered: ClusteredLighting::default(),
            max_lights: 1024,
            exposure: 1.0,
            hdr_peak: 1000.0,
        }
    }
}