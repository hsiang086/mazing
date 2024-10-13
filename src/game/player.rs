pub struct Player {
    pub position: (usize, usize),
    pub render_position: (f32, f32),
    pub is_moving: bool,
    pub direction: (isize, isize),
}

impl Player {
    pub fn new(start_x: usize, start_y: usize) -> Self {
        Self {
            position: (start_x, start_y),
            render_position: (start_x as f32, start_y as f32),
            is_moving: false,
            direction: (0, 0),
        }
    }

    pub fn update_position(&mut self, delta_time: f32) {
        if self.is_moving {
            let speed = 5.0; // Cells per second
            let distance = speed * delta_time;

            self.render_position.0 += self.direction.0 as f32 * distance;
            self.render_position.1 += self.direction.1 as f32 * distance;

            let target_x = self.position.0 as f32 + self.direction.0 as f32;
            let target_y = self.position.1 as f32 + self.direction.1 as f32;

            let arrived_x = (self.direction.0 == 0)
                || (self.direction.0 > 0 && self.render_position.0 >= target_x)
                || (self.direction.0 < 0 && self.render_position.0 <= target_x);
            let arrived_y = (self.direction.1 == 0)
                || (self.direction.1 > 0 && self.render_position.1 >= target_y)
                || (self.direction.1 < 0 && self.render_position.1 <= target_y);

            if arrived_x && arrived_y {
                self.position.0 = (self.position.0 as isize + self.direction.0) as usize;
                self.position.1 = (self.position.1 as isize + self.direction.1) as usize;
                self.render_position.0 = self.position.0 as f32;
                self.render_position.1 = self.position.1 as f32;
                self.is_moving = false;
                self.direction = (0, 0);
            }
        }
    }
}
