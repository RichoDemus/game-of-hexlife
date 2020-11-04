mod core;
mod draw;


use quicksilver::blinds::event::MouseButton::Left;
use quicksilver::graphics::VectorFont;
use quicksilver::input::{Event, Key, ScrollDelta};
use quicksilver::{
    geom::Vector, graphics::Color, graphics::Image, run, Graphics, Input, Result, Settings, Timer, Window,
};
use quicksilver::geom::Rectangle;
use crate::core::Core;


// use 144 fps for non wasm release, use 60 fps for wasm or debug
#[cfg(any(target_arch = "wasm32", debug_assertions))]
pub const FPS: f32 = 60.0;
#[cfg(all(not(target_arch = "wasm32"), not(debug_assertions)))]
pub const FPS: f32 = 144.0;
pub const UPS: f32 = 200.;

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;

fn main() {
    run(
        Settings {
            title: "Game of Hexlife",
            size: Vector {
                x: WIDTH,
                y: HEIGHT,
            },
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    let mut frames: u32 = 0;
    let mut last_fps: u32 = 0;
    let dt = 1. / (UPS as f64);

    let mut core = Core::default();

    // Here we make 2 kinds of timers.
    // One to provide an consistant update time, so our example updates 30 times per second
    // the other informs us when to draw the next frame, this causes our example to draw 60 times per second
    let mut update_timer = Timer::time_per_second(UPS);
    let mut draw_timer = Timer::time_per_second(FPS);
    let mut fps_timer = Timer::time_per_second(1.);
    let mut day_tick_timer = Timer::time_per_second(1.);

    let image = Image::from_encoded_bytes(&gfx, include_bytes!("hexagon.png"))?;

    let ttf = VectorFont::from_slice(include_bytes!("BebasNeue-Regular.ttf"));
    let mut font = ttf.to_renderer(&gfx, 20.0)?;

    let mut running = true;
    let mut speed_up = false;
    let mut camera_y_axis;
    let mut camera_x_axis;
    let mut zoom_scale = 1.;
    while running {
        camera_y_axis = 0.;
        camera_x_axis = 0.;
        while let Some(event) = input.next_event().await {
            if let Event::PointerInput(pointer_input_event) = event {
                if !pointer_input_event.is_down() && pointer_input_event.button() == Left {
                    let mouse_position = input.mouse().location();

                    // core.click(convert(mouse_position));
                }
            } else if let Event::KeyboardInput(keyboard_event) = event {
                if keyboard_event.is_down() && keyboard_event.key() == Key::Space {
                    // core.pause();
                } else if keyboard_event.is_down() && keyboard_event.key() == Key::Escape {
                    running = false;
                } else if keyboard_event.is_down() && keyboard_event.key() == Key::LShift {
                    speed_up = true;
                } else {
                    speed_up = false;
                }
            } else if let Event::ScrollInput(delta) = event {
                if let ScrollDelta::Lines(lines) = delta {
                    zoom_scale += lines.y * 0.1;
                }
            }
        }
        if input.key_down(Key::W) {
            camera_y_axis = 1.;
        }
        if input.key_down(Key::S) {
            camera_y_axis = -1.;
        }
        if input.key_down(Key::D) {
            camera_x_axis = -1.;
        }
        if input.key_down(Key::A) {
            camera_x_axis = 1.;
        }

        // We use a while loop rather than an if so that we can try to catch up in the event of having a slow down.
        while update_timer.tick() {
            // core.tick();
            // core.tick(dt, camera_x_axis, camera_y_axis);
            if speed_up {
                for _ in 1..10 {
                    // core.tick(dt, camera_x_axis, camera_y_axis);
                }
            }
        }

        while day_tick_timer.tick() {
            // core.tick_day();
            core.tick();
            if speed_up {
                for _ in 1..10 {
                    // core.tick_day();
                }
            }
        }

        // Unlike the update cycle drawing doesn't change our state
        // Because of this there is no point in trying to catch up if we are ever 2 frames late
        // Instead it is better to drop/skip the lost frames
        if draw_timer.exhaust().is_some() {
            gfx.clear(Color::BLACK);

            // draw::draw(&mut gfx, zoom_scale, &mut font, &core);

            draw::draw(&mut gfx, &core, &image);

            frames += 1;
            if fps_timer.tick() {
                last_fps = frames;
                frames = 0;
            }
            font.draw(
                &mut gfx,
                format!("FPS: {}", last_fps).as_str(),
                Color::GREEN,
                Vector::new(10.0, 30.0),
            )?;

            gfx.present(&window)?;
        }
    }
    Ok(())
}
