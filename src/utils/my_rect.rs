#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    pub width: usize,
    pub height: usize,
    pub velocity_x: isize,
    pub velocity_y: isize,
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

    pub fn move_rect(&mut self) {
        // geser x
        if self.velocity_x > 0 || self.velocity_x < 0 {
            if self.x >= (self.width - self.w) {
                self.x = self.x - 1;
                self.velocity_x = -self.velocity_x;
            } else if self.x <= 0 {
                self.x = self.x + 1;
                self.velocity_x = self.velocity_x * -1;
            } else {
                let adder = self.x as isize+ self.velocity_x;
                self.x = adder as usize;
            }
        }

        // geser y
        if self.velocity_y > 0 || self.velocity_y < 0 {
            if self.y >= (self.height - self.h) {
                self.y = self.y - 1;
                self.velocity_y = -self.velocity_y;
            } else if self.y <= 0 {
                self.y = self.y + 1;
                self.velocity_y = self.velocity_y * -1;
            } else {
                let adder = self.y as isize + self.velocity_y;
                self.y = adder as usize;
            }
        }
    } // end func

    pub fn test_geser(&mut self, new_x: usize) {
        self.x = new_x;
    } // end func
}
