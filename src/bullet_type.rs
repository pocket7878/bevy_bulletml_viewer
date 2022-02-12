use super::app_runner::BulletMLViewerRunner;
use bevy::prelude::*;
use bevy_bulletml::Runner;

#[derive(Component)]
pub enum BulletType {
    Simple,
    WithRunner(Runner<BulletMLViewerRunner>),
}
