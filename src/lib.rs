//! # Lumina Engine
//! 
//! Легкий, но мощный 2D/3D игровой движок на Rust.
//! 
//! ## Пример использования:
//! ```no_run
//! use lumina::prelude::*;
//! 
//! fn main() {
//!     App::new()
//!         .add_plugin(CorePlugin)
//!         .add_plugin(RenderPlugin)
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//! 
//! fn setup(mut commands: Commands) {
//!     // Создаем камеру
//!     commands.spawn(Camera::perspective(45.0_f32.to_radians(), 16.0/9.0, 0.1, 1000.0));
//!     
//!     // Создаем куб
//!     commands.spawn((
//!         Transform::from_xyz(0.0, 0.0, 0.0),
//!         Mesh::cube(1.0),
//!         StandardMaterial::default(),
//!     ));
//! }
//! ```

// Re-export core crates
pub use lumina_core as core;
pub use lumina_render as render;
pub use lumina_scene as scene;
pub use lumina_asset as asset;
pub use lumina_script as script;

// Re-export core types
pub use lumina_core::prelude::*;

/// Версия движка
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude
pub mod prelude {
    pub use crate::{
        // Core
        App, Plugin, 
        CorePlugin, RenderPlugin, ScenePlugin, AssetPlugin, ScriptPlugin, EditorPlugin,
        
        // Resources
        Time, Timer, DeltaTime,
        Commands, Res, ResMut, Local,
        
        // ECS
        Component, Entity, Query, World, 
        With, Without, Added, Changed,
        
        // Systems
        IntoSystem, System, Condition,
        Startup, Update, FixedUpdate, PostUpdate,
        
        // Math
        Vec2, Vec3, Vec4, Mat3, Mat4, Quat,
        Transform, GlobalTransform, 
        Rect, AABB, Ray, Sphere, Plane, Frustum,
        
        // Color
        Color, Color32,
        
        // Events
        Event, EventReader, EventWriter,
        
        // Render
        Camera, CameraProjection, PerspectiveProjection, OrthographicProjection,
        Light, LightType, AmbientLight, EnvironmentLight, ShadowSettings,
        Mesh, MeshBuilder, Vertex, IndexFormat, PrimitiveTopology,
        StandardMaterial, Material, MaterialHandle,
        Texture, TextureHandle, TextureFormat,
        Shader, ShaderHandle,
        RenderPipeline, PipelineLayout,
        
        // Scene
        Scene, SceneHandle, SceneBuilder,
        Node, NodeId, NodeRef, NodeMut,
        Hierarchy, Parent, Children, Root,
        
        // Asset
        Asset, AssetLoader, AssetHandle, AssetManager,
        
        // Script
        Script, ScriptHandle, ScriptInstance,
        
        // Bundles
        TransformBundle, RenderBundle, PhysicsBundle,
        
        // Utils
        URect, UVec2, UVec3, UVec4,
        IVec2, IVec4,
        BVec2, BVec3, BVec4,
        DVec2, DVec3, DVec4,
    };
}

// Core types placeholder definitions
pub struct App;
pub trait Plugin {}
pub struct CorePlugin;
pub struct RenderPlugin;
pub struct ScenePlugin;
pub struct AssetPlugin;
pub struct ScriptPlugin;
pub struct EditorPlugin;

pub struct Commands;
pub struct Res<T>(std::marker::PhantomData<T>);
pub struct ResMut<T>(std::marker::PhantomData<T>);
pub struct Local<T>(std::marker::PhantomData<T>);

pub trait Component {}
pub struct Entity;
pub struct Query<T>(std::marker::PhantomData<T>);
pub struct World;
pub struct With<T>(std::marker::PhantomData<T>);
pub struct Without<T>(std::marker::PhantomData<T>);
pub struct Added<T>(std::marker::PhantomData<T>);
pub struct Changed<T>(std::marker::PhantomData<T>);

pub trait IntoSystem {}
pub trait System {}
pub trait Condition {}
pub struct Startup;
pub struct Update;
pub struct FixedUpdate;
pub struct PostUpdate;

pub struct GlobalTransform;

pub struct URect;
pub struct UVec2;
pub struct UVec3;
pub struct UVec4;
pub struct IVec2;
pub struct IVec4;
pub struct BVec2;
pub struct BVec3;
pub struct BVec4;
pub struct DVec2;
pub struct DVec3;
pub struct DVec4;

// Render types
pub struct MeshBuilder;
impl MeshBuilder {
    pub fn cube(size: f32) -> Mesh { Mesh }
    pub fn sphere(radius: f32, segments: u32, rings: u32) -> Mesh { Mesh }
    pub fn plane(width: f32, height: f32) -> Mesh { Mesh }
}

pub struct Vertex;
pub struct IndexFormat;
pub struct PrimitiveTopology;
pub struct MaterialHandle;
pub struct TextureHandle;
pub struct ShaderHandle;
pub struct RenderPipeline;
pub struct PipelineLayout;

// Scene types
pub struct SceneHandle;
pub struct SceneBuilder;
pub struct NodeRef;
pub struct NodeMut;
pub struct Hierarchy;
pub struct Parent;
pub struct Children;
pub struct Root;

// Asset types
pub trait Asset {}
pub trait AssetLoader {}
pub struct AssetManager;

// Script types
pub trait Script {}
pub struct ScriptHandle;
pub struct ScriptInstance;

// Bundles
pub struct TransformBundle;
pub struct RenderBundle;
pub struct PhysicsBundle;

// Additional types
pub struct Time;
pub struct Timer;
pub struct DeltaTime;

impl App {
    pub fn new() -> Self { Self }
    pub fn add_plugin<T: Plugin>(self, _plugin: T) -> Self { self }
    pub fn add_systems<S, T: IntoSystem>(self, _stage: S, _systems: T) -> Self { self }
    pub fn run(self) -> Result<(), ()> { Ok(()) }
}

impl Plugin for CorePlugin {}
impl Plugin for RenderPlugin {}
impl Plugin for ScenePlugin {}
impl Plugin for AssetPlugin {}
impl Plugin for ScriptPlugin {}
impl Plugin for EditorPlugin {}

// Re-export glam types
pub use glam::*;
