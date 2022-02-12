use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub direction: f64,
    pub speed: f64,
    pub vanished: bool,
}

impl Bullet {
    pub fn update(&self, transform: &mut Transform) {
        transform.translation.x +=
            (f64::sin(self.direction * std::f64::consts::PI / 180.) * self.speed) as f32;
        transform.translation.y +=
            (f64::cos(self.direction * std::f64::consts::PI / 180.) * self.speed) as f32;
    }

    /*
    fn is_out_of_bounds(&self) -> bool {
            self.pos.x < 0.
                    || self.pos.y < 0.
                    || self.pos.x > WIDTH as f64
                    || self.pos.y >= HEIGHT as f64
    }
    */
}
