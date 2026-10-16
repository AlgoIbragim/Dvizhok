//! # Time - Время и таймеры
//! 
//! Управление временем в движке - дельта-время, таймеры, интервалы.

use std::time::{Duration, Instant};

/// Время движка
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    /// Время старта приложения
    startup: Instant,
    /// Время первого кадра
    first_update: Option<Instant>,
    /// Время последнего кадра
    last_update: Option<Instant>,
    /// Дельта времени между кадрами
    delta: Duration,
    /// Масштаб времени
    scale: f32,
    /// Число кадров
    frame_count: u64,
    /// FPS на основе delta
    fps: f32,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    /// Создать новое время
    #[inline]
    pub fn new() -> Self {
        Self {
            startup: Instant::now(),
            first_update: None,
            last_update: None,
            delta: Duration::ZERO,
            scale: 1.0,
            frame_count: 0,
            fps: 0.0,
        }
    }

    /// Обновить время (вызывать каждый кадр)
    #[inline]
    pub fn update(&mut self) {
        let now = Instant::now();
        
        if self.first_update.is_none() {
            self.first_update = Some(now);
        }
        
        if let Some(last) = self.last_update {
            self.delta = now - last;
            self.fps = 1.0 / self.delta.as_secs_f32();
        }
        
        self.last_update = Some(now);
        self.frame_count += 1;
    }

    /// Получить дельту времени
    #[inline]
    pub fn delta(&self) -> Duration {
        self.delta
    }

    /// Получить дельту в секундах
    #[inline]
    pub fn delta_secs(&self) -> f32 {
        self.delta.as_secs_f32() * self.scale
    }

    /// Получить дельту в секундах без масштаба
    #[inline]
    pub fn raw_delta_secs(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Получить масштаб времени
    #[inline]
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Установить масштаб времени
    #[inline]
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    /// Увеличить масштаб времени
    #[inline]
    pub fn scale_by(&mut self, factor: f32) {
        self.scale *= factor;
    }

    /// Получить FPS
    #[inline]
    pub fn fps(&self) -> f32 {
        self.fps
    }

    /// Получить время с запуска
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Instant::now() - self.startup
    }

    /// Получить время с запуска в секундах
    #[inline]
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    /// Получить время с первого обновления
    #[inline]
    pub fn since_startup(&self) -> Duration {
        self.first_update
            .map(|t| Instant::now() - t)
            .unwrap_or(Duration::ZERO)
    }

    /// Получить время с первого обновления в секундах
    #[inline]
    pub fn since_startup_secs(&self) -> f32 {
        self.since_startup().as_secs_f32()
    }

    /// Получить номер кадра
    #[inline]
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Пауза (установить scale = 0)
    #[inline]
    pub fn pause(&mut self) {
        self.scale = 0.0;
    }

    /// Продолжить (восстановить scale = 1)
    #[inline]
    pub fn resume(&mut self) {
        self.scale = 1.0;
    }

    /// На паузе?
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.scale == 0.0
    }
}

/// Таймер для отсчета времени
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Timer {
    duration: Duration,
    elapsed: Duration,
    repeating: bool,
    finished: bool,
}

impl Timer {
    /// Создать таймер
    #[inline]
    pub fn new(duration: Duration, repeating: bool) -> Self {
        Self {
            duration,
            elapsed: Duration::ZERO,
            repeating,
            finished: false,
        }
    }

    /// Создать одноразовый таймер
    #[inline]
    pub fn once(duration: Duration) -> Self {
        Self::new(duration, false)
    }

    /// Создать повторяющийся таймер
    #[inline]
    pub fn repeating(duration: Duration) -> Self {
        Self::new(duration, true)
    }

    /// Из секунд (одноразовый)
    #[inline]
    pub fn from_secs(secs: f32) -> Self {
        Self::once(Duration::from_secs_f32(secs))
    }

    /// Из миллисекунд (одноразовый)
    #[inline]
    pub fn from_millis(millis: u64) -> Self {
        Self::once(Duration::from_millis(millis))
    }

    /// Тик таймера
    #[inline]
    pub fn tick(&mut self, delta: Duration) -> bool {
        if self.finished && !self.repeating {
            return false;
        }

        self.elapsed += delta;

        if self.elapsed >= self.duration {
            self.finished = true;
            if self.repeating {
                self.elapsed -= self.duration;
            } else {
                self.elapsed = self.duration;
            }
            true
        } else {
            false
        }
    }

    /// Завершен?
    #[inline]
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Повторяющийся?
    #[inline]
    pub fn repeating(&self) -> bool {
        self.repeating
    }

    /// Прогресс [0, 1]
    #[inline]
    pub fn fraction(&self) -> f32 {
        let t = self.elapsed.as_secs_f32() / self.duration.as_secs_f32();
        t.clamp(0.0, 1.0)
    }

    /// Оставшееся время
    #[inline]
    pub fn remaining(&self) -> Duration {
        self.duration.saturating_sub(self.elapsed)
    }

    /// Прошедшее время
    #[inline]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// Общая длительность
    #[inline]
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Сбросить
    #[inline]
    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.finished = false;
    }

    /// Пауза
    #[inline]
    pub fn pause(&mut self) {
        // Реализовать если нужно
    }

    /// Установить длительность
    #[inline]
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::from_secs(1.0)
    }
}

/// Интервал для периодических действий
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    timer: Timer,
}

impl Interval {
    /// Создать интервал
    #[inline]
    pub fn new(period: Duration) -> Self {
        Self {
            timer: Timer::repeating(period),
        }
    }

    /// Тик
    #[inline]
    pub fn tick(&mut self, delta: Duration) -> bool {
        self.timer.tick(delta)
    }

    /// Установить период
    #[inline]
    pub fn set_period(&mut self, period: Duration) {
        self.timer.set_duration(period);
    }

    /// Получить период
    #[inline]
    pub fn period(&self) -> Duration {
        self.timer.duration()
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::new(Duration::from_secs(1))
    }
}

/// Stopwatch для измерения времени
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stopwatch {
    elapsed: Duration,
    running: bool,
    last_start: Option<Instant>,
}

impl Stopwatch {
    /// Создать новый
    #[inline]
    pub fn new() -> Self {
        Self {
            elapsed: Duration::ZERO,
            running: false,
            last_start: None,
        }
    }

    /// Запустить
    #[inline]
    pub fn start(&mut self) {
        if !self.running {
            self.last_start = Some(Instant::now());
            self.running = true;
        }
    }

    /// Остановить
    #[inline]
    pub fn stop(&mut self) {
        if let Some(start) = self.last_start.take() {
            self.elapsed += Instant::now() - start;
            self.running = false;
        }
    }

    /// Сбросить
    #[inline]
    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        if self.running {
            self.last_start = Some(Instant::now());
        } else {
            self.last_start = None;
        }
    }

    /// Перезапустить
    #[inline]
    pub fn restart(&mut self) {
        self.reset();
        self.start();
    }

    /// Получить прошедшее время
    #[inline]
    pub fn elapsed(&self) -> Duration {
        let mut elapsed = self.elapsed;
        if let Some(start) = self.last_start {
            elapsed += Instant::now() - start;
        }
        elapsed
    }

    /// Получить в секундах
    #[inline]
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    /// Запущен?
    #[inline]
    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}