//! # ECS (Entity Component System)
//! 
//! Легкая, но мощная ECS система на основе hecs.
//! 
//! ## Пример использования:
//! ```rust
//! use lumina_core::prelude::*;
//! 
//! let mut world = World::new();
//! let entity = world.spawn((
//!     Transform::default(),
//!     Mesh::default(),
//! ));
//! 
//! // Query entities
//! for (id, (transform, mesh)) in world.query::<(&Transform, &Mesh)>() {
//!     // Process...
//! }
//! ```

use hecs::{Entity, World as HecsWorld};
use std::any::TypeId;
use parking_lot::RwLock;
use slotmap::{SlotMap, new_key_type};

pub use hecs::{Query, QueryBorrow, Ref, RefMut, Without, With};

/// Уникальный идентификатор системы
new_key_type! { pub struct SystemId; }

/// Мир ECS - контейнер всех сущностей и компонентов
pub struct World {
    inner: HecsWorld,
    archetypes_changed: bool,
    systems: SlotMap<SystemId, Box<dyn System>>,
    resources: RwLock<ResourceStorage>,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Создать новый пустой мир
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: HecsWorld::new(),
            archetypes_changed: true,
            systems: SlotMap::with_key(),
            resources: RwLock::new(ResourceStorage::new()),
        }
    }

    /// Создать сущность с заданными компонентами
    #[inline]
    pub fn spawn<B>(&mut self, bundle: B) -> Entity
    where
        B: hecs::DynamicBundle,
    {
        self.archetypes_changed = true;
        self.inner.spawn(bundle)
    }

    /// Удалить сущность
    #[inline]
    pub fn despawn(&mut self, entity: Entity) -> Result<(), hecs::NoSuchEntity> {
        self.inner.despawn(entity)
    }

    /// Проверить существует ли сущность
    #[inline]
    pub fn contains(&self, entity: Entity) -> bool {
        self.inner.contains(entity)
    }

    /// Получить компонент
    #[inline]
    pub fn get<T: hecs::Component>(&self, entity: Entity) -> Option<Ref<T>> {
        self.inner.get(entity).ok()
    }

    /// Получить mutable компонент
    #[inline]
    pub fn get_mut<T: hecs::Component>(&self, entity: Entity) -> Option<RefMut<T>> {
        self.inner.get_mut(entity).ok()
    }

    /// Вставить компонент
    #[inline]
    pub fn insert<B: hecs::DynamicBundle>(&mut self, entity: Entity, bundle: B) -> Result<(), hecs::MissingComponent> {
        self.archetypes_changed = true;
        self.inner.insert(entity, bundle)
    }

    /// Удалить компонент
    #[inline]
    pub fn remove<T: hecs::Component>(&mut self, entity: Entity) -> Result<T, hecs::ComponentError> {
        self.archetypes_changed = true;
        self.inner.remove::<T>(entity)
    }

    /// Query компонентов
    #[inline]
    pub fn query<Q: Query>(&self) -> hecs::QueryBorrow<Q> {
        self.inner.query::<Q>()
    }

    /// Query с фильтром
    #[inline]
    pub fn query_mut<Q: Query>(&mut self) -> hecs::QueryMut<Q> {
        self.inner.query_mut::<Q>()
    }

    /// Получить количество сущностей
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Проверить пустой ли мир
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Очистить мир
    #[inline]
    pub fn clear(&mut self) {
        self.inner.clear();
        self.archetypes_changed = true;
    }

    /// Добавить систему
    #[inline]
    pub fn add_system<S: System>(&mut self, system: S) -> SystemId {
        self.systems.insert(Box::new(system))
    }

    /// Удалить систему
    #[inline]
    pub fn remove_system(&mut self, id: SystemId) -> Option<Box<dyn System>> {
        self.systems.remove(id)
    }

    /// Выполнить все системы
    #[inline]
    pub fn run_systems(&mut self) {
        let system_ids: Vec<SystemId> = self.systems.keys().collect();
        for id in system_ids {
            if let Some(system) = self.systems.get_mut(id) {
                system.run(self);
            }
        }
    }

    /// Добавить ресурс
    #[inline]
    pub fn insert_resource<R: Resource>(&self, resource: R) {
        self.resources.write().insert(resource);
    }

    /// Получить ресурс
    #[inline]
    pub fn get_resource<R: Resource>(&self) -> Option<parking_lot::RwLockReadGuard<R>> {
        let resources = self.resources.read();
        // Note: This is a simplified version - in production you'd use proper type maps
        None
    }
}

/// Трейт для систем
pub trait System: Send + Sync {
    fn run(&mut self, world: &mut World);
    fn name(&self) -> &str;
}

/// Маркерный трейт для ресурсов
pub trait Resource: Send + Sync + 'static {}

impl<T: Send + Sync + 'static> Resource for T {}

/// Хранилище ресурсов
pub struct ResourceStorage {
    // Simplified - would use TypeIdMap in production
}

impl ResourceStorage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        // Implementation would store in type map
        let _ = resource;
    }
}

/// Макрос для системы
#[macro_export]
macro_rules! system {
    ($name:expr, |$world:ident| $body:expr) => {
        struct __System;
        impl $crate::System for __System {
            fn run(&mut self, $world: &mut $crate::World) {
                $body
            }
            fn name(&self) -> &str { $name }
        }
        __System
    };
}

/// Стадия выполнения систем
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stage {
    /// Первоначальная инициализация
    Startup,
    /// Предобработка кадра
    PreUpdate,
    /// Основное обновление
    Update,
    /// Постобработка
    PostUpdate,
    /// Рендеринг
    Render,
}

impl Default for Stage {
    fn default() -> Self {
        Stage::Update
    }
}