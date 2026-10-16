//! # Event - Система событий
//! 
//! Типобезопасная система событий для коммуникации между системами.

use std::any::{Any, TypeId};
use std::collections::VecDeque;
use flume::{Sender, Receiver, unbounded};

/// Событие
pub trait Event: Send + Sync + 'static {
    /// Имя события (для дебага)
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Читатель событий
pub struct EventReader<T: Event> {
    receiver: Receiver<T>,
    buffer: VecDeque<T>,
}

impl<T: Event> EventReader<T> {
    /// Создать читатель
    pub(crate) fn new(receiver: Receiver<T>) -> Self {
        Self {
            receiver,
            buffer: VecDeque::new(),
        }
    }

    /// Получить следующее событие
    #[inline]
    pub fn next(&mut self) -> Option<T> {
        // Сначала проверяем буфер
        if let Some(event) = self.buffer.pop_front() {
            return Some(event);
        }

        // Затем получаем из канала
        self.receiver.try_recv().ok()
    }

    /// Прочитать все доступные события
    #[inline]
    pub fn read(&mut self) -> impl Iterator<Item = T> + '_ {
        // Собираем все события в буфер
        while let Ok(event) = self.receiver.try_recv() {
            self.buffer.push_back(event);
        }

        self.buffer.drain(..)
    }

    /// Проверить есть ли события
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty() && self.receiver.is_empty()
    }

    /// Количество доступных событий
    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len() + self.receiver.len()
    }

    /// Очистить все события
    #[inline]
    pub fn clear(&mut self) {
        self.buffer.clear();
        while self.receiver.try_recv().is_ok() {}
    }
}

/// Писатель событий
#[derive(Clone)]
pub struct EventWriter<T: Event> {
    sender: Sender<T>,
}

impl<T: Event> EventWriter<T> {
    /// Создать писатель
    pub(crate) fn new(sender: Sender<T>) -> Self {
        Self { sender }
    }

    /// Отправить событие
    #[inline]
    pub fn send(&self, event: T) {
        let _ = self.sender.send(event);
    }

    /// Отправить несколько событий
    #[inline]
    pub fn send_batch(&self, events: impl IntoIterator<Item = T>) {
        for event in events {
            self.send(event);
        }
    }

    /// Проверить можно ли отправлять
    #[inline]
    pub fn can_send(&self) -> bool {
        !self.sender.is_disconnected()
    }
}

/// Центральный диспетчер событий
pub struct EventBus {
    // Type-erased channels storage
    channels: hashbrown::HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Создать новый EventBus
    #[inline]
    pub fn new() -> Self {
        Self {
            channels: hashbrown::HashMap::new(),
        }
    }

    /// Зарегистрировать тип события
    #[inline]
    pub fn register<T: Event>(&mut self) {
        let type_id = TypeId::of::<T>();
        if !self.channels.contains_key(&type_id) {
            let (tx, rx): (Sender<T>, Receiver<T>) = unbounded();
            self.channels.insert(type_id, Box::new((tx, rx)));
        }
    }

    /// Получить писатель событий
    #[inline]
    pub fn writer<T: Event>(&self) -> Option<EventWriter<T>> {
        let type_id = TypeId::of::<T>();
        self.channels.get(&type_id).and_then(|boxed| {
            let (tx, _): &(Sender<T>, Receiver<T>) = boxed.downcast_ref()?;
            Some(EventWriter::new(tx.clone()))
        })
    }

    /// Получить читатель событий
    #[inline]
    pub fn reader<T: Event>(&self) -> Option<EventReader<T>> {
        let type_id = TypeId::of::<T>();
        self.channels.get(&type_id).and_then(|boxed| {
            let (_, rx): &(Sender<T>, Receiver<T>) = boxed.downcast_ref()?;
            Some(EventReader::new(rx.clone()))
        })
    }

    /// Создать пару writer/reader
    #[inline]
    pub fn channel<T: Event>(&mut self) -> (EventWriter<T>, EventReader<T>) {
        self.register::<T>();
        (self.writer().unwrap(), self.reader().unwrap())
    }

    /// Очистить все события
    #[inline]
    pub fn clear_all(&mut self) {
        // Реализация зависит от типов
    }
}

/// Событие кадра обновления
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpdateEvent {
    pub delta_time: f32,
    pub total_time: f32,
    pub frame: u64,
}

impl Event for UpdateEvent {}

/// Событие кадра рендеринга
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderEvent {
    pub delta_time: f32,
    pub interpolation: f32,
}

impl Event for RenderEvent {}

/// Событие изменения размера окна
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32,
}

impl Event for WindowResizeEvent {}

/// Событие ввода
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    KeyPressed { key: KeyCode, modifiers: KeyModifiers },
    KeyReleased { key: KeyCode, modifiers: KeyModifiers },
    MouseButtonPressed { button: MouseButton, x: f32, y: f32 },
    MouseButtonReleased { button: MouseButton, x: f32, y: f32 },
    MouseMoved { x: f32, y: f32, dx: f32, dy: f32 },
    MouseWheel { delta_x: f32, delta_y: f32 },
}

impl Event for InputEvent {}

/// Код клавиши
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum KeyCode {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Key0, Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9,
    Escape, Space, Enter, Tab, Backspace, Delete,
    Left, Right, Up, Down,
    LShift, RShift, LCtrl, RCtrl, LAlt, RAlt, LSuper, RSuper,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    Unknown,
}

/// Модификаторы клавиш
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}

/// Кнопка мыши
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}