//! # Color - Цвета
//! 
//! Поддержка различных цветовых пространств и форматов.

/// Цвет RGBA (32-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(C)]
pub struct Color32 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Цвет RGBA (float)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color32 {
    /// Прозрачный
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);
    /// Черный
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    /// Белый
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    /// Красный
    pub const RED: Self = Self::new(255, 0, 0, 255);
    /// Зеленый
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    /// Синий
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    /// Желтый
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);
    /// Циан
    pub const CYAN: Self = Self::new(0, 255, 255, 255);
    /// Магента
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    /// Серый
    pub const GRAY: Self = Self::new(128, 128, 128, 255);
    /// Темно-серый
    pub const DARK_GRAY: Self = Self::new(64, 64, 64, 255);
    /// Светло-серый
    pub const LIGHT_GRAY: Self = Self::new(192, 192, 192, 255);
    /// Оранжевый
    pub const ORANGE: Self = Self::new(255, 165, 0, 255);
    /// Розовый
    pub const PINK: Self = Self::new(255, 192, 203, 255);

    /// Создать цвет
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Из RGB
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Из RGBA
    #[inline]
    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(r, g, b, a)
    }

    /// Из hex (0xRRGGBBAA)
    #[inline]
    pub const fn from_hex(hex: u32) -> Self {
        Self::new(
            ((hex >> 24) & 0xFF) as u8,
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    /// Из hex без alpha (0xRRGGBB)
    #[inline]
    pub const fn from_hex_rgb(hex: u32) -> Self {
        Self::new(
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
            255,
        )
    }

    /// Преобразовать в float цвет
    #[inline]
    pub fn to_color(&self) -> Color {
        Color::from(*self)
    }

    /// С alpha
    #[inline]
    pub const fn with_alpha(self, a: u8) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }

    /// Интерполяция
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            (self.r as f32 + (other.r as f32 - self.r as f32) * t) as u8,
            (self.g as f32 + (other.g as f32 - self.g as f32) * t) as u8,
            (self.b as f32 + (other.b as f32 - self.b as f32) * t) as u8,
            (self.a as f32 + (other.a as f32 - self.a as f32) * t) as u8,
        )
    }

    /// Осветлить
    #[inline]
    pub fn lighten(self, amount: u8) -> Self {
        Self::new(
            self.r.saturating_add(amount),
            self.g.saturating_add(amount),
            self.b.saturating_add(amount),
            self.a,
        )
    }

    /// Затемнить
    #[inline]
    pub fn darken(self, amount: u8) -> Self {
        Self::new(
            self.r.saturating_sub(amount),
            self.g.saturating_sub(amount),
            self.b.saturating_sub(amount),
            self.a,
        )
    }
}

impl Color {
    /// Прозрачный
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    /// Черный
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    /// Белый
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    /// Красный
    pub const RED: Self = Self::new(1.0, 0.0, 0.0, 1.0);
    /// Зеленый
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0, 1.0);
    /// Синий
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0, 1.0);
    /// Желтый
    pub const YELLOW: Self = Self::new(1.0, 1.0, 0.0, 1.0);
    /// Циан
    pub const CYAN: Self = Self::new(0.0, 1.0, 1.0, 1.0);
    /// Магента
    pub const MAGENTA: Self = Self::new(1.0, 0.0, 1.0, 1.0);

    /// Создать цвет
    #[inline]
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Из RGB
    #[inline]
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Из RGBA
    #[inline]
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::new(r, g, b, a)
    }

    /// Из Color32
    #[inline]
    pub const fn from_color32(c: Color32) -> Self {
        Self::new(
            c.r as f32 / 255.0,
            c.g as f32 / 255.0,
            c.b as f32 / 255.0,
            c.a as f32 / 255.0,
        )
    }

    /// В Color32
    #[inline]
    pub fn to_color32(&self) -> Color32 {
        Color32::new(
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }

    /// С alpha
    #[inline]
    pub const fn with_alpha(self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }

    /// Без alpha (alpha = 1.0)
    #[inline]
    pub const fn without_alpha(self) -> Self {
        Self::new(self.r, self.g, self.b, 1.0)
    }

    /// RGB компоненты как Vec3
    #[inline]
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    /// RGBA как Vec4
    #[inline]
    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// Из Vec3 (RGB, A = 1.0)
    #[inline]
    pub const fn from_vec3(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z, 1.0)
    }

    /// Из Vec4
    #[inline]
    pub const fn from_vec4(v: Vec4) -> Self {
        Self::new(v.x, v.y, v.z, v.w)
    }

    /// Интерполяция (linear)
    #[inline]
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::new(
            self.r + (other.r - self.r) * t,
            self.g + (other.g - self.g) * t,
            self.b + (other.b - self.b) * t,
            self.a + (other.a - self.a) * t,
        )
    }

    /// Яркость (перцептивная)
    #[inline]
    pub fn luminance(&self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    /// Осветлить
    #[inline]
    pub fn lighten(&self, amount: f32) -> Color {
        Color::new(
            (self.r + amount).clamp(0.0, 1.0),
            (self.g + amount).clamp(0.0, 1.0),
            (self.b + amount).clamp(0.0, 1.0),
            self.a,
        )
    }

    /// Затемнить
    #[inline]
    pub fn darken(&self, amount: f32) -> Color {
        Color::new(
            (self.r - amount).clamp(0.0, 1.0),
            (self.g - amount).clamp(0.0, 1.0),
            (self.b - amount).clamp(0.0, 1.0),
            self.a,
        )
    }

    /// Насыщенность
    #[inline]
    pub fn saturate(&self, amount: f32) -> Color {
        let gray = self.luminance();
        Color::new(
            (gray + (self.r - gray) * amount).clamp(0.0, 1.0),
            (gray + (self.g - gray) * amount).clamp(0.0, 1.0),
            (gray + (self.b - gray) * amount).clamp(0.0, 1.0),
            self.a,
        )
    }

    /// Инвертировать
    #[inline]
    pub fn invert(&self) -> Color {
        Color::new(1.0 - self.r, 1.0 - self.g, 1.0 - self.b, self.a)
    }

    /// Гамма-коррекция
    #[inline]
    pub fn to_linear(&self) -> Color {
        Color::new(
            self.r.powf(2.2),
            self.g.powf(2.2),
            self.b.powf(2.2),
            self.a,
        )
    }

    /// Из линейного в sRGB
    #[inline]
    pub fn to_srgb(&self) -> Color {
        Color::new(
            self.r.powf(1.0 / 2.2),
            self.g.powf(1.0 / 2.2),
            self.b.powf(1.0 / 2.2),
            self.a,
        )
    }

    /// Проверка валидности
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.r.is_finite() && self.g.is_finite() && self.b.is_finite() && self.a.is_finite()
            && self.r >= 0.0 && self.g >= 0.0 && self.b >= 0.0 && self.a >= 0.0
            && self.r <= 1.0 && self.g <= 1.0 && self.b <= 1.0 && self.a <= 1.0
    }

    /// Сделать валидным
    #[inline]
    pub fn make_valid(&mut self) {
        self.r = self.r.clamp(0.0, 1.0);
        self.g = self.g.clamp(0.0, 1.0);
        self.b = self.b.clamp(0.0, 1.0);
        self.a = self.a.clamp(0.0, 1.0);
    }
}

impl Color32 {
    /// Создать
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Из RGB
    #[inline]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// С alpha
    #[inline]
    pub const fn with_alpha(self, a: u8) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }

    /// В u32
    #[inline]
    pub const fn to_u32(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }

    /// Из u32
    #[inline]
    pub const fn from_u32(value: u32) -> Self {
        Self::new(
            ((value >> 24) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            ((value >> 8) & 0xFF) as u8,
            (value & 0xFF) as u8,
        )
    }

    /// Преумножить
    #[inline]
    pub fn premultiply_alpha(self) -> Self {
        let a = self.a as f32 / 255.0;
        Self::new(
            ((self.r as f32) * a) as u8,
            ((self.g as f32) * a) as u8,
            ((self.b as f32) * a) as u8,
            self.a,
        )
    }
}

impl From<Color> for Color32 {
    #[inline]
    fn from(c: Color) -> Self {
        c.to_color32()
    }
}

impl From<Color32> for Color {
    #[inline]
    fn from(c: Color32) -> Self {
        c.to_color()
    }
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from(v: [f32; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }
}

impl From<Color> for [f32; 4] {
    #[inline]
    fn from(c: Color) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}

impl From<[u8; 4]> for Color32 {
    #[inline]
    fn from(v: [u8; 4]) -> Self {
        Self::new(v[0], v[1], v[2], v[3])
    }
}

impl From<Color32> for [u8; 4] {
    #[inline]
    fn from(c: Color32) -> Self {
        [c.r, c.g, c.b, c.a]
    }
}