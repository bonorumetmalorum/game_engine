extern crate gfx;
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate conrod_winit;
extern crate conrod_gfx;
extern crate conrod_core;

use self::conrod_winit::winit;
use gfx::Device;

fn main() {
    let builder = glutin::WindowBuilder::new().with_title("Game Engine").with_dimensions((800, 800).into());

    let context = glutin::ContextBuilder::new().with_multisampling(4);

    let mut events_loop = winit::EventsLoop::new();

    let (window, mut device, mut factory, rvt, _) = gfx_window_glutin::init::<conrod_gfx::ColorFormat, gfx::format::DepthStencil>(builder, context, &events_loop).unwrap();

    let mut renderer = conrod_gfx::Renderer::new(&mut factory, &rvt, window.get_hidpi_factor()).unwrap();

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let mut ui = conrod_core::UiBuilder::new([800 as f64, 800 as f64]).build();

    let mut image_map = conrod_core::image::Map::new();

    'main: loop{
        let (win_1, win_h): (u32, u32) = match window.get_inner_size() {
            Some(s) => s.into(),
            None => break 'main
        };

        let dpi_factor = window.get_hidpi_factor() as f32;

        if let Some(primitives) = ui.draw_if_changed() {
            let dims = (win_1 as f32 * dpi_factor, win_h as f32 * dpi_factor);
            renderer.clear(&mut encoder, [0.2, 0.2, 0.2, 1.0]);
            renderer.fill(&mut encoder, dims, dpi_factor as f64, primitives, &image_map);
            renderer.draw(&mut factory, &mut encoder, &image_map);

            encoder.flush(&mut device);
            window.swap_buffers().unwrap();
            device.cleanup();
        }

        let mut should_quit = false;
        events_loop.poll_events(|event|{
            if let Some(event) = conrod_winit::convert_event(event.clone(), window.window()){
                ui.handle_event(event);
            }

            match event {
                winit::Event::WindowEvent{event, ..} =>
                    match event{
                        winit::WindowEvent::KeyboardInput{input: winit::KeyboardInput{virtual_keycode: Some(winit::VirtualKeyCode::Escape),..},..}| winit::WindowEvent::CloseRequested => should_quit = true,
                        winit::WindowEvent::Resized(logical_size) => {
                            let hidpi_factor = window.get_hidpi_factor();
                            let physical_size = logical_size.to_physical(hidpi_factor);
                            window.resize(physical_size);
                            let (new_color, _) = gfx_window_glutin::new_views::<conrod_gfx::ColorFormat, gfx::format::DepthStencil>(&window);
                            renderer.on_resize(new_color);
                        }
                        _ => {},
                    }
                _ => {},
            }
        });
        if should_quit {
            break 'main;
        }
    }

}
