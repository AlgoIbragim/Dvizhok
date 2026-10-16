//! Постобработка

/// Эффект постобработки
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PostProcessEffect {
    /// Тональная компрессия (HDR -> LDR)
    ToneMapping(ToneMapping),
    /// Цветокоррекция
    ColorGrading,
    /// Экспозиция
    Exposure,
    /// Bloom
    Bloom,
    /// Ambient Occlusion (SSAO)
    SSAO,
    /// Screen Space Reflections (SSR)
    SSR,
    /// Motion Blur
    MotionBlur,
    /// Depth of Field
    DepthOfField,
    /// Chromatic Aberration
    ChromaticAberration,
    /// Vignette
    Vignette,
    /// Film Grain
    FilmGrain,
    /// LUT (Look-Up Table)
    Lut,
    /// FXAA
    FXAA,
    /// SMAA
    SMAA,
    /// TAA
    TAA,
    /// DLSS/FSR upscale
    Upscaling(Upscaler),
    /// Custom
    Custom(u64),
}

/// Алгоритм тональной компрессии
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToneMapping {
    /// Без тональной компрессии
    None,
    /// Reinhard
    #[default]
    Reinhard,
    /// Reinhard с сжатием
    ReinhardExtended,
    /// ACES Filmic
    AcesFilmic,
    /// Uncharted 2
    Uncharted2,
    /// AgX
    AgX,
    /// Custom curve
    Custom(u64),
}

/// Апскейлер
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Upscaler {
    /// FSR 1.0
    Fsr1,
    /// FSR 2.0
    Fsr2,
    /// DLSS
    Dlss,
    /// XeSS
    Xess,
    /// Простой билинейный
    Bilinear,
    /// Bicubic
    Bicubic,
    /// Lanczos
    Lanczos,
}

/// Настройки постобработки
#[derive(Debug, Clone)]
pub struct PostProcessSettings {
    /// Включена ли постобработка
    pub enabled: bool,
    /// Тональная компрессия
    pub tone_mapping: ToneMapping,
    /// Экспозиция
    pub exposure: f32,
    /// Минимальная экспозиция (для автоэкспозиции)
    pub min_exposure: f32,
    /// Максимальная экспозиция (для автоэкспозиции)
    pub max_exposure: f32,
    /// Скорость адаптации экспозиции
    pub exposure_adaptation_speed: f32,
    /// Яркость
    pub brightness: f32,
    /// Контраст
    pub contrast: f32,
    /// Насыщенность
    pub saturation: f32,
    /// Температура цвета
    pub color_temperature: f32,
    /// Оттенок
    pub tint: f32,
    /// Bloom
    pub bloom: BloomSettings,
    /// SSAO
    pub ssao: SsaoSettings,
    /// Motion blur
    pub motion_blur: MotionBlurSettings,
    /// Depth of field
    pub depth_of_field: DepthOfFieldSettings,
    /// Vignette
    pub vignette: VignetteSettings,
    /// Film grain
    pub film_grain: FilmGrainSettings,
    /// Chromatic aberration
    pub chromatic_aberration: f32,
    /// FXAA
    pub fxaa_enabled: bool,
    /// SMAA
    pub smaa_enabled: bool,
    /// TAA
    pub taa_enabled: bool,
    /// Апскейлинг
    pub upscaling: UpscaleSettings,
}

impl Default for PostProcessSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            tone_mapping: ToneMapping::default(),
            exposure: 1.0,
            min_exposure: 0.1,
            max_exposure: 10.0,
            exposure_adaptation_speed: 1.0,
            brightness: 0.0,
            contrast: 1.0,
            saturation: 1.0,
            color_temperature: 6500.0,
            tint: 0.0,
            bloom: BloomSettings::default(),
            ssao: SsaoSettings::default(),
            motion_blur: MotionBlurSettings::default(),
            depth_of_field: DepthOfFieldSettings::default(),
            vignette: VignetteSettings::default(),
            film_grain: FilmGrainSettings::default(),
            chromatic_aberration: 0.0,
            fxaa_enabled: false,
            smaa_enabled: false,
            taa_enabled: false,
            upscaling: UpscaleSettings::default(),
        }
    }
}

/// Настройки Bloom
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BloomSettings {
    pub enabled: bool,
    pub intensity: f32,
    pub threshold: f32,
    pub soft_knee: f32,
    pub diffusion: f32,
    pub anamorphic_ratio: f32,
}

impl Default for BloomSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            intensity: 0.5,
            threshold: 1.0,
            soft_knee: 0.5,
            diffusion: 7.0,
            anamorphic_ratio: 0.0,
        }
    }
}

/// Настройки SSAO
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SsaoSettings {
    pub enabled: bool,
    pub radius: f32,
    pub intensity: f32,
    pub bias: f32,
    pub blur: bool,
    pub quality: SsaoQuality,
}

impl Default for SsaoSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            radius: 0.5,
            intensity: 1.0,
            bias: 0.025,
            blur: true,
            quality: SsaoQuality::Medium,
        }
    }
}

/// Настройки SSAOMotion Blur
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotionBlurSettings {
    pub enabled: bool,
    pub intensity: f32,
    pub sample_count: u32,
}

impl Default for MotionBlurSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 1.0,
            sample_count: 8,
        }
    }
}

/// Настройки Depth of Field
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DepthOfFieldSettings {
    pub enabled: bool,
    pub focal_distance: f32,
    pub focal_length: f32,
    pub aperture: f32,
    pub blur_size: f32,
}

impl Default for DepthOfFieldSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            focal_distance: 10.0,
            focal_length: 50.0,
            aperture: 2.8,
            blur_size: 5.0,
        }
    }
}

/// Настройки Vignette
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VignetteSettings {
    pub enabled: bool,
    pub intensity: f32,
    pub smoothness: f32,
    pub roundness: f32,
    pub color: [f32; 3],
}

impl Default for VignetteSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 0.4,
            smoothness: 0.2,
            roundness: 1.0,
            color: [0.0, 0.0, 0.0],
        }
    }
}

/// Настройки Film Grain
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilmGrainSettings {
    pub enabled: bool,
    pub intensity: f32,
    pub size: f32,
    pub lum_contrib: f32,
}

impl Default for FilmGrainSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            intensity: 0.1,
            size: 1.0,
            lum_contrib: 0.8,
        }
    }
}

/// Настройки/// Настройки апскейлинга
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpscaleSettings {
    pub enabled: bool,
    pub upscaler: Upscaler,
    pub render_scale: f32,
    pub sharpness: f32,
}

impl Default for UpscaleSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            upscaler: Upscaler::Bilinear,
            render_scale: 1.0,
            sharpness: 0.5,
        }
    }
}

/// Тип апскейлера
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Upscaler {
    Fsr1,
    Fsr2,
    Dlss,
    Xess,
    #[default]
    Bilinear,
    Bicubic,
    Lanczos,
}

/// Качество SSAO
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SsaoQuality {
    #[default]
    Low,
    Medium,
    High,
    Ultra,
}

/// Теневая карта
#[derive(Debug)]
pub struct ShadowMap {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: u32,
    pub format: wgpu::TextureFormat,
}

impl ShadowMap {
    pub fn new(
        device: &wgpu::Device,
        size: u32,
        format: wgpu::TextureFormat,
        label: Option<&str>,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label.map(|s| s.to_string()).as_deref(),
            size: wgpu::Extent3d {
                width: size,
                height: size,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

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
            format,
        }
    }
}
