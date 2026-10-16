//! # Scene - Сцена
//! 
//! Контейнер для игровых объектов и их компонентов.

use crate::{Node, NodeId, Hierarchy};
use lumina_core::World;
use std::collections::HashMap;

/// Сцена - контейнер игровых объектов
#[derive(Debug)]
pub struct Scene {
    /// Имя сцены
    pub name: String,
    /// Корневые узлы
    pub roots: Vec<NodeId>,
    /// Все узлы
    nodes: HashMap<NodeId, Node>,
    /// Иерархия
    pub hierarchy: Hierarchy,
    /// ECS мир
    pub world: World,
    /// Следующий ID
    next_id: u64,
}

impl Default for Scene {
    fn default() -> Self {
        Self::new("Untitled")
    }
}

impl Scene {
    /// Создать новую сцену
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            roots: Vec::new(),
            nodes: HashMap::new(),
            hierarchy: Hierarchy::new(),
            world: World::new(),
            next_id: 1,
        }
    }

    /// Создать узел
    pub fn create_node(&mut self, name: impl Into<String>) -> NodeId {
        let id = NodeId::from(self.next_id);
        self.next_id += 1;

        let node = Node::new(name);
        self.nodes.insert(id, node);
        self.hierarchy.add_node(id);

        // Добавляем в корневые если нет родителя
        if !self.roots.contains(&id) {
            self.roots.push(id);
        }

        id
    }

    /// Удалить узел
    pub fn remove_node(&mut self, id: NodeId) {
        // Удаляем детей рекурсивно
        if let Some(node) = self.nodes.get(&id) {
            let children: Vec<_> = node.children.clone();
            for child in children {
                self.remove_node(child);
            }
        }

        // Удаляем из родителя
        if let Some(node) = self.nodes.get(&id) {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.nodes.get_mut(&parent_id) {
                    parent.remove_child(id);
                }
            }
        }

        // Удаляем из корневых
        if let Some(pos) = self.roots.iter().position(|&r| r == id) {
            self.roots.remove(pos);
        }

        // Удаляем узел
        self.nodes.remove(&id);
        self.hierarchy.remove_node(id);
    }

    /// Получить узел
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Получить mutable узел
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Установить родителя
    pub fn set_parent(&mut self, child: NodeId, parent: Option<NodeId>) {
        // Удаляем из старого родителя
        if let Some(node) = self.nodes.get(&child) {
            if let Some(old_parent) = node.parent {
                if let Some(p) = self.nodes.get_mut(&old_parent) {
                    p.remove_child(child);
                }
            }
        }

        // Удаляем из корневых если есть родитель
        if parent.is_some() {
            if let Some(pos) = self.roots.iter().position(|&r| r == child) {
                self.roots.remove(pos);
            }
        } else {
            // Добавляем в корневые если нет родителя
            if !self.roots.contains(&child) {
                self.roots.push(child);
            }
        }

        // Устанавливаем нового родителя
        if let Some(node) = self.nodes.get_mut(&child) {
            node.set_parent(parent);
        }

        // Добавляем к новому родителю
        if let Some(p) = parent {
            if let Some(parent_node) = self.nodes.get_mut(&p) {
                parent_node.add_child(child);
            }
        }

        self.hierarchy.set_parent(child, parent);
    }

    /// Итерация по всем узлам
    pub fn iter_nodes(&self) -> impl Iterator<Item = (NodeId, &Node)> {
        self.nodes.iter().map(|(&id, node)| (id, node))
    }

    /// Итерация по корневым узлам
    pub fn iter_roots(&self) -> impl Iterator<Item = &Node> {
        self.roots.iter().filter_map(|&id| self.nodes.get(&id))
    }

    /// Глубина узла в иерархии
    pub fn depth(&self, node: NodeId) -> usize {
        self.hierarchy.depth(node)
    }

    /// Проверить является ли предком
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool {
        self.hierarchy.is_ancestor(ancestor, descendant)
    }

    /// Получить путь от корня
    pub fn path(&self, node: NodeId) -> Vec<NodeId> {
        self.hierarchy.path_to_root(node)
    }
}