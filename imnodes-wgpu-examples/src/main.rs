use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowAttributes,
};

mod color_editor;
mod hello_world;
mod multi_editor;
mod save_load;

fn main() {
    // Set up window and GPU
    let event_loop = EventLoop::new().unwrap();
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

    let window = event_loop
        .create_window(
            WindowAttributes::new()
                .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 720.0))
                .with_title("imnodes-wgpu"),
        )
        .unwrap();

    let size = window.inner_size();
    // Safety: create_surface is safe according to wgpu docs if window handle is valid.
    let surface = instance.create_surface(&window).unwrap(); // Removed unsafe block

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

    // Set up swap chain/surface config
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps
        .formats
        .iter()
        .copied()
        .find(|f| *f == wgpu::TextureFormat::Bgra8UnormSrgb)
        .unwrap_or(surface_caps.formats[0]);

    let mut surface_config = wgpu::SurfaceConfiguration {
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

    let mut imgui = imgui::Context::create();
    let imnodes_ui = imnodes::Context::new();

    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );
    imgui.set_ini_filename(None);

    let mut hidpi_factor = window.scale_factor();

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui
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

    let mut renderer = imgui_wgpu::Renderer::new(&mut imgui, &device, &queue, renderer_config);

    let mut last_frame = Instant::now();
    let mut last_cursor = None;

    // Create editor states
    let mut first_editor = imnodes_ui.create_editor();
    let mut second_editor_state_1 = multi_editor::MultiEditState::new(&imnodes_ui);
    let mut second_editor_state_2 = multi_editor::MultiEditState::new(&imnodes_ui);
    let mut color_editor = color_editor::State::new(&imnodes_ui);
    let mut save_load_editor = save_load::SaveLoadState::new(&imnodes_ui);

    event_loop
        .run(|event, elwt| {
            // Pass the outer event reference to handle_event
            platform.handle_event(imgui.io_mut(), &window, &event);

            match event {
                Event::WindowEvent {
                    event: window_event,
                    window_id,
                } if window_id == window.id() => {
                    match window_event {
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            hidpi_factor = scale_factor;
                        }
                        WindowEvent::Resized(size) => {
                            if size.width > 0 && size.height > 0 {
                                surface_config.width = size.width;
                                surface_config.height = size.height;
                                surface.configure(&device, &surface_config);
                            }
                        }
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::RedrawRequested => {
                            let now = Instant::now();
                            imgui.io_mut().update_delta_time(now - last_frame);
                            last_frame = now;

                            let frame = match surface.get_current_texture() {
                                Ok(frame) => frame,
                                Err(wgpu::SurfaceError::Outdated) => {
                                    surface.configure(&device, &surface_config);
                                    surface.get_current_texture().expect(
                                        "Failed to acquire next surface texture after outdated!",
                                    )
                                }
                                Err(e) => {
                                    eprintln!("Dropped frame: {e:?}");
                                    window.request_redraw(); // Request redraw on error
                                    return;
                                }
                            };
                            let view = frame
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default());

                            platform
                                .prepare_frame(imgui.io_mut(), &window)
                                .expect("Failed to prepare frame");
                            let ui = imgui.frame();

                            {
                                let window_ref = ui
                                    .window("Hello imnodes")
                                    .resizable(false)
                                    .position([0.0, 0.0], imgui::Condition::Always)
                                    .size(
                                        [
                                            surface_config.width as f32 / hidpi_factor as f32,
                                            surface_config.height as f32 / hidpi_factor as f32,
                                        ],
                                        imgui::Condition::Always,
                                    );

                                window_ref.build(|| {
                                    ui.text("Hello from imnodes-rs!");

                                    if imgui::CollapsingHeader::new("hello world").build(ui) {
                                        ui.child_window("hello_world_child")
                                            .size([0.0, 200.0])
                                            .build(|| {
                                                let _ = first_editor.set_as_current_editor();
                                                hello_world::show(ui, &mut first_editor);
                                            });
                                    }

                                    if imgui::CollapsingHeader::new("Save/Load Example").build(ui) {
                                        ui.child_window("save_load_child").size([0.0, 0.0]).build(
                                            || {
                                                let _ = save_load_editor
                                                    .editor_context
                                                    .set_as_current_editor();
                                                save_load::show(ui, &mut save_load_editor);
                                            },
                                        );
                                    }

                                    if imgui::CollapsingHeader::new("multi editor").build(ui) {
                                        // Wrap ui.style() call in unsafe block
                                        let item_spacing = unsafe { ui.style().item_spacing[0] };
                                        let half_width = (ui.window_content_region_max()[0]
                                            - ui.window_content_region_min()[0])
                                            * 0.5
                                            - item_spacing * 0.5;

                                        ui.child_window("multi_editor_child_1")
                                            .size([half_width, 300.0])
                                            .build(|| {
                                                let _ = second_editor_state_1
                                                    .editor_context
                                                    .set_as_current_editor();
                                                multi_editor::show(ui, &mut second_editor_state_1);
                                            });

                                        ui.same_line();

                                        ui.child_window("multi_editor_child_2")
                                            .size([half_width, 300.0])
                                            .build(|| {
                                                let _ = second_editor_state_2
                                                    .editor_context
                                                    .set_as_current_editor();
                                                multi_editor::show(ui, &mut second_editor_state_2);
                                            });
                                    }

                                    if imgui::CollapsingHeader::new("color editor").build(ui) {
                                        ui.child_window("color_editor_child")
                                            .size([0.0, 0.0])
                                            .build(|| {
                                                let _ = color_editor
                                                    .editor_context
                                                    .set_as_current_editor();
                                                color_editor::show(ui, &mut color_editor);
                                            });
                                    }
                                });
                            }

                            let mut encoder: wgpu::CommandEncoder =
                                device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                    label: None,
                                });

                            if last_cursor != Some(ui.mouse_cursor()) {
                                last_cursor = Some(ui.mouse_cursor());
                                platform.prepare_render(ui, &window);
                            }

                            let mut rpass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });

                            renderer
                                .render(imgui.render(), &queue, &device, &mut rpass)
                                .expect("Rendering failed");

                            drop(rpass);

                            queue.submit(Some(encoder.finish()));
                            frame.present();
                        }
                        _ => (),
                    }
                }
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            }

            if !(imgui.io().want_capture_mouse || imgui.io().want_capture_keyboard) {
                // Handle application-specific input events here if needed
            }
        })
        .unwrap();
}
