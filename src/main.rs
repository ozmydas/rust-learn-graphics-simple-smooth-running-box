use minifb::{Window, WindowOptions};

fn main() {
    println!("Hello, world!");

    let width = 1280;
    let height = 720;
    let mut pin: usize = 0;
    let wh = width * height;
    let qheight = (height / 4);
    let mut x = 0;
    let mut xx = width;
    let round_size = 50;

    let mut buffer: Vec<u32> = vec![0; width * height];
    let mut window = Window::new(
        "Simpe Smooth Running Box",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    // window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
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
        }

        draw_rect(
            &mut buffer,
            width,
            x,
            (qheight * 1) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );
        draw_rect_flipped_x(
            &mut buffer,
            width,
            xx,
            (qheight * 2) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );
        draw_rect(
            &mut buffer,
            width,
            x,
            (qheight * 3) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );
        draw_rect_flipped_x(
            &mut buffer,
            width,
            xx,
            (qheight * 4) - (qheight - ((qheight - round_size) / 2)),
            round_size,
            round_size,
            0x000000,
        );

        if !window.is_key_down(minifb::Key::Space) {
            x = (x + 1) % width;

            if xx > 0 {
                xx -= 1;
            } else {
                xx = width - 1;
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
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
