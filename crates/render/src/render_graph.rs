//! Рендер граф - система управления рендер проходами

use std::collections::HashMap;
use slotmap::{SlotMap, new_key_type};

new_key_type! { pub struct NodeId; }
new_key_type! { pub struct EdgeId; }

/// Рендер граф
#[derive(Debug)]
pub struct RenderGraph {
    nodes: SlotMap<NodeId, Node>,
    edges: SlotMap<EdgeId, Edge>,
    inputs: HashMap<String, ResourceId>,
    outputs: HashMap<String, ResourceId>,
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderGraph {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    /// Добавить ноду
    pub fn add_node(&mut self, name: impl Into<String>) -> NodeId {
        self.nodes.insert(Node {
            name: name.into(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            run_fn: None,
        })
    }

    /// Добавить связь между нодами
    pub fn add_edge(
        &mut self,
        from: NodeId,
        to: NodeId,
        from_output: impl Into<String>,
        to_input: impl Into<String>,
    ) -> EdgeId {
        let edge = Edge {
            from,
            to,
            from_output: from_output.into(),
            to_input: to_input.into(),
        };

        let edge_id = self.edges.insert(edge);

        if let Some(node) = self.nodes.get_mut(from) {
            node.outputs.push(edge_id);
        }
        if let Some(node) = self.nodes.get_mut(to) {
            node.inputs.push(edge_id);
        }

        edge_id
    }

    /// Установить функцию выполнения для ноды
    pub fn set_node_run_fn(
        &mut self,
        node_id: NodeId,
        run_fn: Box<dyn FnMut(&mut RenderContext) + Send + Sync>,
    ) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.run_fn = Some(run_fn);
        }
    }

    /// Получить ноду
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Получить mutable ноду
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Получить все ноды
    pub fn nodes(&self) -> &SlotMap<NodeId, Node> {
        &self.nodes
    }

    /// Получить все связи
    pub fn edges(&self) -> &SlotMap<EdgeId, Edge> {
        &self.edges
    }

    /// Топологическая сортировка для выполнения
    pub fn topological_sort(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut visited = hashbrown::HashSet::new();
        let mut temp_marked = hashbrown::HashSet::new();

        for (id, _) in &self.nodes {
            if !visited.contains(&id) {
                self.visit_node(id, &mut visited, &mut temp_marked, &mut result);
            }
        }

        result
    }

    fn visit_node(
        &self,
        node_id: NodeId,
        visited: &mut hashbrown::HashSet<NodeId>,
        temp_marked: &mut hashbrown::HashSet<NodeId>,
        result: &mut Vec<NodeId>,
    ) {
        if temp_marked.contains(&node_id) {
            panic!("Cycle detected in render graph");
        }

        if visited.contains(&node_id) {
            return;
        }

        temp_marked.insert(node_id);

        if let Some(node) = self.nodes.get(node_id) {
            for edge_id in &node.outputs {
                if let Some(edge) = self.edges.get(*edge_id) {
                    self.visit_node(edge.to, visited, temp_marked, result);
                }
            }
        }

        temp_marked.remove(&node_id);
        visited.insert(node_id);
        result.push(node_id);
    }
}

/// Нода рендер графа
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub inputs: Vec<EdgeId>,
    pub outputs: Vec<EdgeId>,
    pub run_fn: Option<Box<dyn FnMut(&mut RenderContext) + Send + Sync>>,
}

/// Связь между нодами
#[derive(Debug)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub from_output: String,
    pub to_input: String,
}

/// Контекст рендеринга
#[derive(Debug)]
pub struct RenderContext {
    // pub encoder: wgpu::CommandEncoder,
    // pub device: Arc<wgpu::Device>,
    // pub queue: Arc<wgpu::Queue>,
    // pub surface: Arc<wgpu::Surface>,
    pub frame: Option<wgpu::SurfaceTexture>,
    pub view: Option<wgpu::TextureView>,
    pub depth_view: Option<wgpu::TextureView>,
    pub resources: hashbrown::HashMap<ResourceId, RenderResource>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            frame: None,
            view: None,
            depth_view: None,
            resources: hashbrown::HashMap::new(),
        }
    }

    pub fn add_resource(&mut self, id: ResourceId, resource: RenderResource) {
        self.resources.insert(id, resource);
    }

    pub fn get_resource(&self, id: ResourceId) -> Option<&RenderResource> {
        self.resources.get(&id)
    }

    pub fn get_resource_mut(&mut self, id: ResourceId) -> Option<&mut RenderResource> {
        self.resources.get_mut(&id)
    }
}

/// ID ресурса
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(pub u64);

/// Рендер ресурс
#[derive(Debug)]
pub enum RenderResource {
    Texture(wgpu::Texture),
    TextureView(wgpu::TextureView),
    Buffer(wgpu::Buffer),
    Sampler(wgpu::Sampler),
    BindGroup(wgpu::BindGroup),
    BindGroupLayout(wgpu::BindGroupLayout),
    PipelineLayout(wgpu::PipelineLayout),
    RenderPipeline(wgpu::RenderPipeline),
    ComputePipeline(wgpu::ComputePipeline),
}

/// Построитель рендер графа
pub struct RenderGraphBuilder {
    graph: RenderGraph,
}

impl RenderGraphBuilder {
    pub fn new() -> Self {
        Self {
            graph: RenderGraph::new(),
        }
    }

    /// Добавить ноду
    pub fn add_node(mut self, name: impl Into<String>) -> NodeBuilder {
        let id = self.graph.add_node(name);
        NodeBuilder {
            graph_builder: self,
            node_id: id,
        }
    }

    /// Собрать граф
    pub fn build(self) -> RenderGraph {
        self.graph
    }
}

/// Построитель ноды
pub struct NodeBuilder {
    graph_builder: RenderGraphBuilder,
    node_id: NodeId,
}

impl NodeBuilder {
    /// Добавить вход
    pub fn input(mut self, name: impl Into<String>, resource: ResourceId) -> Self {
        // Store input reference
        self
    }

    /// Добавить выход
    pub fn output(mut self, name: impl Into<String>, resource: ResourceId) -> Self {
        // Store output reference
        self
    }

    /// Установить функцию выполнения
    pub fn run<F>(mut self, f: F) -> RenderGraphBuilder
    where
        F: FnMut(&mut RenderContext) + Send + Sync + 'static,
    {
        self.graph_builder.graph.set_node_run_fn(
            self.node_id,
            Box::new(f),
        );
        self.graph_builder
    }
}

impl Default for RenderGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}
