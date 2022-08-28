use std::time::Duration;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::{thread_rng, Rng};

use crate::{combine::Harvester, despawn::despawn, field::Field, movement::Velocity, GameState};

const SPEED: f32 = 3.0;

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
        Self(Timer::new(Duration::from_secs_f32(0.5), true))
    }
}

pub struct PlayerHit;

#[derive(Default)]
pub struct Plugin;

#[derive(Debug, Clone, Copy, SystemLabel)]
struct Spawn;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetTable>()
            .init_resource::<SpawnTimer>()
            .add_event::<PlayerHit>()
            .add_startup_system(Self::load_assets)
            .add_exit_system(GameState::GameOver, despawn::<Enemy>)
            .add_enter_system(GameState::GameOver, Self::stop)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Self::cool_down)
                    .with_system(Self::spawn.run_if(Self::should_spawn))
                    .with_system(Self::aim)
                    .with_system(Self::hit_combine)
                    .into(),
            );
    }
}

impl Plugin {
    fn stop(mut commands: Commands, enemies: Query<Entity, (With<Enemy>, With<Velocity>)>) {
        for enemy in &enemies {
            commands.entity(enemy).remove::<Velocity>();
        }
    }

    fn cool_down(mut timer: ResMut<SpawnTimer>, time: Res<Time>) {
        timer.tick(time.delta());
    }

    fn should_spawn(timer: Res<SpawnTimer>) -> bool {
        timer.just_finished()
    }

    fn aim(
        mut enemies: Query<(&Transform, &mut Velocity), With<Enemy>>,
        combines: Query<&Transform, (With<Harvester>, Without<Enemy>)>,
    ) {
        let combine_transform = match combines.get_single() {
            Ok(t) => t,
            Err(_) => {
                error!("Combine not found");
                return;
            }
        };

        for (enemy_transform, mut enemy_velocity) in enemies.iter_mut() {
            let separation =
                combine_transform.translation.truncate() - enemy_transform.translation.truncate();
            **enemy_velocity = separation.normalize() * SPEED;
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
            .insert(Velocity(Vec2::ZERO));
    }

    fn load_assets(
        mut table: ResMut<AssetTable>,
        server: Res<AssetServer>,
        mut textures: ResMut<Assets<TextureAtlas>>,
    ) {
        table.bird = textures.add(TextureAtlas::from_grid(
            server.load("sprites/enemy.png"),
            Vec2::splat(32.0),
            1,
            1,
        ));
    }
}
