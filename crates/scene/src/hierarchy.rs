//! # Hierarchy - Иерархия узлов
//! 
//! Управление отношениями родитель-потомок.

use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;

/// ID узла в иерархии
new_key_type! { pub struct NodeId; }

/// Иерархия узлов
#[derive(Debug, Default)]
pub struct Hierarchy {
    /// Связи родитель-потомок
    parent: HashMap<NodeId, NodeId>,
    /// Дети каждого узла
    children: HashMap<NodeId, Vec<NodeId>>,
}

impl Hierarchy {
    /// Создать пустую иерархию
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавить узел
    pub fn add_node(&mut self, id: NodeId) {
        // Узел добавляется без родителя
        // Дети добавляются отдельно
    }

    /// Удалить узел
    pub fn remove_node(&mut self, id: NodeId) {
        // Удаляем из родителя
        if let Some(parent) = self.parent.remove(&id) {
            if let Some(children) = self.children.get_mut(&parent) {
                children.retain(|&child| child != id);
            }
        }

        // Удаляем связи с детьми
        if let Some(children) = self.children.remove(&id) {
            for child in children {
                self.parent.remove(&child);
            }
        }
    }

    /// Установить родителя
    pub fn set_parent(&mut self, child: NodeId, parent: Option<NodeId>) {
        // Удаляем из старого родителя
        if let Some(old_parent) = self.parent.remove(&child) {
            if let Some(children) = self.children.get_mut(&old_parent) {
                children.retain(|&c| c != child);
            }
        }

        // Устанавливаем нового родителя
        if let Some(p) = parent {
            self.parent.insert(child, p);
            self.children.entry(p).or_default().push(child);
        }
    }

    /// Получить родителя
    pub fn parent(&self, id: NodeId) -> Option<NodeId> {
        self.parent.get(&id).copied()
    }

    /// Получить детей
    pub fn children(&self, id: NodeId) -> &[NodeId] {
        self.children.get(&id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Есть ли дети
    pub fn has_children(&self, id: NodeId) -> bool {
        self.children.get(&id).map(|v| !v.is_empty()).unwrap_or(false)
    }

    /// Есть ли родитель
    pub fn has_parent(&self, id: NodeId) -> bool {
        self.parent.contains_key(&id)
    }

    /// Является ли предком
    pub fn is_ancestor(&self, ancestor: NodeId, descendant: NodeId) -> bool {
        let mut current = descendant;
        while let Some(parent) = self.parent(current) {
            if parent == ancestor {
                return true;
            }
            current = parent;
        }
        false
    }

    /// Является ли потомком
    pub fn is_descendant(&self, descendant: NodeId, ancestor: NodeId) -> bool {
        self.is_ancestor(ancestor, descendant)
    }

    /// Глубина узла (корень = 0)
    pub fn depth(&self, id: NodeId) -> usize {
        let mut depth = 0;
        let mut current = id;
        
        while let Some(parent) = self.parent(current) {
            depth += 1;
            current = parent;
        }
        
        depth
    }

    /// Путь от узла до корня
    pub fn path_to_root(&self, id: NodeId) -> Vec<NodeId> {
        let mut path = vec![id];
        let mut current = id;
        
        while let Some(parent) = self.parent(current) {
            path.push(parent);
            current = parent;
        }
        
        path
    }

    /// Путь от одного узла до другого (вверх)
    pub fn path_between(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
        if !self.is_ancestor(to, from) {
            return None;
        }

        let mut path = vec![];
        let mut current = from;

        while current != to {
            path.push(current);
            current = self.parent(current)?;
        }
        path.push(to);

        Some(path)
    }

    /// Итерация по всем узлам в глубину (DFS)
    pub fn dfs(&self, root: NodeId) -> Vec<NodeId> {
        let mut result = vec![];
        let mut stack = vec![root];

        while let Some(node) = stack.pop() {
            result.push(node);
            let children: Vec<_> = self.children(node).iter().copied().rev().collect();
            stack.extend(children);
        }

        result
    }

    /// Итерация по всем узлам в ширину (BFS)
    pub fn bfs(&self, root: NodeId) -> Vec<NodeId> {
        let mut result = vec![];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root);

        while let Some(node) = queue.pop_front() {
            result.push(node);
            for &child in self.children(node) {
                queue.push_back(child);
            }
        }

        result
    }
}