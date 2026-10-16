//! # Render Context - Контекст рендеринга
//! 
//! Инициализация wgpu и управление устройством/очередью.

use wgpu::{Instance, Adapter, Device, Queue, Surface, SurfaceConfiguration, PowerPreference};
use winit::window::Window;

/// Контекст рендеринга
pub struct RenderContext {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub surface: Option<Surface<'static>>,
    pub config: Option<SurfaceConfiguration>,
    pub size: (u32, u32),
}

impl RenderContext {
    /// Создать новый контекст
    pub async fn new(window: Option<&Window>) -> Self {
        let instance = Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = window.map(|w| unsafe {
            std::mem::transmute::<Surface<'_>, Surface<'static>>(
                instance.create_surface(w).unwrap()
            )
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Render Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: adapter.limits(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let config = surface.as_ref().map(|surface| {
            let caps = surface.get_capabilities(&adapter);
            let format = caps.formats.iter()
                .find(|f| f.is_srgb())
                .copied()
                .unwrap_or(caps.formats[0]);

            SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: window.map(|w| w.inner_size().width).unwrap_or(800),
                height: window.map(|w| w.inner_size().height).unwrap_or(600),
                present_mode: wgpu::PresentMode::AutoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            }
        });

        let size = if let Some(ref cfg) = config {
            (cfg.width, cfg.height)
        } else {
            (800, 600)
        };

        Self {
            instance,
            adapter,
            device,
            queue,
            surface,
            config,
            size,
        }
    }

    /// Изменить размер
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.size = (width, height);

        if let Some(ref mut config) = self.config {
            config.width = width;
            config.height = height;

            if let Some(ref surface) = self.surface {
                surface.configure(&self.device, config);
            }
        }
    }

    /// Получить текущий surface texture
    pub fn get_current_texture(&self) -> Option<wgpu::SurfaceTexture> {
        self.surface.as_ref()?.get_current_texture().ok()
    }
}