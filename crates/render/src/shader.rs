//! Шейдеры и шейдерные модули

use wgpu::{ShaderModule, ShaderModuleDescriptor, ShaderSource};

/// Хендл шейдера
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ShaderHandle(pub u64);

/// Встроенные шейдеры
pub mod builtin {
    /// PBR шейдер
    pub const PBR: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/pbr.wgsl"));
    
    /// Unlit шейдер
    pub const UNLIT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/unlit.wgsl"));
    
    /// Шейдер теней
    pub const SHADOW: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/shadow.wgsl"));
    
    /// Постобработка
    pub const POST_PROCESS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/post_process.wgsl"));
    
    /// Глубина и нормали
    pub const DEPTH_NORMAL: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/depth_normal.wgsl"));
    
    /// Гизмос
    pub const GIZMO: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/gizmo.wgsl"));
    
    /// Линии
    pub const LINE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/line.wgsl"));
    
    /// UI
    pub const UI: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/ui.wgsl"));
}

/// Описание шейдера
#[derive(Debug, Clone)]
pub struct Shader {
    pub handle: ShaderHandle,
    pub source: String,
    pub module: Option<ShaderModule>,
    pub stage: ShaderStage,
    pub label: Option<String>,
}

/// Стадия шейдера
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
    VertexFragment,
    Mesh,
    Task,
}

impl Default for ShaderStage {
    fn default() -> Self {
        Self::VertexFragment
    }
}

impl Shader {
    /// Создать шейдер из строки
    pub fn new(source: impl Into<String>, stage: ShaderStage) -> Self {
        let source = source.into();
        Self {
            handle: ShaderHandle(0),
            source,
            module: None,
            stage,
            label: None,
        }
    }

    /// Создать шейдер из файла
    pub fn from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let source = std::fs::read_to_string(path)?;
        Ok(Self::new(source, ShaderStage::VertexFragment))
    }

    /// Установить лейбл
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Компилировать шейдер
    pub fn compile(&mut self, device: &wgpu::Device) -> Result<(), String> {
        let module = device.create_shader_module(ShaderModuleDescriptor {
            label: self.label.as_deref(),
            source: ShaderSource::Wgsl(self.source.clone().into()),
        });

        self.module = Some(module);
        Ok(())
    }

    /// Получить модуль шейдера
    pub fn module(&self) -> Option<&ShaderModule> {
        self.module.as_ref()
    }
}

/// Кэш шейдеров
#[derive(Debug, Default)]
pub struct ShaderCache {
    shaders: std::collections::HashMap<ShaderHandle, Shader>,
    next_handle: u64,
}

impl ShaderCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавить шейдер
    pub fn insert(&mut self, mut shader: Shader) -> ShaderHandle {
        let handle = ShaderHandle(self.next_handle);
        self.next_handle += 1;
        shader.handle = handle;
        self.shaders.insert(handle, shader);
        handle
    }

    /// Получить шейдер
    pub fn get(&self, handle: ShaderHandle) -> Option<&Shader> {
        self.shaders.get(&handle)
    }

    /// Получить mutable шейдер
    pub fn get_mut(&mut self, handle: ShaderHandle) -> Option<&mut Shader> {
        self.shaders.get_mut(&handle)
    }

    /// Удалить шейдер
    pub fn remove(&mut self, handle: ShaderHandle) -> Option<Shader> {
        self.shaders.remove(&handle)
    }

    /// Очистить кэш
    pub fn clear(&mut self) {
        self.shaders.clear();
        self.next_handle = 0;
    }
}

/// Предопределенные шейдеры
pub struct BuiltinShaders;

impl BuiltinShaders {
    /// Получить PBR шейдер
    pub fn pbr() -> &'static str {
        // Встроенный PBR шейдер на WGSL
        r#"
// PBR Shader

struct Camera {
    view_proj: mat4x4<f32>,
    position: vec3<f32>,
    _pad: f32,
}

struct Light {
    position: vec3<f32>,
    intensity: f32,
    color: vec3<f32>,
    radius: f32,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var<uniform> light: Light;

// Vertex Input
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec4<f32>,
}

// Vertex Output
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    
    output.clip_position = camera.view_proj * vec4<f32>(input.position, 1.0);
    output.world_position = input.position;
    output.normal = normalize(input.normal);
    output.uv = input.uv;
    output.tangent = normalize(input.tangent.xyz);
    output.bitangent = cross(output.normal, output.tangent) * input.tangent.w;
    
    return output;
}

// Material uniforms
struct Material {
    base_color: vec4<f32>,
    metallic_roughness: vec2<f32>,
    emissive: vec3<f32>,
    normal_scale: f32,
}

@group(2) @binding(0)
var<uniform> material: Material;

@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

// Constants
const PI: f32 = 3.14159265359;

// PBR functions
fn distribution_ggx(n: vec3<f32>, h: vec3<f32>, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let n_dot_h = max(dot(n, h), 0.0);
    let n_dot_h2 = n_dot_h * n_dot_h;
    
    let num = a2;
    let denom = (n_dot_h2 * (a2 - 1.0) + 1.0);
    
    return num / (PI * denom * denom);
}

fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;
    
    let num = n_dot_v;
    let denom = n_dot_v * (1.0 - k) + k;
    
    return num / denom;
}

fn geometry_smith(n: vec3<f32>, v: vec3<f32>, l: vec3<f32>, roughness: f32) -> f32 {
    let n_dot_v = max(dot(n, v), 0.0);
    let n_dot_l = max(dot(n, l), 0.0);
    let ggx2 = geometry_schlick_ggx(n_dot_v, roughness);
    let ggx1 = geometry_schlick_ggx(n_dot_l, roughness);
    
    return ggx1 * ggx2;
}

fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    return f0 + (1.0 - f0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Sample material
    let base_color = material.base_color * textureSample(base_color_texture, base_color_sampler, input.uv);
    let metallic = material.metallic_roughness.x;
    let roughness = material.metallic_roughness.y;
    let emissive = material.emissive;
    
    // Discard if fully transparent
    if (base_color.a < 0.01) {
        discard;
    }
    
    // Vectors
    let n = normalize(input.normal);
    let v = normalize(camera.position - input.world_position);
    let l = normalize(light.position - input.world_position);
    let h = normalize(v + l);
    
    // Reflection at normal incidence for dielectrics
    let f0 = mix(vec3<f32>(0.04), base_color.rgb, metallic);
    
    // Cook-Torrance BRDF
    let ndf = distribution_ggx(n, h, roughness);
    let g = geometry_smith(n, v, l, roughness);
    let f = fresnel_schlick(max(dot(h, v), 0.0), f0);
    
    let numerator = ndf * g * f;
    let denominator = 4.0 * max(dot(n, v), 0.0) * max(dot(n, l), 0.0) + 0.0001;
    let specular = numerator / denominator;
    
    // Lambertian diffuse
    let k_s = f;
    let k_d = (vec3<f32>(1.0) - k_s) * (1.0 - metallic);
    let diffuse = k_d * base_color.rgb / PI;
    
    // Combine
    let n_dot_l = max(dot(n, l), 0.0);
    let radiance = light.color * light.intensity;
    let lo = (diffuse + specular) * radiance * n_dot_l;
    
    // Ambient
    let ambient = vec3<f32>(0.03) * base_color.rgb;
    
    // Emissive
    let emissive_contribution = emissive;
    
    let color = ambient + lo + emissive_contribution;
    
    // HDR tone mapping (Reinhard)
    let mapped = color / (color + vec3<f32>(1.0));
    
    // Gamma correction
    let gamma_corrected = pow(mapped, vec3<f32>(1.0 / 2.2));
    
    return vec4<f32>(gamma_corrected, base_color.a);
}
        "#}
}

/// Утилиты для шейдеров
pub mod shader_utils {
    /// Предпроцессинг шейдера (include-ы, дефайны)
    pub fn preprocess(source: &str, _defines: &[(String, String)]) -> String {
        // Простой препроцессинг - можно расширить
        source.to_string()
    }

    /// Вставка #defines в шейдер
    pub fn inject_defines(source: &str, defines: &[(String, String)]) -> String {
        let define_lines: String = defines
            .iter()
            .map(|(k, v)| format!("#define {} {}\n", k, v))
            .collect();
        
        format!("{}\n{}", define_lines, source)
    }

    /// Получить entry point для стадии
    pub fn entry_point(stage: super::ShaderStage) -> &'static str {
        match stage {
            super::ShaderStage::Vertex => "vs_main",
            super::ShaderStage::Fragment => "fs_main",
            super::ShaderStage::Compute => "cs_main",
            super::ShaderStage::VertexFragment => "main",
            super::ShaderStage::Mesh => "ms_main",
            super::ShaderStage::Task => "ts_main",
        }
    }
}
