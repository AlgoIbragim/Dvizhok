//! Система теней

use crate::post_process::{Upscaler, UpscaleSettings};
use glam::{Mat4, Vec3, Vec4};

/// Тип теней
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShadowType {
    /// Каскадные теневые карты (CSM) для направленного света
    Cascaded,
    /// Variance Shadow Maps (VSM) для мягких теней
    Variance,
    /// Exponential Shadow Maps (ESM)
    Exponential,
    /// PCF с фильтрацией
    PCF,
    /// PCSS (Percentage Closer Soft Shadows)
    PCSS,
}

impl Default for ShadowType {
    fn default() -> Self {
        Self::Cascaded
    }
}

/// Настройки теней
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowSettings {
    /// Включены ли тени
    pub enabled: bool,
    /// Тип теней
    pub shadow_type: ShadowType,
    /// Размер карты теней
    pub map_size: u32,
    /// Дальность отбрасывания теней
    pub distance: f32,
    /// Смещение для борьбы с shadow acne
    pub bias: f32,
    /// Нормальное смещение
    pub normal_bias: f32,
    /// Количество каскадов (для CSM)
    pub cascade_count: u32,
    /// Распределение каскадов
    pub cascade_split_lambda: f32,
}

impl Default for ShadowSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            shadow_type: ShadowType::default(),
            map_size: 2048,
            distance: 100.0,
            bias: 0.001,
            normal_bias: 0.005,
            cascade_count: 4,
            cascade_split_lambda: 0.5,
        }
    }
}

impl ShadowSettings {
    /// Создать настройки для быстрого рендера
    pub fn fast() -> Self {
        Self {
            enabled: true,
            shadow_type: ShadowType::PCF,
            map_size: 1024,
            distance: 50.0,
            bias: 0.002,
            normal_bias: 0.01,
            cascade_count: 2,
            ..Default::default()
        }
    }

    /// Создать настройки для качественного рендера
    pub fn high_quality() -> Self {
        Self {
            enabled: true,
            shadow_type: ShadowType::Cascaded,
            map_size: 4096,
            distance: 200.0,
            bias: 0.0005,
            normal_bias: 0.002,
            cascade_count: 4,
            ..Default::default()
        }
    }

    /// Отключить тени
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
}

/// Каскад для CSM
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cascade {
    /// View-projection матрица для этого каскада
    pub view_proj: Mat4,
    /// Ближняя плоскость
    pub near: f32,
    /// Дальняя плоскость
    pub far: f32,
    /// Границы в мировом пространстве
    pub bounds: CascadeBounds,
}

impl Default for Cascade {
    fn default() -> Self {
        Self {
            view_proj: Mat4::IDENTITY,
            near: 0.0,
            far: 1.0,
            bounds: CascadeBounds::default(),
        }
    }
}

/// Границы каскада
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CascadeBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for CascadeBounds {
    fn default() -> Self {
        Self {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        }
    }
}

/// Каскадные теневые карты
#[derive(Debug, Default)]
pub struct CascadedShadowMaps {
    pub cascades: Vec<Cascade>,
    pub shadow_map: Option<ShadowMapArray>,
    pub settings: ShadowSettings,
}

impl CascadedShadowMaps {
    pub fn new(settings: ShadowSettings) -> Self {
        let cascades = vec![Cascade::default(); settings.cascade_count as usize];
        
        Self {
            cascades,
            shadow_map: None,
            settings,
        }
    }

    /// Обновить каскады на основе позиции камеры и направления света
    pub fn update_cascades(
        &mut self,
        camera_pos: Vec3,
        camera_forward: Vec3,
        camera_fov: f32,
        camera_near: f32,
        camera_far: f32,
        light_dir: Vec3,
    ) {
        let cascade_count = self.settings.cascade_count as usize;
        let lambda = self.settings.cascade_split_lambda;

        for i in 0..cascade_count {
            let f = (i + 1) as f32 / cascade_count as f32;
            
            // Logarithmic split
            let log_split = camera_near * (camera_far / camera_near).powf(f);
            // Linear split
            let linear_split = camera_near + (camera_far - camera_near) * f;
            // Combined split
            let cascade_far = lambda * log_split + (1.0 - lambda) * linear_split;
            let cascade_near = if i == 0 {
                camera_near
            } else {
                self.cascades[i - 1].far
            };

            // Вычислить view-projection матрицу для этого каскада
            let view_proj = self.compute_cascade_view_proj(
                camera_pos,
                camera_forward,
                camera_fov,
                cascade_near,
                cascade_far,
                light_dir,
            );

            self.cascades[i].view_proj = view_proj;
            self.cascades[i].near = cascade_near;
            self.cascades[i].far = cascade_far;
        }
    }

    fn compute_cascade_view_proj(
        &self,
        camera_pos: Vec3,
        camera_forward: Vec3,
        camera_fov: f32,
        near: f32,
        far: f32,
        light_dir: Vec3,
    ) -> Mat4 {
        // Simplified cascade computation
        // In a full implementation, this would:
        // 1. Compute the view frustum for this cascade
        // 2. Find the bounding box of the frustum in light space
        // 3. Create an orthographic projection that covers this bounding box
        
        use glam::{Mat4, Vec3, Quat};
        
        let light_pos = camera_pos - light_dir * (far - near) * 0.5;
        let view = Mat4::look_at_rh(light_pos, camera_pos, Vec3::Y);
        
        // Simple orthographic projection
        let size = (far - near) * 0.5 * camera_fov.tan();
        let proj = Mat4::orthographic_rh(-size, size, -size, size, near, far);
        
        proj * view
    }
}

/// Массив теневых карт
#[derive(Debug)]
pub struct ShadowMapArray {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: u32,
    pub count: u32,
}

impl ShadowMapArray {
    pub fn new(
        device: &wgpu::Device,
        size: u32,
        count: u32,
        format: wgpu::TextureFormat,
        label: Option<&str>,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label.map(|s| s.to_string()).as_deref(),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: count,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            label: label.map(|s| format!("{}_view", s)).as_deref(),
            format: Some(format),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: Some(count),
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: label.map(|s| format!("{}_sampler", s)).as_deref(),
            address_mode_u: wgpu::AddressMode::ClampToBorder,
            address_mode_v: wgpu::AddressMode::ClampToBorder,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            border_color: Some(wgpu::SamplerBorderColor::OpaqueWhite),
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            size,
            count,
        }
    }
}

use glam::Mat4;
