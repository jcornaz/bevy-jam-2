use std::time::Duration;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};

use crate::field::Field;

#[derive(Debug, Clone, Default)]
struct AssetTable {
    bird: Handle<TextureAtlas>,
}

#[derive(Debug, Clone, Copy, Component)]
struct Enemy;

#[derive(Debug, Deref, DerefMut)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f64(0.5), true))
    }
}

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .init_resource::<SpawnTimer>()
            .add_startup_system(Self::load_assets)
            .add_system(Self::spawn.with_run_criteria(Self::should_spawn));
    }
}

impl Plugin {
    fn should_spawn(mut timer: ResMut<SpawnTimer>, time: Res<Time>) -> ShouldRun {
        timer.tick(time.delta());
        if timer.just_finished() {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }

    fn spawn(mut commands: Commands, field: Res<Field>) {
        let mut rng = thread_rng();
        let pos = match (rng.gen_bool(0.5), rng.gen_bool(0.5)) {
            (true, true) => IVec2::new(-1, rng.gen_range(0..field.height) as i32),
            (true, false) => IVec2::new(field.width as i32, rng.gen_range(0..field.height) as i32),
            (false, true) => IVec2::new(rng.gen_range(0..field.width) as i32, -1),
            (false, false) => IVec2::new(rng.gen_range(0..field.width) as i32, field.height as i32),
        };

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform::from_translation(pos.as_vec2().extend(2.0)),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Name::from("Enemy"));
    }

    fn load_assets(
        mut table: ResMut<AssetTable>,
        server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
    ) {
        table.bird = textures.add(TextureAtlas::from_grid(
            server.load("enemy.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
    }
}
