# Lumina Engine 🎮

Легкий, но мощный 2D/3D игровой движок на Rust с акцентом на простоту использования и производительность.

## ✨ Особенности

### 🎯 Простота использования
- Интуитивный API, вдохновленный Bevy и Unity
- Минимум шаблонного кода
- Горячая перезагрузка скриптов и ресурсов

### ⚡ Производительность
- Модульная ECS архитектура (hecs)
- Современный рендерер на wgpu
- Многопоточная обработка систем

### 🎨 Рендеринг
- PBR материалы
- Направленное, точечное, прожекторное освещение
- Тени (CSM, VSM)
- HDR и постобработка
- Deferred/Forward/Forward+ рендеринг

### 🛠️ Инструменты
- Встроенный редактор с UI (egui)
- Инспектор сущностей
- Scene view с gizmos
- Asset browser
- Консоль отладки

## 🚀 Быстрый старт

### Установка

```bash
# Добавьте в Cargo.toml
[dependencies]
lumina = "0.1"
```

### Простой пример

```rust
use lumina::prelude::*;

fn main() {
    App::new()
        .with_title("My Game")
        .with_size(1280, 720)
        .add_plugin(CorePlugin)
        .add_plugin(RenderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cubes)
        .run();
}

fn setup(mut commands: Commands) {
    // Камера
    commands.spawn(Camera::perspective(
        45f32.to_radians(), 
        16.0/9.0, 
        0.1, 
        1000.0
    ));

    // Куб
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        Mesh::cube(1.0),
        StandardMaterial {
            base_color: Color::RED,
            metallic: 0.5,
            roughness: 0.5,
        },
    ));

    // Свет
    commands.spawn((
        Light::directional(Vec3::new(-1.0, -1.0, -1.0), Color::WHITE, 1.0),
        Transform::default(),
    ));
}

fn rotate_cubes(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Mesh>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(time.delta_secs());
    }
}
```

## 📁 Структура проекта

```
lumina/
├── crates/
│   ├── core/          # ECS, математика, события
│   ├── render/        # wgpu рендерер
│   ├── scene/         # Иерархия сцен
│   ├── asset/         # Управление ресурсами
│   ├── script/        # Скриптинг (Rust hot reload)
│   └── editor/        # Встроенный редактор
├── examples/          # Примеры использования
├── assets/            # Стандартные ресурсы
└── src/               # Основной код
```

## 🎯 Примеры

### 2D Platformer
```bash
cargo run --example 2d_platformer
```

### 3D FPS
```bash
cargo run --example 3d_fps
```

### UI Demo
```bash
cargo run --example ui_demo
```

### Physics Playground
```bash
cargo run --example physics
```

## 🔧 Системные требования

- **OS**: Windows 10+, macOS 10.15+, Linux (Ubuntu 20.04+)
- **GPU**: DirectX 12, Vulkan 1.1, или Metal compatible
- **RAM**: 4GB минимум, 8GB рекомендуется
- **Rust**: 1.75+

## 🤝 Вклад в проект

Мы приветствуем вклад! См. [CONTRIBUTING.md](CONTRIBUTING.md) для деталей.

## 📄 Лицензия

Этот проект лицензирован под MIT или Apache-2.0 на ваш выбор.

## 🙏 Благодарности

- [Bevy](https://bevyengine.org/) - вдохновение для ECS и архитектуры
- [wgpu](https://wgpu.rs/) - кросс-платформенный рендерер
- [hecs](https://github.com/Ralith/hecs) - быстрая ECS библиотека
- [egui](https://github.com/emilk/egui) - immediate mode GUI

---

**Made with ❤️ in Rust**