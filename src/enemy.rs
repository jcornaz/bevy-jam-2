use std::time::Duration;

use bevy::{ecs::schedule::ShouldRun, prelude::*};
use rand::{thread_rng, Rng};

use crate::{combine::Harvester, field::Field, Moving};

#[derive(Debug, Clone, Default)]
struct AssetTable {
    bird: Handle<TextureAtlas>,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Enemy;

#[derive(Debug, Deref, DerefMut)]
struct SpawnTimer(Timer);

impl Default for SpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs(1), true))
    }
}

pub struct PlayerHit;

#[derive(Default)]
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .init_resource::<SpawnTimer>()
            .add_event::<PlayerHit>()
            .add_startup_system(Self::load_assets)
            .add_system(Self::spawn.with_run_criteria(Self::should_spawn))
            .add_system(Self::aim)
            .add_system_to_stage(CoreStage::PostUpdate, Self::hit_combine);
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

    fn aim(
        mut enemies: Query<&mut Transform, With<Enemy>>,
        combines: Query<&Transform, (With<Harvester>, Without<Enemy>)>,
    ) {
        let combine_transform = match combines.get_single() {
            Ok(t) => t,
            Err(_) => {
                error!("Combine not found");
                return;
            }
        };

        for mut enemy_transform in enemies.iter_mut() {
            let separation =
                combine_transform.translation.truncate() - enemy_transform.translation.truncate();
            if separation.length_squared() > 0.2 {
                let direction = separation.normalize();
                let angle = Vec2::X.angle_between(direction);
                enemy_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
            }
        }
    }

    fn hit_combine(
        mut events: EventWriter<PlayerHit>,
        combines: Query<&GlobalTransform, With<Harvester>>,
        enemies: Query<&GlobalTransform, With<Enemy>>,
    ) {
        for combine in &combines {
            for enemy in &enemies {
                let distance_squared = (combine.translation().truncate()
                    - enemy.translation().truncate())
                .length_squared();
                if distance_squared < 0.1 {
                    events.send(PlayerHit);
                }
            }
        }
    }

    fn spawn(mut commands: Commands, field: Res<Field>, assets: Res<AssetTable>) {
        let mut rng = thread_rng();
        let pos = match (rng.gen_bool(0.5), rng.gen_bool(0.5)) {
            (true, true) => IVec2::new(-1, rng.gen_range(0..field.height) as i32),
            (true, false) => IVec2::new(field.width as i32, rng.gen_range(0..field.height) as i32),
            (false, true) => IVec2::new(rng.gen_range(0..field.width) as i32, -1),
            (false, false) => IVec2::new(rng.gen_range(0..field.width) as i32, field.height as i32),
        };

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform::from_translation(pos.as_vec2().extend(3.0)),
                texture_atlas: assets.bird.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(Name::from("Enemy"))
            .insert(Moving { speed: 3.0 });
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
