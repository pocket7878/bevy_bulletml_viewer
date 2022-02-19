use bevy::prelude::*;

// 0が真上で、時計まわりにしたい
pub fn get_direction(from: &Vec3, to: &Vec3) -> f64 {
    f64::atan2((to.x - from.x) as f64, (to.y - from.y) as f64)
}
