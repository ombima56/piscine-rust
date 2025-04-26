#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        const GRAVITY: f32 = 9.8;

        // If it's already on or below the ground, return None
        if self.actual_position.y <= 0.0 && self.time != 0.0 {
            return None;
        }

        self.time += 1.0;

        // Calculate new velocity after 1s
        let new_velocity_x = self.init_velocity.x;
        let new_velocity_y = self.init_velocity.y - GRAVITY * self.time;

        // Calculate new position after 1s
        let new_position_x = self.init_position.x + self.init_velocity.x * self.time;
        let new_position_y = self.init_position.y + self.init_velocity.y * self.time - 0.5 * GRAVITY * self.time * self.time;

        self.actual_position.x = new_position_x;
        self.actual_position.y = new_position_y;
        self.actual_velocity.x = new_velocity_x;
        self.actual_velocity.y = new_velocity_y;

        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}
