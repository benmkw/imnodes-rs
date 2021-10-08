use futures::executor::block_on;
use imgui::*;
use imgui_wgpu::{Renderer, RendererConfig};
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

// the actual imnodes samples are in there
mod color_editor;
mod hello_world;
mod multi_editor;

fn main() {
    // Set up window and GPU
    let event_loop = EventLoop::new();
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

    let (window, size, surface) = {
        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 1280.0,
            height: 720.0,
        });
        window.set_title(&"imnodes-wgpu".to_string());
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        (window, size, surface)
    };

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        ..Default::default()
    }))
    .unwrap();

    let (device, queue) =
        block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None)).unwrap();

    // Set up swap chain
    let mut surface_desc = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
    };

    surface.configure(&device, &surface_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    // Set up dear imnodes
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

    imgui.fonts().add_font(&[FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 1,
            pixel_snap_h: true,
            size_pixels: font_size,
            ..Default::default()
        }),
    }]);

    //
    // Set up dear imgui wgpu renderer
    //
    let renderer_config = RendererConfig {
        texture_format: surface_desc.format,
        ..Default::default()
    };

    let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

    let mut last_frame = Instant::now();
    let mut last_cursor = None;

    let mut first_editor = imnodes_ui.create_editor();
    let mut second_editor_state_1 = multi_editor::MultiEditState::new(&imnodes_ui);
    let mut second_editor_state_2 = multi_editor::MultiEditState::new(&imnodes_ui);
    let mut color_editor = color_editor::State::new(&imnodes_ui);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::ScaleFactorChanged { scale_factor, .. },
                ..
            } => {
                hidpi_factor = scale_factor;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Recreate the swap chain with the new size
                surface_desc.width = size.width;
                surface_desc.height = size.height;
                surface.configure(&device, &surface_desc);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawEventsCleared => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;

                let frame = match surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        eprintln!("dropped frame: {:?}", e);
                        return;
                    }
                };

                platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();

                {
                    let window = imgui::Window::new("Hello imnodes")
                        .resizable(false)
                        .position([0.0, 0.0], Condition::Always)
                        .size(
                            [
                                surface_desc.width as f32 / hidpi_factor as f32,
                                surface_desc.height as f32 / hidpi_factor as f32,
                            ],
                            Condition::Always,
                        );

                    window.build(&ui, || {
                        ui.text("Hello from imnodes-rs!");

                        if CollapsingHeader::new("hello world").build(&ui) {
                            ChildWindow::new("1").size([0.0, 0.0]).build(&ui, || {
                                hello_world::show(&ui, &mut first_editor);
                            });
                        }

                        if CollapsingHeader::new("multi editor").build(&ui) {
                            let width = ui.window_content_region_width() / 2_f32;
                            ChildWindow::new("2").size([width, 0.0]).build(&ui, || {
                                multi_editor::show(&ui, &mut second_editor_state_1);
                            });

                            ui.same_line();

                            ChildWindow::new("3").size([width, 0.0]).build(&ui, || {
                                multi_editor::show(&ui, &mut second_editor_state_2);
                            });
                        }

                        if CollapsingHeader::new("color editor").build(&ui) {
                            ChildWindow::new("1").size([0.0, 0.0]).build(&ui, || {
                                color_editor::show(&ui, &mut color_editor);
                            });
                        }
                    });
                }

                let mut encoder: wgpu::CommandEncoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

                if last_cursor != Some(ui.mouse_cursor()) {
                    last_cursor = Some(ui.mouse_cursor());
                    platform.prepare_render(&ui, &window);
                }

                let view = &frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.4,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });

                renderer
                    .render(ui.render(), &queue, &device, &mut rpass)
                    .expect("Rendering failed");

                drop(rpass); // renders to screen on drop, will probaly be changed in wgpu 0.7 or later

                queue.submit(Some(encoder.finish()));
                frame.present()
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}
