//! # Lumina Scene
//! 
//! Сцена - иерархия объектов, компоненты, сериализация.

pub mod scene;
pub mod node;
pub mod hierarchy;
pub mod component;
pub mod bundle;

pub use scene::*;
pub use node::*;
pub use hierarchy::*;
pub use component::*;
pub use bundle::*;

/// Prelude
pub mod prelude {
    pub use crate::{
        Scene, SceneBuilder,
        Node, NodeId, NodeRef, NodeMut,
        Hierarchy, Parent, Children, Root,
        Component, ComponentStorage,
        TransformBundle, RenderBundle, PhysicsBundle,
    };
}