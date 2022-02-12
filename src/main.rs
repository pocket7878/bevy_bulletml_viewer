mod app_runner;
mod bml_manager;
mod bullet;
mod bullet_type;
mod math_util;

use app_runner::{BulletMLViewerRunner, BulletMLViewerRunnerData};
use bevy::prelude::*;
use bevy_bulletml::{Runner, RunnerData};
use bml_manager::BMLManager;
use bullet::Bullet;
use bullet_type::BulletType;
use once_cell::sync::Lazy;
use std::env;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
static ENEMY_POSITION: Lazy<Vec3> = Lazy::new(|| Vec3::new(0., HEIGHT as f32 / 2. * 0.7, 0.0));
static INITIAL_SHIP_POSITION: Lazy<Vec3> =
    Lazy::new(|| Vec3::new(0., -(HEIGHT as f32 / 2. * 0.7), 0.0));

#[derive(Component)]
struct BulletFrameTimer(Timer);

impl Default for BulletFrameTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0 / 60.0, true)) // 60fps
    }
}

#[derive(Component)]
struct Ship;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bml_file = &args[1];

    let bml_manager = BMLManager::new(bml_file.to_string());
    let top_data = BulletMLViewerRunnerData { turn: 0 };

    App::new()
        .insert_resource(WindowDescriptor {
            title: "BulletML Viewer".to_string(),
            width: WIDTH as f32,
            height: HEIGHT as f32,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(bml_manager)
        .insert_resource(BulletFrameTimer::default())
        .insert_resource(top_data)
        .add_startup_system(setup)
        .add_system(update_bullet_system)
        .add_system(despwan_bullet_system)
        .run();
}

fn setup(bml_manager: Res<BMLManager>, mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    /* Spawn enemy */
    let runner = Runner::new(BulletMLViewerRunner, &bml_manager.bml);
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: *ENEMY_POSITION,
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bullet {
            direction: math_util::get_direction(&ENEMY_POSITION, &INITIAL_SHIP_POSITION) * 180.
                / std::f64::consts::PI,
            speed: 0.,
            vanished: true,
        })
        .insert(BulletType::WithRunner(runner));

    /* Spawn ship */
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: *INITIAL_SHIP_POSITION,
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ship);
}

fn update_bullet_system(
    mut commands: Commands,
    mut runner_data: ResMut<BulletMLViewerRunnerData>,
    bml_manager: Res<BMLManager>,
    time: Res<Time>,
    mut timer: ResMut<BulletFrameTimer>,
    mut bullet_query: Query<(&mut Bullet, &mut Transform, &mut BulletType), Without<Ship>>,
    ship_query: Query<(&Ship, &Transform), Without<Bullet>>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let (_, player_transform) = ship_query.single();
    for (mut bullet, mut transform, mut bullet_type) in &mut bullet_query.iter_mut() {
        match *bullet_type {
            BulletType::Simple => {
                bullet.update(&mut transform);
            }
            BulletType::WithRunner(ref mut runner) => {
                bullet.update(&mut transform);
                runner.run(
                    &mut RunnerData {
                        bml: &bml_manager.bml,
                        data: &mut runner_data,
                    },
                    &mut bullet,
                    &transform,
                    &player_transform,
                    &mut commands,
                );
            }
        }
    }

    runner_data.turn += 1;
}

fn despwan_bullet_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &BulletType), With<Bullet>>,
) {
    for (entity, transform, bullet_type) in query.iter() {
        match *bullet_type {
            BulletType::Simple => {
                if outside_check(&transform.translation) {
                    commands.entity(entity).despawn();
                }
            }
            BulletType::WithRunner(ref runner) => {
                if outside_check(&transform.translation) && runner.is_end() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn outside_check(translation: &Vec3) -> bool {
    let max_x = WIDTH as f32 / 2.0;
    let min_x = -max_x;
    let max_y = HEIGHT as f32 / 2.0;
    let min_y = -max_y;

    translation.x > max_x || translation.x < min_x || translation.y > max_y || translation.y < min_y
}
