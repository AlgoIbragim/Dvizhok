//! # Lumina Core
//! 
//! Ядро движка Lumina - ECS, математика и основные абстракции.
//! 
//! ## Философия
//! - Минимум кода, максимум функционала
//! - Каждая строчка имеет смысл
//! - Zero-cost abstractions

pub mod ecs;
pub mod math;
pub mod time;
pub mod event;
pub mod transform;
pub mod color;

pub use ecs::*;
pub use math::*;
pub use time::*;
pub use event::*;
pub use transform::*;
pub use color::*;

/// Версия движка
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude для удобного импорта
pub mod prelude {
    pub use crate::{
        Color, Color32, Transform, Quat, Vec2, Vec3, Vec4, 
        Mat3, Mat4, Rect, AABB, Ray,
        Component, Entity, Query, World, Commands,
        EventReader, EventWriter, Event,
        Time, Timer, DeltaTime,
        System, IntoSystem, Stage,
    };
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Entity not found: {0}")]
    EntityNotFound(String),
    
    #[error("Component not found: {0}")]
    ComponentNotFound(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type CoreResult<T> = Result<T, CoreError>;