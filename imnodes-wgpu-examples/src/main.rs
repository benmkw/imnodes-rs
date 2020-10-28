use futures::executor::block_on;
use imgui::*;
use imgui_wgpu::RendererConfig;
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
    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

    let (window, size, surface) = {
        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize {
            width: 1280.0,
            height: 720.0,
        });
        window.set_title(&"implot-wgpu".to_string());
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };

        (window, size, surface)
    };

    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
    }))
    .unwrap();

    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            shader_validation: false,
        },
        None,
    ))
    .unwrap();

    // Set up swap chain
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();

    // Set up dear implot
    // let implot = implot::Context::create();

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
    let mut renderer = RendererConfig::new()
        .set_texture_format(sc_desc.format)
        .build(&mut imgui, &device, &queue);

    let mut last_frame = Instant::now();
    let mut last_cursor = None;

    let mut make_fullscreen = true;

    let first_editor = imnodes_ui.create_editor();
    let mut second_editor_state_1 = multi_editor::MultiEditState::new(&imnodes_ui);
    let mut second_editor_state_2 = multi_editor::MultiEditState::new(&imnodes_ui);
    let mut color_editor = color_editor::State::new(&imnodes_ui);

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // let plot_ui = implot.get_plot_ui();
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
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
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

                let frame = match swap_chain.get_current_frame() {
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
                    let window = imgui::Window::new(im_str!("Hello implot"));
                    let window = if make_fullscreen {
                        let border = 10.0;
                        window.position([0.0, 0.0], Condition::Always).size(
                            [
                                sc_desc.width as f32 / hidpi_factor as f32 - border,
                                sc_desc.height as f32 / hidpi_factor as f32 - border,
                            ],
                            Condition::Always,
                        )
                    } else {
                        window.size([400.0, 300.0], Condition::FirstUseEver)
                    };

                    window.build(&ui, || {
                        ui.text(im_str!("Hello from imnodes-rs!"));

                        ui.checkbox(
                            im_str!("make the imnodes window fill the whole outer window"),
                            &mut make_fullscreen,
                        );

                        if CollapsingHeader::new(im_str!("hello world")).build(&ui) {
                            ChildWindow::new(im_str!("1"))
                                .size([0.0, 0.0])
                                .build(&ui, || {
                                    hello_world::show(&ui, &first_editor);
                                });
                        }

                        if CollapsingHeader::new(im_str!("multi editor")).build(&ui) {
                            let width = ui.window_content_region_width() / 2 as f32;
                            ChildWindow::new(im_str!("2"))
                                .size([width, 0.0])
                                .build(&ui, || {
                                    multi_editor::show(&ui, &mut second_editor_state_1);
                                });

                            ui.same_line(0.0);

                            ChildWindow::new(im_str!("3"))
                                .size([width, 0.0])
                                .build(&ui, || {
                                    multi_editor::show(&ui, &mut second_editor_state_2);
                                });
                        }

                        if CollapsingHeader::new(im_str!("color editor")).build(&ui) {
                            ChildWindow::new(im_str!("1"))
                                .size([0.0, 0.0])
                                .build(&ui, || {
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

                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
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
            }
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}
