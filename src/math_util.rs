use bevy::prelude::*;

pub fn get_direction(from: &Vec3, to: &Vec3) -> f64 {
    f64::atan2((to.x - from.x) as f64, (from.y - to.y) as f64)
}
