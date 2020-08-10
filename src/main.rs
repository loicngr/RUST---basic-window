extern crate image;

use std::path::Path;

use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder}
};

const WINDOW_CURRENT_WIDTH: u32 = 800;
const WINDOW_CURRENT_HEIGHT: u32 = 600;

const WINDOW_MIN_WIDTH: u32 = 500;
const WINDOW_MIN_HEIGHT: u32 = 500;

const WINDOW_TITLE: &str = "Test de fenêtre !";

// Chargement d'un icône
fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}

// Fonction principal
fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png");

    let window_min_size = winit::dpi::PhysicalSize::new(WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT);
    let window_size = winit::dpi::PhysicalSize::new(WINDOW_CURRENT_WIDTH, WINDOW_CURRENT_HEIGHT);

    let window_icon = load_icon(Path::new(path));

    let events_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(window_size)
        .with_min_inner_size(window_min_size)
        .with_window_icon(Some(window_icon))
        .build(&events_loop)
        .unwrap();

    events_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::MouseInput {
                    state: ElementState::Released,
                    ..
                } => {
                    window.request_redraw()
                },
                _ => {},
            },
            Event::RedrawRequested(_) => {
                println!("\nredrawing\n");
            },
            _ => (),
        }
    });
}