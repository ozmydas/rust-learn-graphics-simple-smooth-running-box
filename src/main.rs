use crossterm::{execute, terminal};
use minifb::{Window, WindowOptions};
use std::{io::stdout, time::{Duration, Instant}};

use crate::utils::my_rect::Rect;

mod utils;

fn main() {
    println!("Hello, world!");

    let width = 640;
    let height = 480;
    let mut byte_index: usize = 0;
    let wh = width * height;
    let qheight = (height / 4);
    let x = 0.0;
    let round_size = 50;
    let velocity = 1.0;
    let mut fps_timer = 0.0;
    let mut frame_count = 0;
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window = Window::new(
        "Simpe Smooth Running Box",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    // window.set_target_fps(60);
    let mut last_time = Instant::now();
    let target_frame_time = Duration::from_secs_f64(1.0 / 60.0);

    // define box
    let mut box_1 = Rect {
        x: x as usize,
        y: (qheight * 1) - (qheight - ((qheight - round_size) / 2)),
        w: round_size,
        h: round_size,
        velocity: velocity,
        color: 0x000000,
    };

    let mut box_2 = Rect {
        x: width - round_size as usize,
        y: (qheight * 2) - (qheight - ((qheight - round_size) / 2)),
        w: round_size,
        h: round_size,
        velocity: velocity * -1.0,
        color: 0x000000,
    };

    let mut box_3 = Rect {
        x: x as usize,
        y: (qheight * 3) - (qheight - ((qheight - round_size) / 2)),
        w: round_size,
        h: round_size,
        velocity: velocity,
        color: 0x000000,
    };

    let mut box_4 = Rect {
        x: width - round_size as usize,
        y: (qheight * 4) - (qheight - ((qheight - round_size) / 2)),
        w: round_size,
        h: round_size,
        velocity: velocity* -1.0,
        color: 0x000000,
    };
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

        // draw box 1
        box_1.draw_rect(&mut buffer, &width);
        
        // draw box 2
        box_2.draw_rect(&mut buffer, &width);

        // draw box 3
        box_3.draw_rect(&mut buffer, &width);
        
        // draw box 4
        box_4.draw_rect(&mut buffer, &width);

        // moving moving
        if !window.is_key_down(minifb::Key::Space) {
            box_1.move_rect(&width, &round_size);
            box_2.move_rect(&width, &round_size);
            box_3.move_rect(&width, &round_size);
            box_4.move_rect(&width, &round_size);
        }

        // push frame
        window.update_with_buffer(&buffer, width, height).unwrap();

        // show fps dll
        frame_count += 1;
        fps_timer += delta.as_secs_f64();

        if fps_timer >= 1.0 {
            // clear cmd
            execute!(stdout(),terminal::Clear(terminal::ClearType::All)).unwrap();

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