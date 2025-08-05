use crossterm::{execute, terminal};
use minifb::{Window, WindowOptions};
use std::{
    io::stdout,
    time::{Duration, Instant},
};

use crate::utils::my_rect::Rect;

mod utils;

fn main() {
    println!("Hello, world!");

    let width = 640;
    let height = 640;
    let mut byte_index: usize = 0;
    let wh = width * height;
    // let qheight = (height / 4);
    let x = 0.0;
    let round_size = 50;
    let velocity = 1.0;
    let mut fps_timer = 0.0;
    let mut frame_count = 0;
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window = Window::new(
        "Simple Smooth moving Box",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    // window.set_target_fps(60);
    let mut last_time = Instant::now();
    let target_frame_time = Duration::from_secs_f64(1.0 / 60.0);

    // define box
    let mut box_1 = create_box(0x000000, 1, 0, Some(width), Some(height), Some(round_size));
    let mut box_2 = create_box(0x000000, -1, 0, Some(width), Some(height), Some(round_size));
    let mut box_3 = create_box(0x000000, 0, 1, Some(width), Some(height), Some(round_size));
    let mut box_4 = create_box(0x000000, 0, -1, Some(width), Some(height), Some(round_size));

    let mut box_5 = create_box(0x000000, -1, -1, Some(width), Some(height), Some(round_size));
    let mut box_6 = create_box(0x000000, 1, -1, Some(width), Some(height), Some(round_size));
    let mut box_7 = create_box(0x000000, -1, 1, Some(width), Some(height), Some(round_size));
    let mut box_8 = create_box(0x000000, 1, 1, Some(width), Some(height), Some(round_size));
    // end define box

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Calculate time since last frame
        let now = Instant::now();
        let delta = now - last_time;
        last_time = now;

        // draw bg
        for i in buffer.iter_mut() {
            if byte_index < (wh / 4) * 1 {
                *i = 0xff0000;
            } else if byte_index < (wh / 4) * 2 {
                *i = 0x00ff00;
            } else if byte_index < (wh / 4) * 3 {
                *i = 0x0000ff;
            } else {
                *i = 0xff00ff;
            }

            byte_index += 1;

            if byte_index >= width * height {
                byte_index = 0;
            }
        } // end for
        // end draw bg

        // draw boxx
        box_1.draw_rect(&mut buffer, &width);
        box_2.draw_rect(&mut buffer, &width);
        box_3.draw_rect(&mut buffer, &width);
        box_4.draw_rect(&mut buffer, &width);

        box_5.draw_rect(&mut buffer, &width);
        box_6.draw_rect(&mut buffer, &width);
        box_7.draw_rect(&mut buffer, &width);
        box_8.draw_rect(&mut buffer, &width);

        // moving moving
        if !window.is_key_down(minifb::Key::Space) {
            box_1.move_rect();
            box_2.move_rect();
            box_3.move_rect();
            box_4.move_rect();

            box_5.move_rect();
            box_6.move_rect();
            box_7.move_rect();
            box_8.move_rect();
        }

        // push frame
        window.update_with_buffer(&buffer, width, height).unwrap();

        // show fps dll
        frame_count += 1;
        fps_timer += delta.as_secs_f64();

        if fps_timer >= 1.0 {
            // clear cmd
            execute!(stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();

            println!(
                "FPS: {} \nX pos: {} \nVelocity: {} \nTest: {}",
                frame_count, x, velocity, box_1.x
            );
            frame_count = 0;
            fps_timer = 0.0;
        }

        // Optional: sleep to cap frame rate
        if delta < target_frame_time {
            std::thread::sleep(target_frame_time - delta);
        }
    }
} // end func

fn create_box(
    color: u32,
    velocity_x: isize,
    velocity_y: isize,
    width: Option<usize>,
    height: Option<usize>,
    round_size: Option<usize>,
) -> Rect {
    let width = width.unwrap_or(640);
    let height = height.unwrap_or(640);
    let round_size = round_size.unwrap_or(50);

    return Rect {
        x: width / 2 - (round_size / 2),
        y: height / 2 - (round_size / 2),
        w: round_size,
        h: round_size,
        width: width,
        height: height,
        velocity_x: velocity_x,
        velocity_y: velocity_y,
        color: color,
    };
} // end func
