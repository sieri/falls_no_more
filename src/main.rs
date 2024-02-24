use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::world::World;

mod world;

const SCALE: u32 = 3;

struct MouseState {
    pub x: usize,
    pub y: usize,
    pub clicked: bool,
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let size = LogicalSize::new(1000f64, 500f64);

    let window = WindowBuilder::new()
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_resizable(false)
        .with_title("Falls No More!")
        .build(&event_loop)
        .unwrap();

    let win_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(win_size.width, win_size.height, &window);

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    let width = size.width as u32 / SCALE;
    let height = size.height as u32 / SCALE;

    let mut pixels = Pixels::new(width, height, surface_texture).unwrap();

    let mut world = World::new(width, height);
    world.show_all(&mut pixels);
    let mut mouse_state = MouseState {
        x: 0,
        y: 0,
        clicked: false,
    };

    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                }
                Event::AboutToWait => {
                    // Application update code.
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    if mouse_state.clicked {
                        world.clicked(mouse_state.x, mouse_state.y);
                    }
                    world.fall();
                    // Redraw the application.
                    world.show(&mut pixels);

                    if let Err(err) = pixels.render() {
                        println!("pixels.render {:?}", err);
                        elwt.exit();
                    }
                }
                Event::WindowEvent {
                    event:
                    WindowEvent::MouseInput {
                        state: pressed,
                        button: MouseButton::Left,
                        ..
                    },
                    ..
                } => {
                    match pressed {
                        ElementState::Pressed => {
                            // if new click
                            if !mouse_state.clicked {}

                            mouse_state.clicked = true
                        }
                        ElementState::Released => mouse_state.clicked = false,
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position: pos, .. },
                    ..
                } => {
                    let cursor_position: (f32, f32) = pos.into();
                    if let Ok((x, y)) = pixels.window_pos_to_pixel(cursor_position) {
                        mouse_state.x = x;
                        mouse_state.y = y;
                    }
                }
                _ => (),
            }
        })
        .expect("TODO: panic message");
}
