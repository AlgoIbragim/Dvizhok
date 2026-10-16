//! Текстуры и сэмплеры

pub use wgpu::{TextureFormat, TextureUsages, AddressMode, FilterMode, SamplerDescriptor};

use wgpu::{TextureFormat, TextureUsages, AddressMode, FilterMode};

/// Описание текстуры
#[derive(Debug, Clone)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: TextureFormat,
    pub usage: TextureUsages,
    pub mip_levels: u32,
    pub sample_count: u32,
    pub label: Option<String>,
}

impl Default for TextureDescriptor {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            depth: 1,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            mip_levels: 1,
            sample_count: 1,
            label: None,
        }
    }
}

/// Текстура на GPU
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub descriptor: TextureDescriptor,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl Texture {
    /// Создать текстуру из данных
    pub fn from_data(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &[u8],
        width: u32,
        height: u32,
        format: TextureFormat,
        label: Option<&str>,
    ) -> Self {
        let size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: label.map(|s| s.to_string()).as_deref(),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width * format.block_copy_size(None).unwrap()),
                rows_per_image: Some(height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });

        Self {
            texture,
            view,
            sampler,
            descriptor: TextureDescriptor {
                width,
                height,
                depth: 1,
                format,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                mip_levels: 1,
                sample_count: 1,
                label: label.map(|s| s.to_string()),
            },
            bind_group: None,
        }
    }

    /// Создать цветную текстуру
    pub fn from_color(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color: [u8; 4],
        label: Option<&str>,
    ) -> Self {
        Self::from_data(device, queue, &color, 1, 1, wgpu::TextureFormat::Rgba8UnormSrgb, label)
    }

    /// Создать нормальную карту по умолчанию
    pub fn default_normal(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        // Нормаль pointing up (128, 128, 255) in tangent space
        Self::from_data(
            device,
            queue,
            &[128, 128, 255, 255],
            1,
            1,
            wgpu::TextureFormat::Rgba8Unorm,
            Some("default_normal"),
        )
    }

    /// Создать белую текстуру
    pub fn white(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::from_color(device, queue, [255, 255, 255, 255], Some("white_texture"))
    }

    /// Создать черную текстуру
    pub fn black(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self::from_color(device, queue, [0, 0, 0, 255], Some("black_texture"))
    }
}

/// Дескриптор сэмплера
#[derive(Debug, Clone)]
pub struct SamplerDescriptor {
    pub address_mode_u: AddressMode,
    pub address_mode_v: AddressMode,
    pub address_mode_w: AddressMode,
    pub mag_filter: FilterMode,
    pub min_filter: FilterMode,
    pub mipmap_filter: FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<wgpu::CompareFunction>,
    pub anisotropy_clamp: u16,
    pub border_color: Option<wgpu::SamplerBorderColor>,
}

impl Default for SamplerDescriptor {
    fn default() -> Self {
        Self {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_min_clamp: 0.0,
            lod_max_clamp: 32.0,
            compare: None,
            anisotropy_clamp: 1,
            border_color: None,
        }
    }
}

/// Предустановленные сэмплеры
pub struct Samplers;

impl Samplers {
    /// Линейный сэмплер с повторением
    pub fn linear_repeat(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        })
    }

    /// Ближайший сосед с повторением
    pub fn nearest_repeat(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        })
    }

    /// Линейный с clamp к краю
    pub fn linear_clamp(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        })
    }

    /// Анизотропный сэмплер
    pub fn anisotropic(device: &wgpu::Device, max_anisotropy: u16) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            anisotropy_clamp: max_anisotropy,
            ..Default::default()
        })
    }

    /// Сэмплер для теней (comparison)
    pub fn shadow_comparison(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToBorder,
            address_mode_v: wgpu::AddressMode::ClampToBorder,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            border_color: Some(wgpu::SamplerBorderColor::OpaqueWhite),
            ..Default::default()
        })
    }
}

/// Утилиты для работы с текстурами
pub mod texture_utils {
    use super::*;

    /// Вычислить количество mip уровней
    pub fn mip_levels(width: u32, height: u32) -> u32 {
        ((width.max(height) as f32).log2().floor() as u32).max(1)
    }

    /// Размер mip уровня
    pub fn mip_size(width: u32, height: u32, level: u32) -> (u32, u32) {
        let scale = 2u32.pow(level);
        ((width / scale).max(1), (height / scale).max(1))
    }
}

use crate::{TextureHandle, SamplerDescriptor, AddressMode, FilterMode};
