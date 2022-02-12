use super::bullet::Bullet;
use super::bullet_type::BulletType;
use super::math_util::get_direction;
use bevy::prelude::*;
use bevy_bulletml::{AppRunner, Runner, State};
use rand::prelude::*;

pub struct BulletMLViewerRunner;

pub struct BulletMLViewerRunnerData {
    pub turn: u32,
}

impl AppRunner<BulletMLViewerRunnerData, Bullet> for BulletMLViewerRunner {
    fn get_bullet_direction(&self, _data: &BulletMLViewerRunnerData, bullet: &Bullet) -> f64 {
        bullet.direction
    }

    fn get_aim_direction(
        &self,
        _data: &BulletMLViewerRunnerData,
        bullet_transform: &Transform,
        target_transform: &Transform,
    ) -> f64 {
        get_direction(&bullet_transform.translation, &target_transform.translation) * 180.
            / std::f64::consts::PI
    }

    fn get_bullet_speed(&self, _data: &BulletMLViewerRunnerData, bullet: &Bullet) -> f64 {
        bullet.speed
    }

    fn get_default_speed(&self) -> f64 {
        1.
    }

    fn get_rank(&self, _data: &BulletMLViewerRunnerData) -> f64 {
        0.5
    }

    fn create_simple_bullet(
        &mut self,
        _data: &mut BulletMLViewerRunnerData,
        direction: f64,
        speed: f64,
        bullet_transform: &Transform,
        commands: &mut Commands,
    ) {
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: bullet_transform.translation,
                    scale: Vec3::new(5.0, 5.0, 5.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet {
                direction,
                speed,
                vanished: false,
            })
            .insert(BulletType::Simple);
    }

    fn create_bullet(
        &mut self,
        _data: &mut BulletMLViewerRunnerData,
        state: State,
        direction: f64,
        speed: f64,
        bullet_transform: &Transform,
        commands: &mut Commands,
    ) {
        let runner = Runner::new_from_state(BulletMLViewerRunner, state);
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: bullet_transform.translation,
                    scale: Vec3::new(5.0, 5.0, 5.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Bullet {
                direction,
                speed,
                vanished: false,
            })
            .insert(BulletType::WithRunner(runner));
    }

    fn get_turn(&self, data: &BulletMLViewerRunnerData) -> u32 {
        data.turn
    }

    fn do_vanish(&mut self, _data: &mut BulletMLViewerRunnerData, bullet: &mut Bullet) {
        bullet.vanished = true;
    }

    fn do_change_direction(
        &mut self,
        _data: &mut BulletMLViewerRunnerData,
        direction: f64,
        bullet: &mut Bullet,
    ) {
        bullet.direction = direction;
    }

    fn do_change_speed(
        &mut self,
        _data: &mut BulletMLViewerRunnerData,
        speed: f64,
        bullet: &mut Bullet,
    ) {
        bullet.speed = speed;
    }

    fn get_rand(&self, _data: &mut BulletMLViewerRunnerData) -> f64 {
        rand::thread_rng().gen()
    }
}
