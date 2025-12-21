use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

mod color_editor;
mod hello_world;
mod multi_editor;
mod save_load;

struct RenderState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    platform: imgui_winit_support::WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    surface_config: wgpu::SurfaceConfiguration,
    last_frame: Instant,
    last_cursor: Option<imgui::MouseCursor>,
}

struct ImNodesApp<'a> {
    imgui: imgui::Context,
    _imnodes: &'a imnodes::Context,

    // Logic states
    first_editor: imnodes::EditorContext<'a>,
    second_editor_state_1: multi_editor::MultiEditState<'a>,
    second_editor_state_2: multi_editor::MultiEditState<'a>,
    color_editor: color_editor::State<'a>,
    save_load_editor: save_load::SaveLoadState<'a>,

    // Render state (initialized in resumed)
    render_state: Option<RenderState>,
}

impl<'a> ApplicationHandler for ImNodesApp<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    winit::window::Window::default_attributes()
                        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
                        .with_title("imnodes-wgpu"),
                )
                .unwrap(),
        );

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }))
            .unwrap();

        let (device, queue) =
            futures::executor::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()))
                .unwrap();

        let size = window.inner_size();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| *f == wgpu::TextureFormat::Bgra8UnormSrgb)
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,

            format: surface_format,

            width: size.width,

            height: size.height,

            present_mode: surface_caps.present_modes[0],

            alpha_mode: surface_caps.alpha_modes[0],

            view_formats: vec![],

            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let mut platform = imgui_winit_support::WinitPlatform::new(&mut self.imgui);

        platform.attach_window(
            self.imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );

        let hidpi_factor = window.scale_factor();

        let font_size = (13.0 * hidpi_factor) as f32;

        self.imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        self.imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData {
                config: Some(imgui::FontConfig {
                    oversample_h: 1,

                    pixel_snap_h: true,

                    size_pixels: font_size,

                    ..Default::default()
                }),
            }]);

        let renderer_config = imgui_wgpu::RendererConfig {
            texture_format: surface_config.format,

            ..Default::default()
        };

        let renderer = imgui_wgpu::Renderer::new(&mut self.imgui, &device, &queue, renderer_config);

        self.render_state = Some(RenderState {
            window,

            surface,

            device,

            queue,

            platform,

            renderer,

            surface_config,

            last_frame: Instant::now(),

            last_cursor: None,
        });
    }

    fn window_event(
        &mut self,

        event_loop: &ActiveEventLoop,

        _window_id: WindowId,

        event: WindowEvent,
    ) {
        let render_state = match &mut self.render_state {
            Some(s) => s,

            None => return,
        };

        // Construct Event::WindowEvent for platform.handle_event

        // Note: This relies on winit 0.30 Event enum structure which is still available

        let winit_event: winit::event::Event<()> = winit::event::Event::WindowEvent {
            window_id: render_state.window.id(),

            event: event.clone(),
        };

        render_state
            .platform
            .handle_event(self.imgui.io_mut(), &render_state.window, &winit_event);

        match event {
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    render_state.surface_config.width = size.width;

                    render_state.surface_config.height = size.height;

                    render_state
                        .surface
                        .configure(&render_state.device, &render_state.surface_config);
                }
            }

            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                let now = Instant::now();

                self.imgui
                    .io_mut()
                    .update_delta_time(now - render_state.last_frame);

                render_state.last_frame = now;

                let frame = match render_state.surface.get_current_texture() {
                    Ok(frame) => frame,

                    Err(wgpu::SurfaceError::Outdated) => {
                        render_state
                            .surface
                            .configure(&render_state.device, &render_state.surface_config);

                        render_state
                            .surface
                            .get_current_texture()
                            .expect("Failed to acquire next surface texture!")
                    }

                    Err(e) => {
                        eprintln!("Dropped frame: {e:?}");

                        return;
                    }
                };

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                render_state
                    .platform
                    .prepare_frame(self.imgui.io_mut(), &render_state.window)
                    .expect("Failed to prepare frame");

                let ui = self.imgui.frame();

                {
                    let window_ref = ui
                        .window("Hello imnodes")
                        .resizable(false)
                        .position([0.0, 0.0], imgui::Condition::Always)
                        .size(
                            [
                                render_state.surface_config.width as f32
                                    / render_state.window.scale_factor() as f32,
                                render_state.surface_config.height as f32
                                    / render_state.window.scale_factor() as f32,
                            ],
                            imgui::Condition::Always,
                        );

                    window_ref.build(|| {
                        ui.text("Hello from imnodes-rs!");

                        if imgui::CollapsingHeader::new("hello world").build(ui) {
                            ui.child_window("hello_world_child")
                                .size([0.0, 200.0])
                                .build(|| {
                                    hello_world::show(ui, &mut self.first_editor);
                                });
                        }

                        if imgui::CollapsingHeader::new("Save/Load Example").build(ui) {
                            ui.child_window("save_load_child")
                                .size([0.0, 0.0])
                                .build(|| {
                                    save_load::show(ui, &mut self.save_load_editor);
                                });
                        }

                        if imgui::CollapsingHeader::new("multi editor").build(ui) {
                            let item_spacing = unsafe { ui.style().item_spacing[0] };

                            let half_width = (ui.window_content_region_max()[0]
                                - ui.window_content_region_min()[0])
                                * 0.5
                                - item_spacing * 0.5;

                            ui.child_window("multi_editor_child_1")
                                .size([half_width, 300.0])
                                .build(|| {
                                    multi_editor::show(ui, &mut self.second_editor_state_1);
                                });

                            ui.same_line();

                            ui.child_window("multi_editor_child_2")
                                .size([half_width, 300.0])
                                .build(|| {
                                    multi_editor::show(ui, &mut self.second_editor_state_2);
                                });
                        }

                        if imgui::CollapsingHeader::new("color editor").build(ui) {
                            ui.child_window("color_editor_child")
                                .size([0.0, 0.0])
                                .build(|| {
                                    color_editor::show(ui, &mut self.color_editor);
                                });
                        }
                    });
                }

                let mut encoder = render_state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if render_state.last_cursor != ui.mouse_cursor() {
                    render_state.last_cursor = ui.mouse_cursor();
                    render_state
                        .platform
                        .prepare_render(ui, &render_state.window);
                }

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("ImGui Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.4,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_state
                    .renderer
                    .render(
                        self.imgui.render(),
                        &render_state.queue,
                        &render_state.device,
                        &mut rpass,
                    )
                    .expect("Rendering failed");

                drop(rpass);

                render_state.queue.submit(Some(encoder.finish()));
                frame.present();

                render_state.window.request_redraw();
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(state) = &self.render_state {
            state.window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut imgui = imgui::Context::create();
    let imnodes = imnodes::Context::new(&imnodes::ContextDesc::default());

    imgui.set_ini_filename(None);

    let first_editor = imnodes.create_editor(&imnodes::EditorContextDesc::default());
    let second_editor_state_1 = multi_editor::MultiEditState::new(&imnodes);
    let second_editor_state_2 = multi_editor::MultiEditState::new(&imnodes);
    let color_editor = color_editor::State::new(&imnodes);
    let save_load_editor = save_load::SaveLoadState::new(&imnodes);

    let mut app = ImNodesApp {
        imgui,
        _imnodes: &imnodes,
        first_editor,
        second_editor_state_1,
        second_editor_state_2,
        color_editor,
        save_load_editor,
        render_state: None,
    };

    event_loop.run_app(&mut app).unwrap();
}
