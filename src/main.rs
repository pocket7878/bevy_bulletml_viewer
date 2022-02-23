mod app_runner;
mod bullet;
mod bullet_type;
mod math_util;

use app_runner::{BulletMLViewerRunner, BulletMLViewerRunnerData};
use bevy::prelude::*;
use bevy_bulletml::{BulletMLServer, Runner};
use bullet::Bullet;
use bullet_type::BulletType;
use once_cell::sync::Lazy;
use std::env;

const WIDTH: f32 = 640.;
const HEIGHT: f32 = 480.;
static ENEMY_POSITION: Lazy<Vec3> = Lazy::new(|| Vec3::new(0., HEIGHT / 2. * 0.7, 0.0));
static INITIAL_SHIP_POSITION: Lazy<Vec3> = Lazy::new(|| Vec3::new(0., -(HEIGHT / 2. * 0.7), 0.0));

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

    let mut bml_server = BulletMLServer::new();
    bml_server
        .load_file_with_capacities("sample", bml_file, 256, 256)
        .unwrap();
    let top_data = BulletMLViewerRunnerData { turn: 0 };

    App::new()
        .insert_resource(WindowDescriptor {
            title: "BulletML Viewer".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(bml_server)
        .insert_resource(BulletFrameTimer::default())
        .insert_resource(top_data)
        .add_startup_system(setup)
        .add_system(update_bullet_system)
        .add_system(despwan_bullet_system)
        .add_system(update_ship_system)
        .run();
}

fn setup(bml_server: Res<BulletMLServer>, mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    /* Spawn enemy */
    commands.spawn_bundle(SpriteBundle {
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
    });

    // Bullet
    let runner = Runner::new(BulletMLViewerRunner, bml_server.get("sample").unwrap());
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

fn update_ship_system(windows: Res<Windows>, mut query: Query<&mut Transform, With<Ship>>) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let mut ship_transform = query.single_mut();
        ship_transform.translation =
            Vec3::new(position.x - WIDTH / 2., position.y - HEIGHT / 2., 0.);
    }
}

fn update_bullet_system(
    mut commands: Commands,
    mut runner_data: ResMut<BulletMLViewerRunnerData>,
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
                    &mut runner_data,
                    &mut bullet,
                    &transform.translation,
                    &player_transform.translation,
                    &mut commands,
                );
            }
        }
    }

    runner_data.turn += 1;
}

fn despwan_bullet_system(
    mut commands: Commands,
    query: Query<(Entity, &Bullet, &Transform, &BulletType)>,
) {
    for (entity, bullet, transform, bullet_type) in query.iter() {
        match *bullet_type {
            BulletType::Simple => {
                if outside_check(&transform.translation) {
                    commands.entity(entity).despawn();
                }
            }
            BulletType::WithRunner(ref runner) => {
                if (outside_check(&transform.translation) || bullet.vanished) && runner.is_end() {
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
