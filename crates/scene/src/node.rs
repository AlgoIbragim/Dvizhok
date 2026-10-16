//! # Node - Узел сцены
//! 
//! Узел - это сущность в иерархии сцены с трансформом и компонентами.

use lumina_core::{Entity, Transform};
use slotmap::{SlotMap, new_key_type};
use std::collections::HashSet;

/// Идентификатор узла
new_key_type! { pub struct NodeId; }

/// Узел сцены
#[derive(Debug, Clone)]
pub struct Node {
    /// Имя узла
    pub name: String,
    /// Локальная трансформация
    pub transform: Transform,
    /// ID родителя
    pub parent: Option<NodeId>,
    /// ID детей
    pub children: Vec<NodeId>,
    /// Флаги активности
    pub active: bool,
    /// Флаги видимости
    pub visible: bool,
    /// Пользовательские данные
    pub tag: u32,
    /// Слой
    pub layer: u32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "Node".to_string(),
            transform: Transform::default(),
            parent: None,
            children: Vec::new(),
            active: true,
            visible: true,
            tag: 0,
            layer: 0,
        }
    }
}

impl Node {
    /// Создать новый узел
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Создать с трансформацией
    #[inline]
    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    /// Создать с родителем
    #[inline]
    pub fn with_parent(mut self, parent: NodeId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Активен ли
    #[inline]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Видим ли
    #[inline]
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Есть ли родитель
    #[inline]
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// Есть ли дети
    #[inline]
    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    /// Количество детей
    #[inline]
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Добавить ребенка
    #[inline]
    pub fn add_child(&mut self, child: NodeId) {
        if !self.children.contains(&child) {
            self.children.push(child);
        }
    }

    /// Удалить ребенка
    #[inline]
    pub fn remove_child(&mut self, child: NodeId) -> bool {
        if let Some(pos) = self.children.iter().position(|&c| c == child) {
            self.children.remove(pos);
            true
        } else {
            false
        }
    }

    /// Установить родителя
    #[inline]
    pub fn set_parent(&mut self, parent: Option<NodeId>) {
        self.parent = parent;
    }

    /// Отсоединить от родителя
    #[inline]
    pub fn detach(&mut self) {
        self.parent = None;
    }

    /// Включить
    #[inline]
    pub fn enable(&mut self) {
        self.active = true;
    }

    /// Выключить
    #[inline]
    pub fn disable(&mut self) {
        self.active = false;
    }

    /// Показать
    #[inline]
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Скрыть
    #[inline]
    pub fn hide(&mut self) {
        self.visible = false;
    }
}

/// Ссылка на узел
pub struct NodeRef<'a> {
    pub id: NodeId,
    pub node: &'a Node,
}

/// Mutable ссылка на узел
pub struct NodeMut<'a> {
    pub id: NodeId,
    pub node: &'a mut Node,
}