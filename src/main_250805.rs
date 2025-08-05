use clearscreen::{ClearScreen, clear};
use minifb::{Window, WindowOptions};
use std::time::{Duration, Instant};

fn main() {
    println!("Hello, world!");

    let width = 640;
    let height = 480;
    let mut pin: usize = 0;
    let wh = width * height;
    let qheight = (height / 4);
    let mut x = 0.0;
    let mut xx = width as f64;
    let round_size = 50;
    let speed = 160.0;
    let mut fps_timer = 0.0;
    let mut frame_count = 0;
    let mut velocity = 1.0;
    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window = Window::new(
        "Simpe Smooth moving Box",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    // window.set_target_fps(60);
    let mut last_time = Instant::now();
    let target_frame_time = Duration::from_secs_f64(1.0 / 60.0);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Calculate time since last frame
        let now = Instant::now();
        let delta = now - last_time;
        last_time = now;

        let seconds = delta.as_secs_f64();

        // draw bg
        for i in buffer.iter_mut() {
            if pin < (wh / 4) * 1 {
                *i = 0xff0000;
            } else if pin < (wh / 4) * 2 {
                *i = 0x00ff00;
            } else if pin < (wh / 4) * 3 {
                *i = 0x0000ff;
            } else {
                *i = 0xff00ff;
            }

            pin += 1;

            if pin >= width * height {
                pin = 0;
            }
        } // end for
        // end draw bg

        // draw box

        draw_rect(
            &mut buffer,
            width,
            x as usize,
            (qheight * 1) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );
        draw_rect_flipped_x(
            &mut buffer,
            width,
            xx as usize,
            (qheight * 2) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );
        draw_rect(
            &mut buffer,
            width,
            x as usize,
            (qheight * 3) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );
        draw_rect_flipped_x(
            &mut buffer,
            width,
            xx as usize,
            (qheight * 4) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );

        // moving moving
        if !window.is_key_down(minifb::Key::Space) {
            // normal move
            // x = (x + seconds) % width as f64;

            // anti thesis normal move
            if xx > 0.0 {
                xx -= velocity;
            } else {
                xx = width as f64 - velocity;
            }

            // pingpong move
            if x >= (width - round_size) as f64 {
                x -= 1.0;
                velocity = -velocity;
            } else if x <= 0.0 {
                x += 1.0;
                velocity = velocity * -1.0;
            } else {
                x += velocity;
            }
        }

        // push frame
        window.update_with_buffer(&buffer, width, height).unwrap();

        frame_count += 1;
        fps_timer += delta.as_secs_f64();

        if fps_timer >= 1.0 {
            clear().unwrap();
            println!(
                "FPS: {} \nX pos: {} \nVelocity: {}",
                frame_count, x, velocity
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

fn draw_rect(buffer: &mut [u32], width: usize, x: usize, y: usize, w: usize, h: usize, color: u32) {
    for iy in y..y + h {
        for ix in x..x + w {
            let mut ixx = ix;

            if ixx > width {
                ixx = ixx - width;
            }

            if iy < buffer.len() / width && ixx < width {
                buffer[iy * width + ixx] = color;
            }
        }
    }
} // end func

fn draw_rect_flipped_x(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    color: u32,
) {
    for iy in y..y + h {
        for ix in x..x + w {
            let mut ixx: i32 = ix as i32 - w as i32;

            if ixx < 0 {
                ixx = width as i32 + ixx;
            }

            if iy < buffer.len() / width && ixx < width as i32 {
                buffer[iy * width + ixx as usize] = color;
            }
        }
    }
} // end func