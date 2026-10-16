//! Lumina Editor - Встроенный редактор с UI

pub mod ui;
pub mod inspector;
pub mod viewport;
pub mod asset_browser;
pub mod console;

use lumina_core::{App, Plugin, World, EventReader};
use lumina_render::RenderContext;
use lumina_scene::Scene;

/// Плагин редактора
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorState>()
            .add_systems(Startup, setup_editor)
            .add_systems(Update, (
                update_ui,
                handle_editor_input,
                draw_gizmos,
            ));
    }
}

/// Состояние редактора
#[derive(Debug, Default)]
pub struct EditorState {
    pub selected_entity: Option<u64>,
    pub viewport_size: (u32, u32),
    pub show_inspector: bool,
    pub show_hierarchy: bool,
    pub show_console: bool,
    pub gizmo_mode: GizmoMode,
    pub is_playing: bool,
}

/// Режим гизмо
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum GizmoMode {
    #[default]
    Translate,
    Rotate,
    Scale,
}

fn setup_editor(mut editor: ResMut<EditorState>) {
    editor.show_inspector = true;
    editor.show_hierarchy = true;
    editor.show_console = false;
}

fn update_ui(world: &mut World) {
    // UI отрисовка через egui
}

fn handle_editor_input(
    mut editor: ResMut<EditorState>,
    // mut events: EventReader<EditorEvent>,
) {
    // Обработка ввода редактора
}

fn draw_gizmos(
    editor: Res<EditorState>,
    // mut gizmos: Gizmos,
) {
    // Отрисовка gizmos для выбранных объектов
}

// Re-exports
pub use ui::*;
pub use inspector::*;
pub use viewport::*;
