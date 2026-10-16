//! Материалы и шейдеры

use lumina_core::Color;
use glam::{Vec2, Vec4};

/// Тип материала
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MaterialType {
    Standard,
    Unlit,
    Wireframe,
    Custom(u64),
}

impl Default for MaterialType {
    fn default() -> Self {
        Self::Standard
    }
}

/// Стандартный PBR материал
#[derive(Debug, Clone, PartialEq)]
pub struct StandardMaterial {
    /// Базовый цвет (albedo)
    pub base_color: Color,
    /// Базовая текстура (опционально)
    pub base_color_texture: Option<TextureHandle>,
    /// Металличность (0.0 - диэлектрик, 1.0 - металл)
    pub metallic: f32,
    /// Шероховатость (0.0 - гладкий, 1.0 - шероховатый)
    pub roughness: f32,
    /// Metallic-roughness текстура (опционально)
    pub metallic_roughness_texture: Option<TextureHandle>,
    /// Нормальная карта (опционально)
    pub normal_texture: Option<TextureHandle>,
    /// Сила нормали
    pub normal_scale: f32,
    /// AO текстура (опционально)
    pub occlusion_texture: Option<TextureHandle>,
    /// Сила окклюзии
    pub occlusion_strength: f32,
    /// Эмиссионный цвет
    pub emissive: Color,
    /// Эмиссионная текстура (опционально)
    pub emissive_texture: Option<TextureHandle>,
    /// Прозрачность (0.0 - полностью прозрачный, 1.0 - непрозрачный)
    pub alpha: f32,
    /// Режим прозрачности
    pub alpha_mode: AlphaMode,
    /// Двусторонний
    pub double_sided: bool,
    /// Показывать ли в wireframe режиме
    pub wireframe: bool,
    /// Пользовательские данные шейдера
    pub shader_data: Vec<f32>,
}

impl Default for StandardMaterial {
    fn default() -> Self {
        Self {
            base_color: Color::WHITE,
            base_color_texture: None,
            metallic: 0.0,
            roughness: 0.5,
            metallic_roughness_texture: None,
            normal_texture: None,
            normal_scale: 1.0,
            occlusion_texture: None,
            occlusion_strength: 1.0,
            emissive: Color::BLACK,
            emissive_texture: None,
            alpha: 1.0,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
            wireframe: false,
            shader_data: Vec::new(),
        }
    }
}

impl StandardMaterial {
    /// Создает материал с только цветом
    pub fn from_color(color: Color) -> Self {
        Self {
            base_color: color,
            ..Default::default()
        }
    }

    /// Создает металлический материал
    pub fn metallic(color: Color, roughness: f32) -> Self {
        Self {
            base_color: color,
            metallic: 1.0,
            roughness: roughness.clamp(0.0, 1.0),
            ..Default::default()
        }
    }

    /// Создает непрозрачный материал
    pub fn unlit(color: Color) -> Self {
        Self {
            base_color: color,
            ..Default::default()
        }
    }

    /// Устанавливает прозрачность
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self.alpha_mode = if self.alpha < 1.0 {
            AlphaMode::Blend
        } else {
            AlphaMode::Opaque
        };
        self
    }

    /// Устанавливает текстуру
    pub fn with_texture(mut self, texture: TextureHandle) -> Self {
        self.base_color_texture = Some(texture);
        self
    }

    /// Устанавливает нормальную карту
    pub fn with_normal_map(mut self, texture: TextureHandle, scale: f32) -> Self {
        self.normal_texture = Some(texture);
        self.normal_scale = scale;
        self
    }
}

/// Режим прозрачности
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlphaMode {
    /// Непрозрачный
    Opaque,
    /// Прозрачный с blend
    Blend,
    /// Прозрачный с cutout
    Mask(f32),
    /// Additive blending
    Add,
    /// Multiply blending
    Multiply,
}

/// Хендл текстуры
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TextureHandle(pub u64);

/// Хендл материала
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MaterialHandle(pub u64);

/// Менеджер материалов
#[derive(Debug, Default)]
pub struct MaterialManager {
    materials: std::collections::HashMap<MaterialHandle, StandardMaterial>,
    next_id: u64,
}

impl MaterialManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, material: StandardMaterial) -> MaterialHandle {
        let handle = MaterialHandle(self.next_id);
        self.next_id += 1;
        self.materials.insert(handle, material);
        handle
    }

    pub fn get(&self, handle: MaterialHandle) -> Option<&StandardMaterial> {
        self.materials.get(&handle)
    }

    pub fn get_mut(&mut self, handle: MaterialHandle) -> Option<&mut StandardMaterial> {
        self.materials.get_mut(&handle)
    }

    pub fn remove(&mut self, handle: MaterialHandle) -> Option<StandardMaterial> {
        self.materials.remove(&handle)
    }
}
