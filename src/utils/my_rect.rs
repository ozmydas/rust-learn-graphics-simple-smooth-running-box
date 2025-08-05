#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub velocity: f64,
    pub color: u32,
}

impl Rect {
    pub fn draw_rect(self, buffer: &mut [u32], width: &usize) {
        for iy in self.y..self.y + self.h {
            for ix in self.x..self.x + self.w {
                let mut ixx = ix;

                if ixx > *width {
                    ixx = ixx - width;
                }

                if iy < buffer.len() / width && ixx < *width {
                    buffer[iy * width + ixx] = self.color;
                }
            }
        }
    } // end func

    pub fn move_rect(&mut self, width: &usize, round_size: &usize) {
        if self.x >= (width - round_size) {
            self.x = self.x - 1;
            self.velocity = -self.velocity;
        } else if self.x <= 0 {
            self.x = self.x + 1;
            self.velocity = self.velocity * -1.0;
        } else {
            let adder = self.x as f64 + self.velocity;
            self.x = adder as usize;
        }
    } // end func

    pub fn test_geser(&mut self, new_x: usize) {
        self.x = new_x;
    } // end func
}
