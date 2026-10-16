//! 2D Platformer Example - Пример 2D платформера
//! 
//! Показывает:
//! - 2D рендеринг (спрайты)
//! - Физика платформера
//! - Анимации
//! - Ввод с клавиатуры

use lumina::prelude::*;

fn main() {
    App::new()
        .with_title("2D Platformer Example")
        .with_size(1280, 720)
        .with_vsync(true)
        .add_plugin(CorePlugin)
        .add_plugin(RenderPlugin {
            render_mode: RenderMode::Forward,
            ..Default::default()
        })
        .add_plugin(Physics2DPlugin {
            gravity: Vec2::new(0.0, -9.81),
        })
        .add_systems(Startup, setup_level)
        .add_systems(Update, (player_movement, camera_follow))
        .run();
}

/// Компонент игрока
#[derive(Debug, Clone, Copy, PartialEq)]
struct Player {
    speed: f32,
    jump_force: f32,
    on_ground: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 5.0,
            jump_force: 10.0,
            on_ground: false,
        }
    }
}

/// Компонент платформы
#[derive(Debug, Clone, Copy, PartialEq)]
struct Platform;

/// Компонент монетки
#[derive(Debug, Clone, Copy, PartialEq)]
struct Coin {
    value: u32,
}

/// Система создания уровня
fn setup_level(
    mut commands: Commands,
    mut sprites: ResMut<SpriteAssets>,
) {
    // Загружаем спрайты
    let player_sprite = sprites.load("sprites/player.png");
    let platform_sprite = sprites.load("sprites/platform.png");
    let coin_sprite = sprites.load("sprites/coin.png");
    let background = sprites.load("sprites/background.png");

    // Создаем камеру
    commands.spawn((
        Camera2D::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Фон
    commands.spawn((
        Sprite::new(background),
        Transform::from_xyz(0.0, 0.0, -10.0)
            .with_scale(Vec3::new(20.0, 15.0, 1.0)),
    ));

    // Создаем игрока
    commands.spawn((
        Player::default(),
        Sprite::new(player_sprite),
        Transform::from_xyz(0.0, 2.0, 0.0),
        RigidBody2D::dynamic()
            .with_mass(1.0)
            .with_linear_damping(0.5),
        Collider2D::rectangle(0.8, 1.6),
    ));

    // Создаем платформы
    let platforms = [
        (0.0, -2.0, 8.0, 1.0),   // ground
        (-4.0, 1.0, 2.0, 0.5),   // left platform
        (4.0, 1.0, 2.0, 0.5),    // right platform
        (0.0, 4.0, 3.0, 0.5),    // high center platform
        (-6.0, 5.0, 1.5, 0.5),   // far left
        (6.0, 5.0, 1.5, 0.5),    // far right
    ];

    for (x, y, w, h) in platforms {
        commands.spawn((
            Platform,
            Sprite::new(platform_sprite),
            Transform::from_xyz(x, y, 0.0)
                .with_scale(Vec3::new(w, h, 1.0)),
            Collider2D::rectangle(w, h).as_static(),
        ));
    }

    // Создаем монетки
    let coin_positions = [
        (-4.0, 2.5), (4.0, 2.5), (0.0, 5.5),
        (-6.0, 6.5), (6.0, 6.5), (0.0, 0.0),
    ];

    for (x, y) in coin_positions {
        commands.spawn((
            Coin { value: 10 },
            Sprite::new(coin_sprite),
            Transform::from_xyz(x, y, 0.0),
            Collider2D::circle(0.3).as_sensor(),
            Rotator::new(2.0),
        ));
    }
}

/// Система движения игрока
fn player_movement(
    time: Res<Time>,
    keyboard: Res<KeyboardInput>,
    mut query: Query<(&mut Transform, &mut Player, &mut RigidBody2D)>,
) {
    for (mut transform, mut player, mut rb) in query.iter_mut() {
        let dt = time.delta_secs();

        // Проверяем на земле ли (упрощенная проверка)
        player.on_ground = transform.translation.y <= 0.1;

        // Горизонтальное движение
        let mut velocity = rb.linear_velocity();
        
        if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left) {
            velocity.x = -player.speed;
            // Отражаем спрайт
            transform.scale.x = -transform.scale.x.abs();
        } else if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right) {
            velocity.x = player.speed;
            transform.scale.x = transform.scale.x.abs();
        } else {
            // Торможение
            velocity.x *= 0.8;
        }

        // Прыжок
        if (keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::W) || keyboard.just_pressed(KeyCode::Up)) 
            && player.on_ground {
            velocity.y = player.jump_force;
            player.on_ground = false;
        }

        rb.set_linear_velocity(velocity);
    }
}

/// Система следования камеры за игроком
fn camera_follow(
    mut camera_query: Query<(&Camera, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (_camera, mut camera_transform) in camera_query.iter_mut() {
            // Плавное следование
            let target_pos = Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y + 2.0,
                camera_transform.translation.z,
            );
            
            let t = 1.0 - (-5.0 * time.delta_secs()).exp();
            camera_transform.translation = camera_transform.translation.lerp(target_pos, t);
        }
    }
}

/// Компонент для вращения (для монеток)
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rotator {
    speed: f32,
    time: f32,
}

impl Rotator {
    fn new(speed: f32) -> Self {
        Self { speed, time: 0.0 }
    }
}

/// Плагин физики 2D (placeholder)
struct Physics2DPlugin {
    gravity: Vec2,
}

struct RigidBody2D;
struct Collider2D;

impl RigidBody2D {
    fn dynamic() -> Self { Self }
    fn with_mass(self, _mass: f32) -> Self { self }
    fn with_linear_damping(self, _damping: f32) -> Self { self }
    fn linear_velocity(&self) -> Vec3 { Vec3::ZERO }
    fn set_linear_velocity(&mut self, _velocity: Vec3) {}
}

impl Collider2D {
    fn rectangle(_width: f32, _height: f32) -> Self { Self }
    fn circle(_radius: f32) -> Self { Self }
    fn as_static(self) -> Self { self }
    fn as_sensor(self) -> Self { self }
}

struct Sprite;
impl Sprite {
    fn new(_handle: Handle<Image>) -> Self { Self }
}

struct Image;
struct SpriteAssets;
impl SpriteAssets {
    fn load(&mut self, _path: &str) -> Handle<Image> { Handle::default() }
}

struct Camera2D;
impl Default for Camera2D {
    fn default() -> Self { Self }
}

struct KeyCode;
impl KeyCode {
    const A: Self = Self;
    const D: Self = Self;
    const W: Self = Self;
    const S: Self = Self;
    const Space: Self = Self;
    const Left: Self = Self;
    const Right: Self = Self;
    const Up: Self = Self;
}

struct KeyboardInput;
impl KeyboardInput {
    fn pressed(&self, _key: KeyCode) -> bool { false }
    fn just_pressed(&self, _key: KeyCode) -> bool { false }
}
